pub mod generator;

use crate::game::maze::generator::{GenerateMazeMessage, GeneratorPlugin};
use bevy::prelude::*;

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

fn startup_generate(mut generate_maze_message: MessageWriter<GenerateMazeMessage>) {
    generate_maze_message.write(GenerateMazeMessage(19));
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
