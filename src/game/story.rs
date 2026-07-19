use crate::control::story_board_shown;
use crate::game::maze::LevelReadyMessage;
use bevy::prelude::*;
use std::ops::RangeInclusive;

const PADDING: f32 = 1.0;
const OFFSET: Vec2 = Vec2::new(1.0, 1.0);
const BOARD_DIM: Vec2 = Vec2::new(128.0, 32.0);

pub(super) struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_story_root_ui_components)
            .add_systems(
                Update,
                (
                    show_story_board_on_ready,
                    (click_story_board, enter_story_board).run_if(story_board_shown),
                    update_story_board,
                ),
            );
    }
}

#[derive(Component)]
pub(crate) struct StoryBoard {
    board_range: Option<BoardRange>,
}

impl StoryBoard {
    fn advance(&mut self) {
        if let Some(board_range) = &mut self.board_range {
            board_range.current_index += 1;
        } else {
            return;
        };
    }
}

#[derive(Debug)]
struct BoardRange {
    range: RangeInclusive<usize>,
    current_index: usize,
}

fn story_board_image_slice(story_index: usize) -> Rect {
    let start = Vec2::new((BOARD_DIM.x + PADDING) * story_index as f32, 0.0) + OFFSET;
    let end = start + BOARD_DIM;

    Rect::from_corners(start, end)
}

fn spawn_story_root_ui_components(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                ..default()
            },
            Name::new("Story Board Container"),
        ))
        .with_child((
            Node {
                width: percent(100),
                height: percent(10),
                ..default()
            },
            ImageNode {
                image: asset_server.load("image/story/story_boards.png"),
                rect: Some(story_board_image_slice(0)),
                ..default()
            },
            StoryBoard { board_range: None },
            Button,
        ));
}

fn show_story_board_on_ready(
    mut story_board_query: Query<&mut StoryBoard>,
    mut level_ready_message: MessageReader<LevelReadyMessage>,
) {
    for level_ready in level_ready_message.read() {
        let mut story_board = story_board_query.single_mut().unwrap();

        let story_index_range = if let Some(range) = level_ready.story_index_range.clone() {
            range
        } else {
            continue;
        };

        let start = *story_index_range.start();

        story_board.board_range = Some(BoardRange {
            range: story_index_range,
            current_index: start,
        });
    }
}

fn update_story_board(
    mut story_board_query: Query<(&StoryBoard, &mut ImageNode, &mut Node), Changed<StoryBoard>>,
) {
    for (story_board, mut image_node, mut node) in story_board_query.iter_mut() {
        let mut hide_board = false;

        if let Some(board_range) = &story_board.board_range {
            if board_range.current_index > *board_range.range.end() {
                hide_board = true;
            } else {
                node.display = Display::Flex;

                image_node.rect = Some(story_board_image_slice(board_range.current_index));
            }
        } else {
            hide_board = true;
        };

        if hide_board {
            node.display = Display::None
        }
    }
}

fn click_story_board(
    mut story_board_query: Query<(&mut StoryBoard, &Interaction), Changed<Interaction>>,
) {
    for (mut story_board, interaction) in story_board_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            story_board.advance();
        }
    }
}

fn enter_story_board(
    mut board_query: Query<&mut StoryBoard>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::Enter) {
        let mut story_board = board_query.single_mut().unwrap();
        story_board.advance();
    }
}
