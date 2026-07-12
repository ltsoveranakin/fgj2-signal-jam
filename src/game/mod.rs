mod camera;
mod maze;
mod player;

use crate::game::camera::GameCameraPlugin;
use crate::game::maze::MazePlugin;
use crate::game::player::PlayerPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MazePlugin, GameCameraPlugin, PlayerPlugin));
    }
}
