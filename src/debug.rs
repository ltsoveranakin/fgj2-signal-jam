use crate::game::level::CurrentLevel;
use crate::ui::start_menu::MazeButton;
use bevy::input::ButtonState;
use bevy::input::common_conditions::input_toggle_active;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{DebugRenderContext, RapierDebugRenderPlugin};

pub(super) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierDebugRenderPlugin::default().disabled(),
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Backquote)),
        ));

        app.insert_resource(DebugModeEnabled::disabled());

        app.add_systems(
            Update,
            (
                toggle_debug_mode,
                change_level.run_if(resource_equals(DebugModeEnabled::enabled())),
                (set_physics_renderer, set_shown_maze_button)
                    .run_if(resource_changed::<DebugModeEnabled>),
            ),
        );
    }
}

#[derive(Resource, Reflect, Default, Eq, PartialEq)]
#[reflect(Resource)]
pub(super) struct DebugModeEnabled(bool);

impl DebugModeEnabled {
    pub(super) fn enabled() -> Self {
        Self(true)
    }

    pub(super) fn disabled() -> Self {
        Self(false)
    }
}

fn toggle_debug_mode(
    mut keyboard_input: MessageReader<KeyboardInput>,
    mut debug_mode: ResMut<DebugModeEnabled>,
) {
    for key in keyboard_input.read() {
        if key.key_code != KeyCode::Backquote || key.state != ButtonState::Pressed || key.repeat {
            continue;
        }

        debug_mode.0 = !debug_mode.0;
    }
}

fn set_physics_renderer(
    mut debug_render_context: ResMut<DebugRenderContext>,
    debug_mode: Res<DebugModeEnabled>,
) {
    debug_render_context.enabled = debug_mode.0;
}

fn set_shown_maze_button(
    mut maze_button_query: Query<&mut Node, With<MazeButton>>,
    debug_mode: Res<DebugModeEnabled>,
) {
    let mut maze_button_node = maze_button_query.single_mut().unwrap();

    maze_button_node.display = if debug_mode.0 {
        Display::DEFAULT
    } else {
        Display::None
    };
}

fn change_level(key_input: Res<ButtonInput<KeyCode>>, mut current_level: ResMut<CurrentLevel>) {
    if key_input.just_pressed(KeyCode::Equal) {
        current_level.next();
    }

    if key_input.just_pressed(KeyCode::Minus) {
        current_level.prev();
    }
}
