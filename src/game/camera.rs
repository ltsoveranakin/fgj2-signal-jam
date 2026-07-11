use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub(super) struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, zoom_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
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
