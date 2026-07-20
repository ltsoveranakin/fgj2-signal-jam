use crate::game::glow::Glow;
use crate::game::level::{LevelReadyMessage, TILE_SIZE_U32, UnlockLevelMessage};
use crate::game::lidar::dots::{FadeDot, LIDAR_DOT_SPEED, SpawnDotsMessage};
use crate::game::player::{PLAYER_INTERACT_RANGE, Player};
use crate::game::z_coord::TARGET_Z_COORD;
use bevy::color::palettes::css;
use bevy::math::FloatPow;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

const FADE_PER_SEC: f32 = 0.15;
const TARGET_HIT_INC_ALPHA: f32 = 0.05;
const FORCE_SHOW_ALPHA_THRESHOLD: f32 = 0.5;

pub(super) struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HitTargetMessage>();

        app.add_systems(Startup, spawn_startup_target).add_systems(
            Update,
            (
                place_target_in_maze,
                hit_target,
                fade_target,
                interact_with_target,
                update_target_state,
            ),
        );
    }
}

#[derive(Component)]
pub(super) struct GameTarget {
    pub(super) is_active: bool,
    pub(super) did_interact: bool,
}

#[derive(Message, Default)]
pub(super) struct HitTargetMessage;

fn spawn_startup_target(mut commands: Commands, asset_server: Res<AssetServer>) {
    let target_image = asset_server.load("image/character/target.png");

    commands.spawn((
        GameTarget {
            is_active: false,
            did_interact: false,
        },
        Sprite {
            image: target_image,
            color: Srgba::WHITE.with_alpha(0.0).into(),
            ..default()
        },
        Glow {
            is_glowing: false,
            glow_color: LinearRgba::new(2.0, 4.0, 2.0, 1.0),
        },
        Transform::from_xyz(1000.0, 1000.0, TARGET_Z_COORD),
        Collider::capsule_x(2.5, 2.5),
        Sensor,
    ));
}

fn place_target_in_maze(
    mut target_query: Query<(&mut GameTarget, &mut Transform, &mut Sprite)>,
    mut level_ready_message: MessageReader<LevelReadyMessage>,
) {
    for place_maze_objects in level_ready_message.read() {
        let (mut target, mut target_transform, mut target_sprite) =
            target_query.single_mut().unwrap();

        target_transform.translation = (place_maze_objects.target * (TILE_SIZE_U32 as usize))
            .as_vec2()
            .extend(target_transform.translation.z);

        target.is_active = false;
        target_sprite.color.set_alpha(0.0);
    }
}

fn hit_target(
    mut target_query: Query<&mut Sprite, With<GameTarget>>,
    mut hit_target_message: MessageReader<HitTargetMessage>,
) {
    for _ in hit_target_message.read() {
        let mut target_sprite = target_query.single_mut().unwrap();

        let alpha = target_sprite.color.alpha();

        target_sprite
            .color
            .set_alpha((alpha + TARGET_HIT_INC_ALPHA).clamp(0.0, 1.0));
    }
}

fn fade_target(mut target_query: Query<(&mut GameTarget, &mut Sprite)>, time: Res<Time>) {
    let (mut target, mut target_sprite) = target_query.single_mut().unwrap();

    let alpha = target_sprite.color.alpha();
    let alpha_delta_magnitude = time.delta_secs() * FADE_PER_SEC;

    if alpha >= FORCE_SHOW_ALPHA_THRESHOLD {
        target.is_active = true;
    }

    let alpha_delta = if target.is_active {
        alpha_delta_magnitude
    } else {
        -alpha_delta_magnitude
    };

    target_sprite
        .color
        .set_alpha((alpha + alpha_delta).clamp(0.0, 1.0));
}

fn interact_with_target(
    player_query: Query<&Transform, With<Player>>,
    mut target_query: Query<(&mut GameTarget, &Transform), With<GameTarget>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut unlock_level_message: MessageWriter<UnlockLevelMessage>,
    mut spawn_dots_message: MessageWriter<SpawnDotsMessage>,
) {
    let player_transform = player_query.single().unwrap();
    let (mut target, target_transform) = target_query.single_mut().unwrap();

    if !target.is_active || target.did_interact {
        return;
    }

    let is_in_range = player_transform
        .translation
        .truncate()
        .distance_squared(target_transform.translation.truncate())
        <= PLAYER_INTERACT_RANGE.squared();

    if is_in_range
        && (key_input.just_pressed(KeyCode::KeyE) || mouse_input.just_pressed(MouseButton::Left))
    {
        unlock_level_message.write_default();
        target.did_interact = true;

        spawn_dots_message.write(SpawnDotsMessage {
            location: target_transform.translation.truncate(),
            fov: PI * 2.0,
            direction: 0.0,
            dot_count: 100,
            speed: LIDAR_DOT_SPEED * 0.2,
            fade: FadeDot::new_alpha(30.0, css::YELLOW),
        });
    }
}

fn update_target_state(mut target_query: Query<(&mut GameTarget, &mut Glow)>) {
    let (mut target, mut glow) = target_query.single_mut().unwrap();

    if !target.is_active {
        target.did_interact = false;
    }

    glow.is_glowing = target.is_active;
}
