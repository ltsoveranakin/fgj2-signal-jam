pub mod game;

use crate::game::GamePlugin;
use bevy::prelude::App;
use bevy::DefaultPlugins;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()))
        .add_plugins(GamePlugin);

    app.run();
}
