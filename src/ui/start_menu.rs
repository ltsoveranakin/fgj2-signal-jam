use crate::ui::story_panel::ShowStoryPanelMessage;
pub(crate) use crate::ui::story_panel::StartGameMessage;
use bevy::color::palettes::css;
use bevy::prelude::*;

pub(super) struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui_elements).add_systems(
            Update,
            (
                start_game_click,
                maze_btn_click,
                hide_root_ui_on_game_start.run_if(on_message::<ShowStoryPanelMessage>),
            ),
        );
    }
}

#[derive(Component, Default, Copy, Clone)]
struct RootContainer;

#[derive(Component, Default, Copy, Clone)]
pub(crate) struct MazeButton;

#[derive(Component)]
struct PlayButton;

fn spawn_ui_elements(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bg_image = asset_server.load("image/menu/menu_bg.png");

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ImageNode::new(bg_image),
        RootContainer,
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
                button("Maze", Display::None, MazeButton)
            ]
        )],
    ));
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

fn hide_root_ui_on_game_start(
    mut root_container_query: Query<&mut Visibility, With<RootContainer>>,
) {
    let mut root_visibility = root_container_query.single_mut().unwrap();

    *root_visibility = Visibility::Hidden;
}

fn start_game_click(
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut show_story_panel_message: MessageWriter<ShowStoryPanelMessage>,
) {
    for interaction in start_button_query.iter() {
        if *interaction == Interaction::Pressed {
            show_story_panel_message.write_default();
        }
    }
}

fn maze_btn_click(
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<MazeButton>)>,
    mut start_game_message: MessageWriter<StartGameMessage>,
) {
    for interaction in start_button_query.iter() {
        if *interaction == Interaction::Pressed {
            start_game_message.write(StartGameMessage::Maze);
        }
    }
}
