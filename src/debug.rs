use crate::ui::start_menu::MazeButton;
use bevy::anti_alias::contrast_adaptive_sharpening::cas;
use bevy::color::palettes::css;
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

        app.init_resource::<DebugMode>();

        app.add_systems(
            Update,
            (
                toggle_debug_mode,
                (set_physics_renderer, set_shown_maze_button).run_if(resource_changed::<DebugMode>),
            ),
        );
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct DebugMode {
    enabled: bool,
}

fn toggle_debug_mode(
    mut keyboard_input: MessageReader<KeyboardInput>,
    mut debug_mode: ResMut<DebugMode>,
) {
    for key in keyboard_input.read() {
        if key.key_code != KeyCode::Backquote || key.state != ButtonState::Pressed || key.repeat {
            continue;
        }

        debug_mode.enabled = !debug_mode.enabled;
    }
}

fn set_physics_renderer(
    mut debug_render_context: ResMut<DebugRenderContext>,
    debug_mode: Res<DebugMode>,
) {
    debug_render_context.enabled = debug_mode.enabled;
}

fn set_shown_maze_button(
    mut maze_button_query: Query<&mut Node, With<MazeButton>>,
    debug_mode: Res<DebugMode>,
) {
    let mut maze_button_node = maze_button_query.single_mut().unwrap();

    maze_button_node.display = if debug_mode.enabled {
        Display::DEFAULT
    } else {
        Display::None
    };
}
