pub(super) mod create_level;

use crate::game::level::create_level::{CreateLevelPlugin, UnlockingBorder};

use crate::ui::story_panel::StartGameMessage;
use bevy::math::USizeVec2;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::ops::RangeInclusive;

const LEVEL_COUNT: usize = 2;

pub(crate) const TILE_SIZE_U32: u32 = 16;
pub(crate) const TILE_SIZE_F32: f32 = TILE_SIZE_U32 as f32;
pub(crate) const HALF_TILE_SIZE_F32: f32 = (TILE_SIZE_U32 / 2) as f32;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<LevelData>::new(&["json"]))
            .add_plugins(CreateLevelPlugin);

        app.add_message::<UnlockLevelMessage>()
            .add_message::<LevelReadyMessage>();

        app.init_resource::<LevelsData>()
            .init_resource::<CurrentLevel>();

        app.add_systems(Startup, load_levels).add_systems(
            Update,
            (
                rcv_start_game.run_if(on_message::<StartGameMessage>),
                unlock_level,
            ),
        );
    }
}

#[derive(Message)]
pub(super) struct LevelReadyMessage {
    pub(super) player: USizeVec2,
    pub(super) target: USizeVec2,
    pub(super) story_index_range: Option<RangeInclusive<usize>>,
}

#[derive(Resource, Default, Debug)]
pub(crate) struct LevelsData {
    pub(crate) levels: Vec<Handle<LevelData>>,
}

#[derive(Asset, TypePath, Deserialize, Debug)]
pub(crate) struct LevelData {
    player_spawn: UVec2,
    target_spawn: UVec2,
    level_size: UVec2,
    walls: Vec<WallData>,
    story: Option<RangeInclusive<usize>>,
    exit: IVec2,
}

#[derive(Deserialize, Debug)]
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

#[derive(Component, Deserialize, Copy, Clone, Debug)]
pub(super) enum WallType {
    Solid,
    Absorb,
    Exit,
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

    pub(crate) fn get(&self) -> usize {
        self.0.unwrap_or_default()
    }
}

fn load_levels(mut levels_data: ResMut<LevelsData>, assert_server: Res<AssetServer>) {
    for i in 0..LEVEL_COUNT {
        let level_data: Handle<LevelData> = assert_server.load(format!("level/level_{i}.json"));

        levels_data.levels.push(level_data);

        info!("Loaded level {}", i);
    }
}

fn rcv_start_game(mut current_level: ResMut<CurrentLevel>) {
    current_level.0 = Some(0);
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
