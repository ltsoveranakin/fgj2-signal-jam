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

        app.add_systems(Update, toggle_physics_renderer);
    }
}

fn toggle_physics_renderer(
    mut keyboard_input: MessageReader<KeyboardInput>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    for key in keyboard_input.read() {
        if key.key_code != KeyCode::Backquote || key.state != ButtonState::Pressed || key.repeat {
            continue;
        }

        debug_render_context.enabled = !debug_render_context.enabled;
    }
}
