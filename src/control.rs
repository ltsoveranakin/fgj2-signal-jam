use crate::game::story_board::StoryBoard;
use bevy::prelude::*;

pub(super) struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::NotPlaying);
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum GameState {
    NotPlaying,
    Playing,
}

pub(super) fn inputs_allowed(
    story_board_query: Query<&Node, With<StoryBoard>>,
    game_state: Res<State<GameState>>,
) -> bool {
    is_playing(game_state) && !story_board_shown(story_board_query)
}

pub(super) fn is_playing(game_state: Res<State<GameState>>) -> bool {
    *game_state == GameState::Playing
}

pub(super) fn story_board_shown(story_board_query: Query<&Node, With<StoryBoard>>) -> bool {
    story_board_query.single().unwrap().display == Display::Flex
}
