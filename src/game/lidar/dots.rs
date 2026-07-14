use crate::game::lidar::{LIDAR_DOTS_COUNT, LidarCasts};
use bevy::prelude::*;

const LIDAR_DOT_SPEED: f32 = 10.0;

pub(super) struct LidarDotsPlugin;

impl Plugin for LidarDotsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_dots)
            .add_systems(Update, move_dots);
    }
}

#[derive(Component)]
struct LidarDot {
    cast_index: usize,
    goal_position_index: usize,
}

fn spawn_initial_dots(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..LIDAR_DOTS_COUNT {
        commands.spawn((
            LidarDot {
                cast_index: i,
                goal_position_index: 0,
            },
            Sprite::from_image(asset_server.load("image/particle/lidar_dot.png")),
            Visibility::Hidden,
        ));
    }
}

fn move_dots(
    mut dot_query: Query<(&mut Transform, &mut Visibility, &mut LidarDot)>,
    lidar_casts: Res<LidarCasts>,
    time: Res<Time>,
) {
    // for (mut dot_transform, mut dot_visibility, mut lidar_dot) in dot_query.iter_mut() {
    //     lidar_casts.
    // }
}
