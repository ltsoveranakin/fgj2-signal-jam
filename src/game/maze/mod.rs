pub(super) mod generator;

use crate::game::maze::generator::{
    GenerateMazeMessage, GeneratorPlugin, MazeCellState, MazeReadyMessage,
};
use bevy::math::USizeVec2;
use bevy::prelude::*;
use rand::prelude::{SliceRandom, SmallRng};
use rand::{RngExt, SeedableRng};

const PATH_INDEX: usize = 0;
const WALL_INDEX: usize = 1;

pub(super) struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GeneratorPlugin);

        app.init_resource::<MazeTileImageAssets>();

        app.add_message::<PlaceMazeObjectsMessage>();

        app.add_systems(Startup, (startup_generate, load_assets))
            .add_systems(Update, prepare_spawn_locations);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct MazeTileImageAssets([Handle<Image>; 2]);

#[derive(Component)]
pub(super) struct MazeMatrix {
    matrix: Vec<MazeCellState>,
    pub(super) maze_size: usize,
    pub(crate) rng: SmallRng,
}

fn startup_generate(mut generate_maze_message: MessageWriter<GenerateMazeMessage>) {
    generate_maze_message.write(GenerateMazeMessage { size: 21, seed: 0 });
}

#[derive(Message, Deref)]
pub(super) struct PlaceMazeObjectsMessage(SpawnPoints);

pub(super) struct SpawnPoints {
    pub(super) player: USizeVec2,
    pub(super) target: USizeVec2,
}

fn prepare_spawn_locations(
    mut maze_query: Query<&mut MazeMatrix>,
    mut maze_ready_message: MessageReader<MazeReadyMessage>,
    mut place_maze_objects_message: MessageWriter<PlaceMazeObjectsMessage>,
) {
    for maze_ready in maze_ready_message.read() {
        let mut maze_matrix = maze_query.get_mut(maze_ready.tilemap_entity).unwrap();

        let spawn_points = find_points_in_maze(&mut maze_matrix);

        place_maze_objects_message.write(PlaceMazeObjectsMessage(spawn_points));
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

fn load_assets(
    mut maze_tile_image_assets: ResMut<MazeTileImageAssets>,
    asset_server: Res<AssetServer>,
) {
    let path = asset_server.load("image/tile/path.png");
    let wall = asset_server.load("image/tile/wall.png");

    maze_tile_image_assets[PATH_INDEX] = path;
    maze_tile_image_assets[WALL_INDEX] = wall;
}

impl MazeMatrix {
    fn new(maze_size: usize, seed: u64) -> Self {
        Self {
            matrix: vec![MazeCellState::Wall; maze_size * maze_size],
            maze_size,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    fn us_get_cell_index(&self, usize_vec2: USizeVec2) -> usize {
        Self::us_compute_index_with_size(usize_vec2, self.maze_size)
    }

    fn get_cell_index(&self, x: usize, y: usize) -> usize {
        Self::compute_index_with_size(x, y, self.maze_size)
    }

    pub(super) fn us_get_cell(&self, usize_vec2: USizeVec2) -> MazeCellState {
        self.get_cell(usize_vec2.x, usize_vec2.y)
    }

    fn get_cell(&self, x: usize, y: usize) -> MazeCellState {
        let index = self.get_cell_index(x, y);

        self.matrix[index]
    }

    fn set_cell_at_index(&mut self, index: usize, state: MazeCellState) {
        self.matrix[index] = state;
    }

    fn compute_index_with_size(x: usize, y: usize, maze_size: usize) -> usize {
        x + (y * maze_size)
    }

    fn us_compute_index_with_size(usize_vec2: USizeVec2, maze_size: usize) -> usize {
        Self::compute_index_with_size(usize_vec2.x, usize_vec2.y, maze_size)
    }
}
