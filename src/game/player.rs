use crate::game::maze::PlaceMazeObjectsMessage;
use crate::game::maze::generator::TILE_SIZE_U32;
use crate::game::z_coord::PLAYER_Z_COORD;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 75.0;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (place_player_in_maze, move_player));
    }
}

#[derive(Component)]
pub(super) struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Transform {
            translation: Vec3::new(0.0, 0.0, PLAYER_Z_COORD),
            scale: Vec3::splat(0.3),
            ..default()
        },
        Visibility::Hidden,
        Sprite::from_image(asset_server.load("image/character/player.png")),
        Collider::ball(4.0),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        Ccd::enabled(),
    ));
}

fn move_player(
    mut player_query: Query<&mut Velocity, With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    let mut player_velocity = player_query.single_mut().unwrap();
    let mut move_dir = Vec2::ZERO;

    if key_input.pressed(KeyCode::KeyW) || key_input.pressed(KeyCode::ArrowUp) {
        move_dir.y += 1.0;
    }

    if key_input.pressed(KeyCode::KeyS) || key_input.pressed(KeyCode::ArrowDown) {
        move_dir.y -= 1.0;
    }

    if key_input.pressed(KeyCode::KeyD) || key_input.pressed(KeyCode::ArrowRight) {
        move_dir.x += 1.0;
    }

    if key_input.pressed(KeyCode::KeyA) || key_input.pressed(KeyCode::ArrowLeft) {
        move_dir.x -= 1.0;
    }

    if move_dir != Vec2::ZERO {
        move_dir = move_dir.normalize() * PLAYER_SPEED;
    }

    player_velocity.linear = move_dir;
}

fn place_player_in_maze(
    mut player_query: Query<(&mut Transform, &mut Visibility), With<Player>>,
    mut place_maze_objects_message: MessageReader<PlaceMazeObjectsMessage>,
) {
    for place_maze_objects in place_maze_objects_message.read() {
        let (mut player_transform, mut player_visibility) = player_query.single_mut().unwrap();

        player_transform.translation = (place_maze_objects.player * (TILE_SIZE_U32 as usize))
            .as_vec2()
            .extend(player_transform.translation.z);

        *player_visibility = Visibility::Visible;
    }
}
