use bevy::math::{Rect, Vec2};

fn image_rect_from_sheet(
    index: usize,
    sprite_dimensions: Vec2,
    padding: f32,
    offset: Vec2,
) -> Rect {
    let start = Vec2::new((sprite_dimensions.x + padding) * index as f32, 0.0) + offset;
    let end = start + sprite_dimensions;

    Rect::from_corners(start, end)
}

pub(super) fn default_sheet(index: usize, sprite_dimensions: Vec2) -> Rect {
    const PADDING: f32 = 1.0;
    const OFFSET: Vec2 = Vec2::new(1.0, 1.0);

    image_rect_from_sheet(index, sprite_dimensions, PADDING, OFFSET)
}

pub(super) fn story_board_rect_from_sheet(index: usize) -> Rect {
    const BOARD_DIM: Vec2 = Vec2::new(128.0, 32.0);

    default_sheet(index, BOARD_DIM)
}

pub(super) fn story_panel_rect_from_sheet(index: usize) -> Rect {
    const PANEL_DIM: Vec2 = Vec2::new(512.0, 512.0);

    default_sheet(index, PANEL_DIM)
}
