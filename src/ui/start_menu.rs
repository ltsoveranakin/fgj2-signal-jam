use bevy::color::palettes::css;
use bevy::prelude::*;

pub(super) struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StartGameMessage>();

        app.add_systems(Startup, spawn_ui_elements).add_systems(
            Update,
            hide_root_ui_on_game_start.run_if(on_message::<StartGameMessage>),
        );
    }
}

#[derive(Message, Eq, PartialEq)]
pub(crate) enum StartGameMessage {
    Normal,
    Maze,
}

#[derive(Component, Default, Copy, Clone)]
struct RootContainer;

#[derive(Component, Default, Copy, Clone)]
pub(crate) struct MazeButton;

fn spawn_ui_elements(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #RootContainer
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        RootContainer
        Children [
            (
                #StartMenu
                BackgroundColor(css::RED)
                Node {
                    width: percent(27),
                    height: percent(80),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                }
                Children [
                    (
                        #TitleText
                        Text("Signal Game")
                        TextColor(css::BLACK)
                    ),
                    (
                        #PlayButton
                        Button
                        Text::new("Play")
                        Node {
                            width: percent(50),
                            height: percent(20),
                        }
                        on(|_event: On<Pointer<Press>>, mut start_game_message: MessageWriter<StartGameMessage>,| {
                            start_game_message.write(StartGameMessage::Normal);
                        })
                    ),
                    (
                        #MazePlayButton
                        Button
                        Text::new("Maze Mode")
                        MazeButton
                        Node {
                            width: percent(50),
                            height: percent(20),
                            display: Display::None,
                        }
                        on(|_event: On<Pointer<Press>>, mut start_game_message: MessageWriter<StartGameMessage>,| {
                            start_game_message.write(StartGameMessage::Maze);
                        })
                    )
                ]
            )
        ]
    });
}

fn hide_root_ui_on_game_start(
    mut root_container_query: Query<&mut Visibility, With<RootContainer>>,
) {
    let mut root_visibility = root_container_query.single_mut().unwrap();

    *root_visibility = Visibility::Hidden;
}
