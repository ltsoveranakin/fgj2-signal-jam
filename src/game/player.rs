use crate::game::maze::generator::MazeReadyMessage;
use bevy::prelude::*;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player_to_maze);
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Transform::default(),
        Visibility::Hidden,
        Sprite::from_image(asset_server.load("image/character/player.png")),
    ));
}

fn move_player_to_maze(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut maze_ready_message: MessageReader<MazeReadyMessage>,
) {
    for maze_ready in maze_ready_message.read() {
        let mut player_transform = player_query.single_mut().unwrap();

        find_points_in_maze()
    }
}

fn find_points_in_maze() {}
