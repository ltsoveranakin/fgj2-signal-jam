pub mod game;

use crate::game::GamePlugin;
use bevy::prelude::App;
use bevy::DefaultPlugins;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugins(GamePlugin);

    app.run();
}
