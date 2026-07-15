pub(super) mod start_menu;

use crate::ui::start_menu::StartMenuPlugin;
use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};

pub(super) struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StartMenuPlugin);

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
