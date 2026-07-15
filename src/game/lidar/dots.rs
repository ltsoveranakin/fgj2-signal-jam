use crate::assign_vec::AssignVec;

use crate::game::player::Player;
use crate::game::target::GameTarget;
use crate::game::z_coord::LIDAR_DOT_Z_COORD;
use bevy::prelude::*;
use bevy_rapier2d::pipeline::QueryFilter;
use bevy_rapier2d::prelude::*;

const LIDAR_DOT_SPEED: f32 = 100.0;
const LIDAR_DOTS_COUNT: usize = 100;

pub(super) struct LidarDotsPlugin;

impl Plugin for LidarDotsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LidarState>();

        app.add_systems(Startup, spawn_initial_dots).add_systems(
            Update,
            (
                position_dots_ready.run_if(in_state(LidarState::Waiting)),
                move_dots.run_if(in_state(LidarState::Scanning)),
            ),
        );
    }
}

#[derive(Component)]
struct LidarDot {
    direction: Vec2,
    last_entity_hit: Entity,
}

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
enum LidarState {
    #[default]
    Waiting,
    Scanning,
    FinishedScan,
}

fn spawn_initial_dots(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut lidar_dots_parent = commands.spawn((
        Transform::default(),
        Visibility::default(),
        Name::new("Lidar Dots"),
    ));

    for _ in 0..LIDAR_DOTS_COUNT {
        lidar_dots_parent.with_child((
            LidarDot {
                direction: Vec2::ZERO,
                last_entity_hit: Entity::PLACEHOLDER,
            },
            Sprite::from_image(asset_server.load("image/particle/lidar_dot.png")),
            Transform {
                translation: Vec3 {
                    z: LIDAR_DOT_Z_COORD,
                    ..default()
                },
                scale: Vec3::splat(0.5),
                ..default()
            },
            Visibility::Hidden,
        ));
    }
}

fn position_dots_ready(
    mut lidar_dot_query: Query<(&mut LidarDot, &mut Transform, &mut Visibility)>,
    player_query: Query<&Transform, (With<Player>, Without<LidarDot>)>,
    mut next_state: ResMut<NextState<LidarState>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    if !key_input.just_pressed(KeyCode::Space) {
        return;
    }

    let mut player_translation = player_query.single().unwrap().translation;
    player_translation.z = LIDAR_DOT_Z_COORD;

    let step = (std::f32::consts::PI * 2.0) / (LIDAR_DOTS_COUNT as f32);

    for (i, (mut lidar_dot, mut lidar_transform, mut lidar_visibility)) in
        lidar_dot_query.iter_mut().enumerate()
    {
        let angle = i as f32 * step;
        let direction = Vec2::from_angle(angle);

        lidar_dot.direction = direction;

        lidar_transform.translation = player_translation;
        *lidar_visibility = Visibility::Visible;
    }

    next_state.set(LidarState::Scanning);
}

fn move_dots(
    mut dot_query: Query<(&mut LidarDot, &mut Transform)>,
    player_query: Query<Entity, With<Player>>,
    target_query: Query<Entity, With<GameTarget>>,
    mut next_state: ResMut<NextState<LidarState>>,
    time: Res<Time>,
    rapier_context: ReadRapierContext,
) {
    let player_entity = player_query.single().unwrap();
    let target_entity = target_query.single().unwrap();
    let rapier_context = rapier_context.single().unwrap();

    let distance_to_travel_this_tick = LIDAR_DOT_SPEED * time.delta_secs();

    for (mut lidar_dot, mut dot_transform) in dot_query.iter_mut() {
        let mut distance_remaining_to_travel_this_tick = distance_to_travel_this_tick;

        loop {
            if distance_remaining_to_travel_this_tick <= 0.0 {
                break;
            }

            let current_dot_position = dot_transform.translation.truncate();

            let solid = true;
            let predicate = |entity| {
                if lidar_dot.last_entity_hit == entity {
                    false
                } else {
                    true
                }
            };
            let query = QueryFilter::new()
                .exclude_collider(player_entity)
                .predicate(&predicate);

            let (entity_hit, hit_result) = if let Some(result) = rapier_context
                .cast_ray_and_get_normal(
                    current_dot_position,
                    lidar_dot.direction,
                    distance_remaining_to_travel_this_tick as bevy_rapier2d::prelude::Real,
                    solid,
                    query,
                ) {
                result
            } else {
                dot_transform.translation = (current_dot_position
                    + (lidar_dot.direction * distance_remaining_to_travel_this_tick))
                    .extend(dot_transform.translation.z);
                break;
            };

            if entity_hit == target_entity {
                next_state.set(LidarState::FinishedScan);
            }

            distance_remaining_to_travel_this_tick -= hit_result.time_of_impact;

            dot_transform.translation.assign_from(hit_result.point);

            let reflected_dir = lidar_dot.direction
                - (2.0 * (lidar_dot.direction * hit_result.normal)) * hit_result.normal;

            lidar_dot.last_entity_hit = entity_hit;
            lidar_dot.direction = reflected_dir;
        }
    }
}
