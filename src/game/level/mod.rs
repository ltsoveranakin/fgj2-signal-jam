pub(super) mod create_level;

use crate::game::level::create_level::{CreateLevelPlugin, UnlockingBorder};
use crate::ui::start_menu::StartGameMessage;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_rapier2d::prelude::{Collider, ColliderDisabled};
use serde::Deserialize;
use std::ops::RangeInclusive;
use std::process::exit;

static LEVEL_COUNT: usize = 2;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<LevelData>::new(&["json"]))
            .add_plugins(CreateLevelPlugin);

        app.init_resource::<LevelsData>()
            .init_resource::<CurrentLevel>();

        app.add_message::<UnlockLevelMessage>();

        app.add_systems(Startup, load_levels)
            .add_systems(Update, (rcv_start_game, unlock_level));
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
    story: Option<RangeInclusive<usize>>,
    exit: IVec2,
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
    wall_type: WallType,
}

#[derive(Deserialize, Component, Copy, Clone)]
enum WallType {
    Solid,
    Hole,
}

#[derive(Message, Default)]
pub(super) struct UnlockLevelMessage;

#[derive(Resource, Default)]
pub(crate) struct CurrentLevel(Option<usize>);

impl CurrentLevel {
    pub(crate) fn next(&mut self) {
        if let Some(level) = &mut self.0 {
            *level += 1;
        } else {
            self.0 = None;
        }
    }

    pub(crate) fn prev(&mut self) {
        if let Some(level) = &mut self.0 {
            if *level > 0 {
                *level -= 1;
            }
        }
    }
}

fn load_levels(mut levels_data: ResMut<LevelsData>, assert_server: Res<AssetServer>) {
    for i in 0..LEVEL_COUNT {
        let level_data: Handle<LevelData> = assert_server.load(format!("level/level_{i}.json"));

        levels_data.levels.push(level_data);

        info!("Loaded level {}", i);
    }
}

fn rcv_start_game(
    mut start_game_message: MessageReader<StartGameMessage>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for start_game in start_game_message.read() {
        if *start_game == StartGameMessage::Normal {
            current_level.0 = Some(0);
        }
    }
}

fn unlock_level(
    mut commands: Commands,
    unlocking_border_query: Query<Entity, With<UnlockingBorder>>,
    mut unlock_level_message: MessageReader<UnlockLevelMessage>,
) {
    for _ in unlock_level_message.read() {
        let entity = unlocking_border_query.single().unwrap();

        commands.entity(entity).insert(ColliderDisabled);
    }
}
