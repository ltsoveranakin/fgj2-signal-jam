pub mod generator;

use bevy::app::App;
use bevy::prelude::Plugin;
use crate::game::maze::generator::GeneratorPlugin;

pub(super) struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GeneratorPlugin);
    }
}
