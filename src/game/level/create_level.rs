use crate::game::level::{LevelData, LevelsData, SetLevelMessage, WallData};
use crate::game::maze::PlaceMazeObjectsMessage;
use crate::game::maze::generator::{HALF_TILE_SIZE_F32, TILE_SIZE_F32};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;

const PATH_TEXTURE_INDEX: usize = 0;
const WALL_TEXTURE_INDEX: usize = 1;

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
    level_parent_query: Query<Entity, With<LevelParent>>,
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

        let _level_parent_entity = level_parent_query.single().unwrap();

        for WallData { dim, coord } in walls {
            for x in 0..dim.x {
                for y in 0..dim.y {
                    let cell_coord = UVec2::new(x, y);

                    let map_coord = coord + cell_coord;

                    println!("map coord {}", map_coord);

                    commands.spawn(collision_box(map_coord.as_ivec2()));
                }
            }
        }

        for x in -1..=(level_size.x as i32) {
            commands.spawn(collision_box(IVec2::new(x, level_size.y as i32)));
            commands.spawn(collision_box(IVec2::new(x, -1)));
        }

        for y in 0..(level_size.y as i32) {
            commands.spawn(collision_box(IVec2::new(level_size.x as i32, y)));
            commands.spawn(collision_box(IVec2::new(-1, y)));
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
