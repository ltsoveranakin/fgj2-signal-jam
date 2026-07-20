use crate::audio::FadeAudio;
use crate::ui::credits_panel::ShowCreditsMessage;
use crate::ui::story_panel::ShowStoryPanelMessage;
pub(crate) use crate::ui::story_panel::StartGameMessage;
use bevy::audio::{PlaybackMode, Volume};
use bevy::color::palettes::css;
use bevy::prelude::*;
use std::time::Duration;

const MENU_VOLUME: f32 = 0.2;

pub(super) struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui_elements).add_systems(
            Update,
            (
                show_story_panel,
                maze_button_click,
                credits_button_click,
                hide_root_ui_on_game_start.run_if(on_message::<ShowStoryPanelMessage>),
                mute_menu_music.run_if(on_message::<StartGameMessage>),
            ),
        );
    }
}

#[derive(Component, Default, Copy, Clone)]
pub(super) struct StartMenuContainer;

#[derive(Component, Default, Copy, Clone)]
pub(crate) struct MazeButton;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct CreditsButton;

fn spawn_ui_elements(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bg_image = asset_server.load("image/menu/menu_bg.png");

    commands.spawn((
        StartMenuContainer,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ImageNode::new(bg_image),
        AudioPlayer::new(asset_server.load("audio/music/spacey_suspence.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.0),
            start_position: Some(Duration::from_secs_f32(1.5)),
            duration: Some(Duration::from_mins(2)),
            ..default()
        },
        FadeAudio::fade_in(MENU_VOLUME),
        children![(
            Node {
                width: percent(27),
                height: percent(60),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                (
                    Text::new("Signal Game"),
                    TextColor::from(css::AZURE),
                    TextShadow {
                        offset: Vec2::splat(1.0),
                        color: css::GREY.into()
                    },
                    TextFont::from_font_size(FontSize::Rem(2.0))
                ),
                button("Play", Display::Flex, PlayButton),
                button("Maze", Display::None, MazeButton),
                button("Credits", Display::Flex, CreditsButton),
            ]
        )],
    ));
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

fn hide_root_ui_on_game_start(
    mut root_container_query: Query<&mut Visibility, With<StartMenuContainer>>,
) {
    let mut root_visibility = root_container_query.single_mut().unwrap();

    *root_visibility = Visibility::Hidden;
}

fn show_story_panel(
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut show_story_panel_message: MessageWriter<ShowStoryPanelMessage>,
) {
    for interaction in start_button_query.iter() {
        if *interaction == Interaction::Pressed {
            show_story_panel_message.write_default();
        }
    }
}

fn maze_button_click(
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<MazeButton>)>,
    mut start_game_message: MessageWriter<StartGameMessage>,
) {
    for interaction in start_button_query.iter() {
        if *interaction == Interaction::Pressed {
            start_game_message.write(StartGameMessage::Maze);
        }
    }
}

fn credits_button_click(
    credits_button_query: Query<&Interaction, (Changed<Interaction>, With<CreditsButton>)>,
    mut show_credits_message: MessageWriter<ShowCreditsMessage>,
) {
    for interaction in credits_button_query.iter() {
        if *interaction == Interaction::Pressed {
            show_credits_message.write_default();
        }
    }
}

fn button(text: &str, display: Display, marker: impl Bundle) -> impl Bundle {
    (
        Button,
        Text::new(text),
        Node {
            display,
            padding: UiRect::all(px(5.0)),
            border: UiRect::all(px(5.0)),
            border_radius: BorderRadius::all(px(5.0)),
            margin: UiRect::vertical(px(20.0)),
            ..default()
        },
        BorderColor::all(Srgba::rgb_u8(36, 36, 36)),
        BackgroundColor::from(css::GREY),
        TextFont::from_font_size(FontSize::Rem(1.5)),
        marker,
    )
}
