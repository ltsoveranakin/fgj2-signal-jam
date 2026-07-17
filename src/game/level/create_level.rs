use crate::game::level::{LevelData, LevelsData, SetLevelMessage};
use crate::game::maze::PlaceMazeObjectsMessage;
use crate::game::maze::generator::{HALF_TILE_SIZE_F32, TILE_SIZE_F32};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

pub(super) struct CreateLevelPlugin;

impl Plugin for CreateLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level_parent)
            .add_systems(Update, create_level_on_set);
    }
}

#[derive(Component)]
struct LevelParent;

fn spawn_level_parent(mut commands: Commands) {
    commands.spawn((LevelParent, Transform::default(), Name::new("Level")));
}

fn create_level_on_set(
    mut commands: Commands,
    levels_data: Res<LevelsData>,
    mut set_level_message: MessageReader<SetLevelMessage>,
    mut place_maze_objects_message: MessageWriter<PlaceMazeObjectsMessage>,
) {
    for set_level in set_level_message.read() {
        let LevelData {
            player_spawn,
            target_spawn,
            level_size,
            walls,
        } = &levels_data.levels[set_level.0];

        #[cfg(debug_assertions)]
        let mut collider_positions =
            HashMap::with_capacity(((level_size.x * 2) + (level_size.y * 2)) as usize);

        #[cfg(debug_assertions)]
        let mut set_collider = |map_coord: IVec2| {
            assert!(!collider_positions.contains_key(&map_coord));

            collider_positions.insert(map_coord, true);
        };

        for wall in walls {
            let wall_coord = UVec2::new(wall.x, wall.y);

            for x in 0..wall.width {
                for y in 0..wall.height {
                    let cell_coord = UVec2::new(x, y);

                    let map_coord = wall_coord + cell_coord;

                    commands.spawn(collision_box(map_coord.as_ivec2()));

                    #[cfg(debug_assertions)]
                    set_collider(map_coord.as_ivec2());
                }
            }
        }

        for x in -1..=(level_size.x as i32) {
            let top_border = IVec2::new(x, level_size.y as i32);
            let bottom_border = IVec2::new(x, -1);

            #[cfg(debug_assertions)]
            {
                set_collider(top_border);
                set_collider(bottom_border);
            }

            commands.spawn(collision_box(top_border));
            commands.spawn(collision_box(bottom_border));
        }

        for y in 0..(level_size.y as i32) {
            let left_border = IVec2::new(-1, y);
            let right_border = IVec2::new(level_size.x as i32, y);

            #[cfg(debug_assertions)]
            {
                set_collider(left_border);
                set_collider(right_border);
            }

            commands.spawn(collision_box(left_border));
            commands.spawn(collision_box(right_border));
        }

        place_maze_objects_message.write(PlaceMazeObjectsMessage {
            player: player_spawn.as_usizevec2(),
            target: target_spawn.as_usizevec2(),
        });
    }
}

fn collision_box(map_coord: IVec2) -> impl Bundle {
    (
        Transform::from_translation(map_coord.as_vec2().extend(0.0) * TILE_SIZE_F32),
        Collider::cuboid(HALF_TILE_SIZE_F32, HALF_TILE_SIZE_F32),
        RigidBody::Fixed,
        Name::new("Wall Segment"),
    )
}
