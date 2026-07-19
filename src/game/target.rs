use crate::game::level::UnlockLevelMessage;
use crate::game::maze::LevelReadyMessage;
use crate::game::maze::generator::TILE_SIZE_U32;
use crate::game::player::{PLAYER_INTERACT_RANGE, Player};
use crate::game::z_coord::TARGET_Z_COORD;
use bevy::math::FloatPow;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
            ),
        );
    }
}

#[derive(Component)]
pub(super) struct GameTarget {
    is_active: bool,
}

#[derive(Message, Default)]
pub(super) struct HitTargetMessage;

fn spawn_startup_target(mut commands: Commands, asset_server: Res<AssetServer>) {
    let target_image = asset_server.load("image/character/target.png");

    commands.spawn((
        GameTarget { is_active: false },
        Sprite {
            image: target_image,
            color: Srgba::WHITE.with_alpha(0.0).into(),
            ..default()
        },
        Transform::from_xyz(1000.0, 1000.0, TARGET_Z_COORD),
        Collider::capsule_x(6.0, 3.0),
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
    let (mut game_target, mut target_sprite) = target_query.single_mut().unwrap();

    let alpha = target_sprite.color.alpha();
    let alpha_delta_magnitude = time.delta_secs() * FADE_PER_SEC;

    if alpha >= FORCE_SHOW_ALPHA_THRESHOLD {
        game_target.is_active = true;
    }

    let alpha_delta_sign = if game_target.is_active { 1.0 } else { -1.0 };

    target_sprite
        .color
        .set_alpha((alpha + (alpha_delta_magnitude * alpha_delta_sign)).clamp(0.0, 1.0));
}

fn interact_with_target(
    player_query: Query<&Transform, With<Player>>,
    target_query: Query<(&GameTarget, &Transform), With<GameTarget>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut unlock_level_message: MessageWriter<UnlockLevelMessage>,
) {
    let player_transform = player_query.single().unwrap();
    let (target, target_transform) = target_query.single().unwrap();

    if !target.is_active {
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
    }
}
