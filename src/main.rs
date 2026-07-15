mod assign_vec;
mod debug;
mod game;
mod ui;

use crate::debug::DebugPlugin;
use crate::game::GamePlugin;
use crate::ui::GameUIPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // def plugins
        .add_plugins((TilemapPlugin, RapierPhysicsPlugin::<NoUserData>::default())) // lib plugins
        .add_plugins(DebugPlugin) // dbg tgl plugin
        .add_plugins((GamePlugin, GameUIPlugin)); // main game plugins

    app.run();
}
