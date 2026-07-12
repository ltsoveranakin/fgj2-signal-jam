use crate::game::maze::MazeMatrix;
use crate::game::maze::generator::{MazeReadyMessage, TILE_SIZE_U32};
use crate::game::z_coord::PLAYER_Z_COORD;
use bevy::math::USizeVec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::RngExt;
use rand::prelude::*;

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
        Transform::from_xyz(0.0, 0.0, PLAYER_Z_COORD),
        Visibility::Hidden,
        Sprite::from_image(asset_server.load("image/character/player.png")),
        Collider::ball(6.0),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        Ccd::enabled(),
    ));
}

fn move_player(
    mut player_query: Query<&mut Velocity, With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
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
    mut maze_query: Query<&mut MazeMatrix>,
    mut maze_ready_message: MessageReader<MazeReadyMessage>,
) {
    for maze_ready in maze_ready_message.read() {
        let (mut player_transform, mut player_visibility) = player_query.single_mut().unwrap();
        let mut maze_matrix = maze_query.get_mut(maze_ready.tilemap_entity).unwrap();

        let spawn_points = find_points_in_maze(&mut maze_matrix);

        player_transform.translation = (spawn_points.player * (TILE_SIZE_U32 as usize))
            .as_vec2()
            .extend(player_transform.translation.z);

        *player_visibility = Visibility::Visible;
    }
}

fn find_points_in_maze(maze_matrix: &mut MazeMatrix) -> SpawnPoints {
    let midpoint_coord = maze_matrix.maze_size.div_ceil(2);
    let midpoint = USizeVec2::splat(midpoint_coord);

    let spawn_radius = (midpoint_coord as f32) * 0.8;

    let player_loc = midpoint + create_offsets(spawn_radius, &mut maze_matrix.rng);
    let target_loc = midpoint + create_offsets(spawn_radius, &mut maze_matrix.rng);

    SpawnPoints {
        player: find_valid_spawn_spot(maze_matrix, player_loc),
        target: find_valid_spawn_spot(maze_matrix, target_loc),
    }
}

fn create_offsets(spawn_radius: f32, rng: &mut SmallRng) -> USizeVec2 {
    let x_offset = rng.random_range(0.0..spawn_radius);
    let y_offset = rng.random_range(0.0..spawn_radius);

    USizeVec2::new(x_offset as usize, y_offset as usize)
}

fn find_valid_spawn_spot(maze_matrix: &mut MazeMatrix, coord: USizeVec2) -> USizeVec2 {
    const OFFSETS: [IVec2; 10] = calc_offsets();

    let mut offsets = OFFSETS;

    offsets.shuffle(&mut maze_matrix.rng);

    for offset in offsets {
        let cell_pos = (coord.as_ivec2() + offset).as_usizevec2();

        if maze_matrix.us_get_cell(cell_pos).is_path() {
            return cell_pos;
        }
    }

    unreachable!()
}

const fn calc_offsets() -> [IVec2; 10] {
    let mut i = 0;

    let mut offsets = [IVec2::ZERO; 10];

    let mut x = -1;

    while x <= 1 {
        let mut y = -1;

        while y <= 1 {
            offsets[i] = IVec2::new(x, y);

            y += 1;
            i += 1;
        }
        x += 1;
    }

    offsets
}

struct SpawnPoints {
    player: USizeVec2,
    target: USizeVec2,
}
