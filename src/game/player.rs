use crate::control::inputs_allowed;
use crate::game::level::create_level::NextLevelSensor;
use crate::game::level::{CurrentLevel, LevelReadyMessage, TILE_SIZE_F32, TILE_SIZE_U32};

use crate::assign_vec::AssignVec;
use crate::game::z_coord::PLAYER_Z_COORD;
use crate::sprite_sheet::player_rect_from_sheet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 75.0;
pub(super) const PLAYER_INTERACT_RANGE: f32 = TILE_SIZE_F32 * 1.5;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                place_player_in_maze,
                (
                    move_player.run_if(inputs_allowed),
                    reset_player_velocity.run_if(not(inputs_allowed)),
                )
                    .chain(),
                player_touch_level_progress,
            ),
        );
    }
}

#[derive(Component)]
pub(super) struct Player;

#[derive(Component)]
pub(super) enum PlayerFacing {
    Up,
    Down,
    Left,
    Right,
}

impl PlayerFacing {
    pub(super) fn direction(&self) -> Vec2 {
        match self {
            Self::Up => Vec2::new(0.0, 1.0),

            Self::Down => Vec2::new(0.0, -1.0),

            Self::Left => Vec2::new(-1.0, 0.0),

            Self::Right => Vec2::new(1.0, 0.0),
        }
    }

    /// Returns the sprite index and if the sprite should be flipped horizontally
    fn get_sprite(&self) -> (bool, usize) {
        match self {
            Self::Up => (false, 1),
            Self::Down => (false, 2),
            Self::Left => (false, 0),
            Self::Right => (true, 0),
        }
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        PlayerFacing::Right,
        Transform {
            translation: Vec3::new(0.0, 0.0, PLAYER_Z_COORD),
            scale: Vec3::splat(0.2),
            ..default()
        },
        Sprite {
            image: asset_server.load("image/character/player.png"),
            rect: Some(player_rect_from_sheet(0)),
            flip_x: true,
            ..default()
        },
        Visibility::Hidden,
        Collider::capsule_y(10.0, 6.0),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        GravityScale(0.0),
        ActiveEvents::COLLISION_EVENTS,
        Name::new("Player"),
    ));
}

fn move_player(
    mut player_query: Query<(&mut Velocity, &mut PlayerFacing, &mut Sprite)>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut player_velocity, mut player_facing, mut player_sprite) =
        player_query.single_mut().unwrap();
    let mut move_dir = Vec2::ZERO;

    if key_input.pressed(KeyCode::KeyW) || key_input.pressed(KeyCode::ArrowUp) {
        move_dir.y += 1.0;
        *player_facing = PlayerFacing::Up;
    }

    if key_input.pressed(KeyCode::KeyS) || key_input.pressed(KeyCode::ArrowDown) {
        move_dir.y -= 1.0;
        *player_facing = PlayerFacing::Down;
    }

    if key_input.pressed(KeyCode::KeyD) || key_input.pressed(KeyCode::ArrowRight) {
        move_dir.x += 1.0;
        *player_facing = PlayerFacing::Right;
    }

    if key_input.pressed(KeyCode::KeyA) || key_input.pressed(KeyCode::ArrowLeft) {
        move_dir.x -= 1.0;
        *player_facing = PlayerFacing::Left;
    }

    let (flip_x, sprite_index) = player_facing.get_sprite();

    player_sprite.rect = Some(player_rect_from_sheet(sprite_index));
    player_sprite.flip_x = flip_x;

    if move_dir != Vec2::ZERO {
        move_dir = move_dir.normalize() * PLAYER_SPEED;
    }

    player_velocity.linear = move_dir;
}

fn reset_player_velocity(mut player_query: Query<&mut Velocity, With<Player>>) {
    let mut player_velocity = player_query.single_mut().unwrap();
    player_velocity.linear = Vec2::ZERO;
}

fn place_player_in_maze(
    mut player_query: Query<(&mut Transform, &mut Visibility, &mut PlayerFacing), With<Player>>,
    mut level_ready_message: MessageReader<LevelReadyMessage>,
) {
    for place_maze_objects in level_ready_message.read() {
        let (mut player_transform, mut player_visibility, mut facing) =
            player_query.single_mut().unwrap();

        player_transform.translation = (place_maze_objects.player * (TILE_SIZE_U32 as usize))
            .as_vec2()
            .extend(player_transform.translation.z);

        *player_visibility = Visibility::Visible;
        *facing = PlayerFacing::Right;
    }
}

fn player_touch_level_progress(
    player_marker_query: Query<(), With<Player>>,
    level_end_marker_query: Query<(), With<NextLevelSensor>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut current_level: ResMut<CurrentLevel>,
    mut collision_event: MessageReader<CollisionEvent>,
) {
    for collision in collision_event.read() {
        match collision {
            CollisionEvent::Started(entity_1, entity_2, _flags) => {
                if let Some((_, _)) = match_colliders(
                    *entity_1,
                    *entity_2,
                    &player_marker_query,
                    &level_end_marker_query,
                ) {
                    player_transform_query
                        .single_mut()
                        .unwrap()
                        .translation
                        .assign_from(Vec2::splat(1000.0));
                    current_level.next();
                }
            }

            CollisionEvent::Stopped(..) => {}
        }
    }
}

fn match_colliders<'w, 's>(
    entity1: Entity,
    entity2: Entity,
    query1: &'w Query<'w, 's, (), With<impl Component>>,
    query2: &'w Query<'w, 's, (), With<impl Component>>,
) -> Option<(Entity, Entity)> {
    if query1.get(entity1).is_ok() && query2.get(entity2).is_ok() {
        Some((entity1, entity2))
    } else if query1.get(entity2).is_ok() && query2.get(entity1).is_ok() {
        Some((entity1, entity2))
    } else {
        None
    }
}
