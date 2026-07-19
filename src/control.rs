use crate::game::story::StoryBoard;
use bevy::prelude::*;

pub(super) fn inputs_allowed(story_board_query: Query<&Node, With<StoryBoard>>) -> bool {
    !story_board_shown(story_board_query)
}

pub(super) fn story_board_shown(story_board_query: Query<&Node, With<StoryBoard>>) -> bool {
    story_board_query.single().unwrap().display == Display::Flex
}
