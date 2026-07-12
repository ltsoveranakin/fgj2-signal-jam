use crate::game::maze::MazeMatrix;
use crate::game::maze::generator::{MazeReadyMessage, TILE_SIZE_U32};
use crate::game::z_coord::PLAYER_Z_COORD;
use bevy::math::USizeVec2;
use bevy::prelude::*;
use rand::RngExt;
use rand::prelude::SmallRng;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, place_player_in_maze);
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
    ));
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

    let spawn_radius = midpoint_coord as f32;

    let player_offset = create_offsets(spawn_radius, &mut maze_matrix.rng);
    let target_offset = create_offsets(spawn_radius, &mut maze_matrix.rng);

    SpawnPoints {
        player: midpoint + player_offset,
        target: midpoint + target_offset,
    }
}

fn create_offsets(spawn_radius: f32, rng: &mut SmallRng) -> USizeVec2 {
    let x_offset = rng.random_range(0.0..spawn_radius);
    let y_offset = rng.random_range(0.0..spawn_radius);

    USizeVec2::new(x_offset as usize, y_offset as usize)
}

struct SpawnPoints {
    player: USizeVec2,
    target: USizeVec2,
}
