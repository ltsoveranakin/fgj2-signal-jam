pub(crate) mod credits_panel;
pub(super) mod start_menu;
pub(super) mod story_panel;

use crate::ui::credits_panel::CreditsPanelPlugin;
use crate::ui::start_menu::StartMenuPlugin;
use crate::ui::story_panel::StoryPanelPlugin;
use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};

pub(super) const TITLE_FONT_SIZE: FontSize = FontSize::Rem(2.4);

pub(super) struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StartMenuPlugin, StoryPanelPlugin, CreditsPanelPlugin));

        app.add_systems(Update, button_mouse_pointer);
    }
}

fn button_mouse_pointer(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (With<Button>, Changed<Interaction>)>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    for window_entity in window_query.iter() {
        for interaction in interaction_query.iter() {
            match interaction {
                Interaction::Hovered => {
                    commands
                        .entity(window_entity)
                        .insert(CursorIcon::System(SystemCursorIcon::Pointer));
                }
                Interaction::None => {
                    commands
                        .entity(window_entity)
                        .insert(CursorIcon::System(SystemCursorIcon::Default));
                }
                _ => {}
            }
        }
    }
}
