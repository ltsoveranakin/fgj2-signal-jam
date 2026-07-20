use crate::control::GameState;
use crate::sprite_sheet::story_panel_rect_from_sheet;
use bevy::asset::ErasedAssetLoader;
use bevy::prelude::*;

const TOTAL_STORY_PANELS: usize = 2;

pub(super) struct StoryPanelPlugin;

impl Plugin for StoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StartGameMessage>()
            .add_message::<ShowStoryPanelMessage>();

        app.add_systems(Startup, spawn_story_panel).add_systems(
            Update,
            (
                show_story_panel.run_if(on_message::<ShowStoryPanelMessage>),
                game_start.run_if(on_message::<StartGameMessage>),
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
pub(super) struct ShowStoryPanelMessage;

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

fn show_story_panel(mut story_panel_container_query: Query<&mut Node, With<StoryPanelContainer>>) {
    let mut container_node = story_panel_container_query.single_mut().unwrap();

    container_node.display = Display::Flex;
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
