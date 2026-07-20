mod assign_vec;
mod audio;
mod control;
mod debug;
mod game;
mod sprite_sheet;
mod ui;

use crate::audio::GameAudioPlugin;
use crate::control::ControlPlugin;
use crate::debug::DebugPlugin;
use crate::game::GamePlugin;
use crate::ui::GameUIPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),)) // default plugins
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default()) // lib plugins
        .add_plugins(DebugPlugin) // dbg tgl plugin
        .add_plugins((GamePlugin, GameUIPlugin, ControlPlugin, GameAudioPlugin)); // main game plugins

    app.run();
}
