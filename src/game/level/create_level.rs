use crate::game::level::{
    CurrentLevel, HALF_TILE_SIZE_F32, LevelData, LevelReadyMessage, LevelsData, TILE_SIZE_F32,
    WallType,
};
use crate::game::outro::GameFinished;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

pub(super) struct CreateLevelPlugin;

impl Plugin for CreateLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            create_level.run_if(resource_changed::<CurrentLevel>),
        );
    }
}

#[derive(Component)]
pub(crate) struct LevelParent;

#[derive(Component)]
pub(crate) struct UnlockingBorder;

#[derive(Component)]
pub(crate) struct NextLevelSensor;

fn create_level(
    mut commands: Commands,
    level_parent_query: Query<Entity, With<LevelParent>>,
    levels_data: Res<LevelsData>,
    current_level: Res<CurrentLevel>,
    level_assets: ResMut<Assets<LevelData>>,
    mut level_ready_message: MessageWriter<LevelReadyMessage>,
    mut game_finished: ResMut<GameFinished>,
) {
    let level_index = if let Some(level) = current_level.0 {
        level
    } else {
        return;
    };

    for old_parent_entity in level_parent_query.iter() {
        commands.entity(old_parent_entity).despawn();
    }

    if level_index >= levels_data.levels.len() {
        game_finished.is_finished = true;
        return;
    }

    commands
        .spawn((Transform::default(), Visibility::Visible, LevelParent))
        .with_children(|parent| {
            let level_data_handle = &levels_data.levels[level_index];

            let level_data = level_assets.get(level_data_handle).unwrap();

            let LevelData {
                player_spawn,
                target_spawn,
                level_size,
                walls,
                story,
                exit,
                absorb_boundary,
            } = level_data;

            #[cfg(debug_assertions)]
            let mut collider_positions =
                HashMap::with_capacity(((level_size.x * 2) + (level_size.y * 2)) as usize);

            #[cfg(debug_assertions)]
            let mut set_collider = |map_coord: IVec2| {
                // assert!(
                //     !collider_positions.contains_key(&map_coord),
                //     "collider at {}",
                //     map_coord
                // );

                collider_positions.insert(map_coord, true);
            };

            for wall in walls {
                let wall_coord = UVec2::new(wall.x, wall.y);

                for x in 0..wall.width {
                    for y in 0..wall.height {
                        let cell_coord = UVec2::new(x, y);

                        let map_coord = wall_coord + cell_coord;

                        parent.spawn(collision_box(
                            map_coord.as_ivec2(),
                            "Inner Wall Segment",
                            wall.wall_type,
                        ));

                        #[cfg(debug_assertions)]
                        set_collider(map_coord.as_ivec2());
                    }
                }
            }

            let border_type = if absorb_boundary.is_some_and(|absorb| absorb) {
                WallType::Absorb
            } else {
                WallType::Solid
            };

            //TODO: optimize. this horrible

            for x in -1..=level_size.x as i32 {
                for y in -1..=level_size.y as i32 {
                    if x == -1
                        || x == (level_size.x as i32)
                        || y == -1
                        || y == (level_size.y as i32)
                    {
                        let border = IVec2::new(x, y);

                        #[cfg(debug_assertions)]
                        set_collider(border);

                        let mut entity_commands =
                            parent.spawn(collision_box(border, "Outer Wall Segment", border_type));

                        if border == *exit {
                            entity_commands.insert(UnlockingBorder);

                            let offset = if border.x.abs() > border.y.abs() {
                                IVec2::new(border.x.signum(), 0)
                            } else {
                                IVec2::new(0, border.y.signum())
                            };

                            parent.spawn(collision_box(
                                border + offset,
                                "Next Level Sensor",
                                (WallType::Exit, NextLevelSensor, Sensor),
                            ));
                        }
                    }
                }
            }

            level_ready_message.write(LevelReadyMessage {
                player: player_spawn.as_usizevec2(),
                target: target_spawn.as_usizevec2(),
                story_index_range: story.clone(),
            });
        });
}

fn collision_box(map_coord: IVec2, name: &str, addl: impl Bundle) -> impl Bundle {
    (
        Transform::from_translation(map_coord.as_vec2().extend(0.0) * TILE_SIZE_F32),
        Collider::cuboid(HALF_TILE_SIZE_F32, HALF_TILE_SIZE_F32),
        RigidBody::Fixed,
        Name::new(name.to_string()),
        addl,
    )
}
