use crate::game::maze::PlaceMazeObjectsMessage;
use crate::game::maze::generator::TILE_SIZE_U32;
use crate::game::z_coord::TARGET_Z_COORD;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(super) struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_startup_target)
            .add_systems(Update, place_target_in_maze);
    }
}

#[derive(Component)]
pub(super) struct GameTarget;

fn spawn_startup_target(mut commands: Commands, asset_server: Res<AssetServer>) {
    let target_image = asset_server.load("image/character/target.png");

    commands.spawn((
        GameTarget,
        Sprite::from_image(target_image),
        Visibility::Hidden,
        Transform::from_xyz(0.0, 0.0, TARGET_Z_COORD),
        Collider::capsule_x(6.0, 3.0),
        Sensor,
    ));
}

fn place_target_in_maze(
    mut target_query: Query<&mut Transform, With<GameTarget>>,
    mut place_maze_objects_message: MessageReader<PlaceMazeObjectsMessage>,
) {
    for place_maze_objects in place_maze_objects_message.read() {
        let mut target_transform = target_query.single_mut().unwrap();

        target_transform.translation = (place_maze_objects.target * (TILE_SIZE_U32 as usize))
            .as_vec2()
            .extend(target_transform.translation.z);
    }
}
