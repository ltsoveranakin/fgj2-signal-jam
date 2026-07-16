mod create_level;

use crate::game::level::create_level::CreateLevelPlugin;
use crate::ui::start_menu::StartGameMessage;
use bevy::prelude::*;
use serde::Deserialize;

static LEVELS: [&str; 1] = [include_str!("levels/level_0.json")];

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CreateLevelPlugin);

        app.init_resource::<LevelsData>();

        app.add_message::<SetLevelMessage>();

        app.add_systems(Startup, load_levels)
            .add_systems(Update, send_set_level);
    }
}

#[derive(Resource, Default)]
struct LevelsData {
    levels: Vec<LevelData>,
}

#[derive(Deserialize, Default)]
struct LevelData {
    player_spawn: IVec2,
    target_spawn: IVec2,
    level_size: UVec2,
    walls: Vec<WallData>,
}

#[derive(Deserialize)]
struct WallData {
    dim: UVec2,
    coord: UVec2,
}

#[derive(Message)]
struct SetLevelMessage(usize);

fn load_levels(mut levels_data: ResMut<LevelsData>) {
    for (i, level_json) in LEVELS.iter().enumerate() {
        let level_data = serde_json::from_str(level_json).unwrap();
        levels_data.levels.push(level_data);

        println!("Loaded level {}", i);
    }
}

fn send_set_level(
    mut start_game_message: MessageReader<StartGameMessage>,
    mut set_level_message: MessageWriter<SetLevelMessage>,
) {
    for start_game in start_game_message.read() {
        if *start_game == StartGameMessage::Normal {
            set_level_message.write(SetLevelMessage(0));
        }
    }
}
