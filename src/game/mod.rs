mod camera;
pub(super) mod level;
pub(super) mod lidar;

mod glow;
mod outro;
mod player;
pub(super) mod story_board;
mod target;
mod z_coord;

use crate::game::camera::GameCameraPlugin;
use crate::game::glow::GlowPlugin;
use crate::game::level::LevelPlugin;
use crate::game::lidar::LidarPlugin;
use crate::game::outro::OutroPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::story_board::StoryBoardPlugin;
use crate::game::target::TargetPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameCameraPlugin,
            PlayerPlugin,
            LidarPlugin,
            TargetPlugin,
            LevelPlugin,
            StoryBoardPlugin,
            GlowPlugin,
            OutroPlugin,
        ));
    }
}
