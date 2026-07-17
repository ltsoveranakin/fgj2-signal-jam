use crate::assign_vec::AssignVec;
use bevy::color::palettes::css;

use crate::game::player::Player;
use crate::game::target::GameTarget;
use crate::game::z_coord::LIDAR_DOT_Z_COORD;
use bevy::prelude::*;
use bevy_rapier2d::pipeline::QueryFilter;
use bevy_rapier2d::prelude::*;

const LIDAR_DOT_SPEED: f32 = 150.0;
const LIDAR_DOTS_COUNT: usize = 200;
const WALL_DOT_ALIVE_TIME: f32 = 10.0;
const LIDAR_DOT_ALIVE_TIME: f32 = 10.0;
const BOUNCE_ALIVE_TIME_PENALTY: f32 = 0.5;

pub(super) struct LidarDotsPlugin;

impl Plugin for LidarDotsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LidarState>();

        app.add_systems(
            Update,
            (
                position_dots_ready.run_if(in_state(LidarState::Waiting)),
                move_dots.run_if(in_state(LidarState::Scanning)),
                fade_wall_dots,
            ),
        );
    }
}

#[derive(Component)]
struct LidarDot {
    direction: Vec2,
    last_entity_hit: Entity,
}

#[derive(Component)]
struct WallDot;

#[derive(Component)]
struct FadeDot {
    total_fade_time: f32,
    fade_time_elapsed: f32,
    fade_from: Color,
    fade_to: Color,
}

impl FadeDot {
    fn new_alpha(total_fade_time: f32, fade_from: impl Into<Color>) -> Self {
        let fade_from = fade_from.into();

        Self::new(total_fade_time, fade_from, fade_from.with_alpha(0.0))
    }

    fn new(total_fade_time: f32, fade_from: impl Into<Color>, fade_to: impl Into<Color>) -> Self {
        Self {
            total_fade_time,
            fade_time_elapsed: 0.0,
            fade_from: fade_from.into(),
            fade_to: fade_to.into(),
        }
    }
}

enum FadeColor {
    WhiteToBlack,
}

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
enum LidarState {
    #[default]
    Waiting,
    Scanning,
    FinishedScan,
}

fn position_dots_ready(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<LidarDot>)>,
    mut next_state: ResMut<NextState<LidarState>>,
    key_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    if !key_input.just_pressed(KeyCode::Space) {
        return;
    }

    let lidar_dot_sprite = asset_server.load("image/particle/lidar_dot.png");

    let mut player_translation_lidar_dot_z = player_query.single().unwrap().translation;
    player_translation_lidar_dot_z.z = LIDAR_DOT_Z_COORD;

    let step = (std::f32::consts::PI * 2.0) / (LIDAR_DOTS_COUNT as f32);

    let batch = (0..LIDAR_DOTS_COUNT).into_iter().map(move |i| {
        let angle = i as f32 * step;
        let direction = Vec2::from_angle(angle);

        (
            LidarDot {
                direction,
                last_entity_hit: Entity::PLACEHOLDER,
            },
            Sprite::from_image(lidar_dot_sprite.clone()),
            Transform {
                translation: player_translation_lidar_dot_z,
                scale: Vec3::splat(0.5),
                ..default()
            },
            FadeDot::new(LIDAR_DOT_ALIVE_TIME, css::GREEN, css::RED.with_alpha(0.0)),
        )
    });

    commands.spawn_batch(batch);

    next_state.set(LidarState::Scanning);
}

fn move_dots(
    mut commands: Commands,
    mut dot_query: Query<(&mut LidarDot, &mut FadeDot, &mut Transform)>,
    player_query: Query<Entity, With<Player>>,
    target_query: Query<Entity, With<GameTarget>>,
    mut next_state: ResMut<NextState<LidarState>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    rapier_context: ReadRapierContext,
) {
    let player_entity = player_query.single().unwrap();
    let target_entity = target_query.single().unwrap();
    let rapier_context = rapier_context.single().unwrap();

    let distance_to_travel_this_tick = LIDAR_DOT_SPEED * time.delta_secs();

    for (mut lidar_dot, mut fade_dot, mut dot_transform) in dot_query.iter_mut() {
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

            dot_transform.translation.assign_from(hit_result.point);

            commands.spawn((
                FadeDot::new_alpha(WALL_DOT_ALIVE_TIME, Color::WHITE),
                Sprite::from_image(asset_server.load("image/particle/lidar_dot_wall_fade.png")),
                *dot_transform,
            ));

            if entity_hit == target_entity {
                next_state.set(LidarState::FinishedScan);
            }

            distance_remaining_to_travel_this_tick -= hit_result.time_of_impact;

            let reflected_dir = lidar_dot.direction
                - (2.0 * (lidar_dot.direction * hit_result.normal)) * hit_result.normal;

            lidar_dot.last_entity_hit = entity_hit;
            lidar_dot.direction = reflected_dir;

            fade_dot.fade_time_elapsed += BOUNCE_ALIVE_TIME_PENALTY;
        }
    }
}

fn fade_wall_dots(
    mut commands: Commands,
    mut fade_dot_query: Query<(Entity, &mut FadeDot, &mut Sprite), With<FadeDot>>,
    time: Res<Time>,
) {
    for (entity, mut fade_dot, mut sprite) in fade_dot_query.iter_mut() {
        if fade_dot.fade_time_elapsed >= fade_dot.total_fade_time {
            commands.entity(entity).despawn();
            continue;
        }

        let percent = fade_dot.fade_time_elapsed / fade_dot.total_fade_time;

        let current_color = fade_dot.fade_from.mix(&fade_dot.fade_to, percent);

        sprite.color = current_color;

        fade_dot.fade_time_elapsed += time.delta_secs();
    }
}
