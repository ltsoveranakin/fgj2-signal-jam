use crate::control::GameState;
use crate::sprite_sheet::story_panel_rect_from_sheet;
use bevy::audio::{PlaybackMode, Volume};
use std::time::Duration;

use crate::audio::FadeAudio;
use crate::ui::start_menu::StartMenuContainer;
use bevy::prelude::*;

const TOTAL_STORY_PANELS: usize = 2;
const MENU_VOLUME: f32 = 0.2;

pub(super) struct StoryPanelPlugin;

impl Plugin for StoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StartGameMessage>()
            .add_message::<ShowStoryPanelMessage>();

        app.add_systems(Startup, spawn_story_panel).add_systems(
            Update,
            (
                show_story_panel.run_if(on_message::<ShowStoryPanelMessage>),
                (mute_menu_music, game_start).run_if(on_message::<StartGameMessage>),
                story_panel_clicked,
            ),
        );
    }
}

#[derive(Component)]
struct StoryPanelContainer;

#[derive(Message, Default, Eq, PartialEq)]
pub(crate) struct StartGameMessage;

#[derive(Message, Default)]
pub(crate) struct ShowStoryPanelMessage;

#[derive(Component)]
pub(super) struct StoryPanel {
    index: usize,
}

fn spawn_story_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                display: Display::None,
                justify_content: JustifyContent::Center,
                ..default()
            },
            StoryPanelContainer,
        ))
        .with_child((
            StoryPanel { index: 0 },
            Node {
                height: vmin(100),
                width: vmin(100),
                ..default()
            },
            ImageNode {
                image: asset_server.load("image/menu/story_panels.png"),
                rect: Some(story_panel_rect_from_sheet(0)),
                ..default()
            },
            Button,
        ));
}

fn show_story_panel(
    mut commands: Commands,
    mut story_panel_container_query: Query<(Entity, &mut Node), With<StoryPanelContainer>>,
    asset_server: Res<AssetServer>,
) {
    let (entity, mut container_node) = story_panel_container_query.single_mut().unwrap();

    commands.entity(entity).insert((
        AudioPlayer::new(asset_server.load("audio/music/spacey_suspence.mp3")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.0),
            start_position: Some(Duration::from_secs_f32(1.5)),
            duration: Some(Duration::from_mins(2)),
            ..default()
        },
        FadeAudio::fade_in(MENU_VOLUME),
    ));

    container_node.display = Display::Flex;
}

fn mute_menu_music(
    mut commands: Commands,
    mut menu_query: Query<Entity, With<StartMenuContainer>>,
) {
    let entity = menu_query.single_mut().unwrap();

    commands
        .entity(entity)
        .insert(FadeAudio::fade_out(MENU_VOLUME));
}

fn game_start(
    mut story_panel_container_query: Query<&mut Node, With<StoryPanelContainer>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut container_node = story_panel_container_query.single_mut().unwrap();

    container_node.display = Display::None;

    game_state.set(GameState::Playing);
}

fn story_panel_clicked(
    mut story_panel_query: Query<
        (&mut StoryPanel, &mut ImageNode, &Interaction),
        Changed<Interaction>,
    >,
    mut start_game_message: MessageWriter<StartGameMessage>,
) {
    for (mut story_panel, mut image_node, interaction) in story_panel_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            story_panel.index += 1;

            if story_panel.index == TOTAL_STORY_PANELS {
                start_game_message.write_default();
                return;
            }

            image_node.rect = Some(story_panel_rect_from_sheet(story_panel.index));
        }
    }
}
