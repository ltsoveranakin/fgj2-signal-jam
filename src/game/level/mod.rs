mod create_level;

use crate::game::level::create_level::CreateLevelPlugin;
use crate::ui::start_menu::StartGameMessage;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;
use std::ops::RangeInclusive;

static LEVEL_COUNT: usize = 1;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<LevelData>::new(&["json"]))
            .add_plugins(CreateLevelPlugin);

        app.init_resource::<LevelsData>();

        app.add_message::<SetLevelMessage>();

        app.add_systems(Startup, load_levels)
            .add_systems(Update, send_set_level);
    }
}

#[derive(Resource, Default)]
struct LevelsData {
    levels: Vec<Handle<LevelData>>,
}

#[derive(Asset, TypePath, Deserialize)]
struct LevelData {
    player_spawn: UVec2,
    target_spawn: UVec2,
    level_size: UVec2,
    walls: Vec<WallData>,
    story: RangeInclusive<usize>,
}

#[derive(Deserialize)]
struct WallData {
    x: u32,
    y: u32,
    #[serde(rename = "h")]
    height: u32,
    #[serde(rename = "w")]
    width: u32,
    #[serde(rename = "ty")]
    wall_type: u32,
}

#[derive(Message)]
struct SetLevelMessage(usize);

fn load_levels(mut levels_data: ResMut<LevelsData>, assert_server: Res<AssetServer>) {
    for i in 0..LEVEL_COUNT {
        let level_data: Handle<LevelData> = assert_server.load(format!("level/level_{i}.json"));

        levels_data.levels.push(level_data);

        info!("Loaded level {}", i);
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
