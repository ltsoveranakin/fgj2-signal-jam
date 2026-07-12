pub(super) mod generator;

use crate::game::maze::generator::{GenerateMazeMessage, GeneratorPlugin, MazeCellState};
use bevy::math::USizeVec2;
use bevy::prelude::*;
use std::slice::SliceIndex;

const PATH_INDEX: usize = 0;
const WALL_INDEX: usize = 1;

pub(super) struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GeneratorPlugin)
            .init_resource::<MazeTileImageAssets>()
            .add_systems(Startup, (startup_generate, load_assets));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct MazeTileImageAssets([Handle<Image>; 2]);

#[derive(Component)]
pub(super) struct MazeMatrix {
    matrix: Vec<MazeCellState>,
    maze_size: usize,
}

impl MazeMatrix {
    fn new(maze_size: usize) -> Self {
        Self {
            matrix: vec![MazeCellState::Wall; maze_size * maze_size],
            maze_size,
        }
    }

    fn us_get_cell_index(&self, usize_vec2: USizeVec2) -> usize {
        Self::us_compute_index_with_size(usize_vec2, self.maze_size)
    }

    fn get_cell_index(&self, x: usize, y: usize) -> usize {
        Self::compute_index_with_size(x, y, self.maze_size)
    }

    fn us_get_cell(&self, usize_vec2: USizeVec2) -> MazeCellState {
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

fn startup_generate(mut generate_maze_message: MessageWriter<GenerateMazeMessage>) {
    generate_maze_message.write(GenerateMazeMessage(91));
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
