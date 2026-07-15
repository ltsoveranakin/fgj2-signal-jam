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

        app.add_systems(Update, (toggle_debug_mode, set_physics_renderer));
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
//
// fn draw_cast_gizmos(mut gizmos: Gizmos, lidar_casts: Res<LidarCasts>, debug_mode: Res<DebugMode>) {
//     if !debug_mode.enabled {
//         return;
//     }
//
//     let working_color = if matches!(lidar_casts.cast_state, CastState::Casting) {
//         Srgba::RED
//     } else {
//         Srgba::rgb_u8(245, 152, 66)
//     };
//
//     draw_cast_line(&mut gizmos, &lidar_casts.casts_working, working_color);
//
//     draw_cast_line(&mut gizmos, &lidar_casts.casts_done, Srgba::GREEN);
// }
//
// fn draw_cast_line(gizmos: &mut Gizmos, casts: &[Option<LidarCast>], color: Srgba) {
//     for cast in casts.iter().flatten() {
//         gizmos.linestrip_2d(
//             cast.cast_positions
//                 .iter()
//                 .map(|cast_position| cast_position.position),
//             color,
//         );
//
//         if let Some(last_position) = cast.cast_positions.last() {
//             gizmos.circle_2d(last_position.position, 2.0, css::ORANGE_RED);
//         }
//     }
// }
