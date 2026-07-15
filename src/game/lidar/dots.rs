use crate::game::lidar::{CastState, LIDAR_DOTS_COUNT, LidarCasts, StartLidarMessage};
use crate::game::player::Player;
use crate::game::z_coord::LIDAR_DOT_Z_COORD;
use bevy::prelude::*;

const LIDAR_DOT_SPEED: f32 = 250.0;

pub(super) struct LidarDotsPlugin;

impl Plugin for LidarDotsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_dots)
            .add_systems(Update, (position_dots_ready, move_dots));
    }
}

#[derive(Component)]
struct LidarDot {
    cast_index: usize,
    goal_position_index: Option<usize>,
}

fn spawn_initial_dots(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..LIDAR_DOTS_COUNT {
        commands.spawn((
            LidarDot {
                cast_index: i,
                goal_position_index: None,
            },
            Sprite::from_image(asset_server.load("image/particle/lidar_dot.png")),
            Transform::from_translation(Vec3 {
                z: LIDAR_DOT_Z_COORD,
                ..default()
            }),
            Visibility::Hidden,
        ));
    }
}

fn position_dots_ready(
    mut lidar_dot_query: Query<(&mut LidarDot, &mut Transform, &mut Visibility)>,
    player_query: Query<&Transform, (With<Player>, Without<LidarDot>)>,
    mut start_lidar_message: MessageReader<StartLidarMessage>,
) {
    for _ in start_lidar_message.read() {
        let player_transform = player_query.single().unwrap();

        for (mut lidar_dot, mut lidar_transform, mut lidar_visibility) in lidar_dot_query.iter_mut()
        {
            lidar_dot.goal_position_index = Some(0);
            lidar_transform.translation = Vec3 {
                z: lidar_transform.translation.z,
                ..player_transform.translation
            };
            *lidar_visibility = Visibility::Visible;
        }
    }
}

fn move_dots(
    mut dot_query: Query<(&mut LidarDot, &mut Transform)>,
    lidar_casts: Res<LidarCasts>,
    time: Res<Time>,
) {
    if lidar_casts.cast_state == CastState::Inactive || !lidar_casts.is_ready() {
        return;
    }

    for (mut lidar_dot, mut transform) in dot_query.iter_mut() {
        let cast_index = lidar_dot.cast_index;
        let goal_position_index = if let Some(goal_pos_index) = &mut lidar_dot.goal_position_index {
            goal_pos_index
        } else {
            continue;
        };

        let lidar_cast = lidar_casts.get_possible_value(cast_index);

        let mut distance_remaining_to_travel_this_tick = LIDAR_DOT_SPEED * time.delta_secs();

        loop {
            if distance_remaining_to_travel_this_tick <= 0.0 {
                break;
            }

            let goal = if let Some(goal) = lidar_cast.cast_positions.get(*goal_position_index) {
                goal
            } else {
                // reached end
                break;
            };

            let current_dot_position = transform.translation.truncate();
            let goal_offset = goal.position - current_dot_position;
            if goal_offset == Vec2::ZERO {
                *goal_position_index += 1;
                continue;
            }
            let goal_direction = goal_offset.normalize();
            let goal_distance = goal_offset.length();

            let mut is_done_this_tick = false;

            let travel_amt = if goal_distance < distance_remaining_to_travel_this_tick {
                *goal_position_index += 1;
                is_done_this_tick = true;
                goal_distance
            } else {
                distance_remaining_to_travel_this_tick
            };

            // let travel_amt = distance_remaining_to_travel_this_tick.min(goal_distance);

            let new_dot_position = current_dot_position + (goal_direction * travel_amt);

            transform.translation = new_dot_position.extend(transform.translation.z);
            distance_remaining_to_travel_this_tick -= travel_amt;

            if is_done_this_tick {
                break;
            }
        }
    }
}
