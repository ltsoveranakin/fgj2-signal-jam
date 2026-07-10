mod maze;

use bevy::app::App;
use bevy::prelude::Plugin;
use crate::game::maze::MazePlugin;

pub (super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MazePlugin);
    }
}