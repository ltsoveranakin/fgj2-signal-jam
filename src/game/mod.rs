mod camera;
mod level;
pub(super) mod lidar;
mod maze;
mod player;
mod story;
mod target;
mod z_coord;

use crate::game::camera::GameCameraPlugin;
use crate::game::level::LevelPlugin;
use crate::game::lidar::LidarPlugin;
use crate::game::maze::MazePlugin;
use crate::game::player::PlayerPlugin;
use crate::game::story::StoryPlugin;
use crate::game::target::TargetPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MazePlugin,
            GameCameraPlugin,
            PlayerPlugin,
            LidarPlugin,
            TargetPlugin,
            LevelPlugin,
            StoryPlugin,
        ));
    }
}
