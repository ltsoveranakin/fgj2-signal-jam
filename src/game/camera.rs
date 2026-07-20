use crate::debug::DebugModeEnabled;
use crate::game::player::Player;
use bevy::camera::Hdr;
use bevy::input::mouse::MouseWheel;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

const CLEAR_COLOR: Color = Color::srgb_u8(9, 2, 20);

pub(super) struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                zoom_camera.run_if(resource_equals(DebugModeEnabled::enabled())),
            )
            .add_systems(PostUpdate, move_camera_to_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(CLEAR_COLOR),
            ..default()
        },
        Bloom::default(),
        Hdr,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.2,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn zoom_camera(
    mut camera_query: Query<&mut Projection>,
    mut mouse_wheel_message: MessageReader<MouseWheel>,
) {
    for mouse_wheel in mouse_wheel_message.read() {
        let mut projection = camera_query.single_mut().unwrap();

        match &mut *projection {
            Projection::Orthographic(projection) => {
                projection.scale -= mouse_wheel.y / 10.;

                projection.scale = projection.scale.clamp(0.1, 5.);
            }

            _ => unreachable!(),
        }
    }
}

fn move_camera_to_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut().unwrap();

    for player_transform in player_query.iter() {
        let mut player_translation = player_transform.translation;
        player_translation.z = camera_transform.translation.z;

        camera_transform.translation = player_translation;
    }
}
