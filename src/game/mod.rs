pub mod camera;
mod maze;

use crate::game::camera::GameCameraPlugin;
use crate::game::maze::MazePlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MazePlugin, GameCameraPlugin));
    }
}
