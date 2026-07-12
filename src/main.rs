mod debug;
mod game;

use crate::debug::DebugPlugin;
use crate::game::GamePlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // def plugins
        .add_plugins((TilemapPlugin, RapierPhysicsPlugin::<NoUserData>::default())) // lib plugins
        .add_plugins(DebugPlugin) // dbg tgl plugin
        .add_plugins(GamePlugin); // main game plugin

    app.run();
}
