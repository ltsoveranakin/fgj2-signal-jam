use crate::game::maze::{MazeTileImageAssets, PATH_INDEX, WALL_INDEX};
use bevy::ecs::relationship::OrderedRelationshipSourceCollection;
use bevy::math::USizeVec2;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::TilemapBundle;
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};
use smallvec::SmallVec;

const TILE_SIZE: u32 = 16;

pub(super) struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GenerateMazeMessage>()
            .add_systems(Update, generate_maze);
    }
}

#[derive(Message)]
pub(super) struct GenerateMazeMessage(pub(super) usize);

#[derive(Copy, Clone)]
enum MazeCellState {
    Wall,
    Path,
}

impl MazeCellState {
    fn is_wall(&self) -> bool {
        match self {
            MazeCellState::Wall => true,

            MazeCellState::Path => false,
        }
    }

    fn is_path(&self) -> bool {
        !self.is_wall()
    }
}

fn generate_maze(
    mut commands: Commands,
    maze_tile_image_assets: Res<MazeTileImageAssets>,
    mut generate_maze_message: MessageReader<GenerateMazeMessage>,
) {
    for maze_msg in generate_maze_message.read() {
        let maze_matrix = generate_maze_from_dimensions(maze_msg.0);

        let maze_size = maze_msg.0 as u32;

        let map_size = TilemapSize::new(maze_size, maze_size);

        let mut tile_storage = TileStorage::empty(map_size);

        let tilemap_entity = commands.spawn_empty().id();

        let mut i = 0;
        for y in 0..maze_size {
            for x in 0..maze_size {
                let tile_pos = TilePos::new(x, y);
                let cell = maze_matrix[i];

                let image_index = match cell {
                    MazeCellState::Path => PATH_INDEX,

                    MazeCellState::Wall => WALL_INDEX,
                };

                let tile_entity = commands
                    .entity(tilemap_entity)
                    .with_child((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(image_index as u32),
                            ..default()
                        },
                        Name::new("Tile"),
                    ))
                    .id();

                tile_storage.set(&tile_pos, tile_entity);

                i += 1;
            }
        }

        let tile_size = TilemapTileSize::new(TILE_SIZE as f32, TILE_SIZE as f32);
        let grid_size = tile_size.into();
        let map_type = TilemapType::Square;

        commands.entity(tilemap_entity).insert((
            TilemapBundle {
                grid_size,
                map_type,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Vector(maze_tile_image_assets.to_vec()),
                tile_size,
                transform: Transform::default(),
                ..default()
            },
            Name::new("TileMap"),
        ));
    }
}

fn us_get_cell_index(usize_vec2: USizeVec2, size: usize) -> usize {
    get_cell_index(usize_vec2.x, usize_vec2.y, size)
}

fn get_cell_index(x: usize, y: usize, size: usize) -> usize {
    x + (y * size)
}

fn generate_maze_from_dimensions(maze_size: usize) -> Vec<MazeCellState> {
    let mut rng = SmallRng::seed_from_u64(10);

    let mut maze = vec![MazeCellState::Wall; maze_size * maze_size];
    let mut visited = vec![false; maze_size * maze_size];

    let room_count = (maze_size - 1) / 2;

    let start = USizeVec2::new(
        (rng.random_range(0..room_count) * 2) + 1,
        (rng.random_range(0..room_count) * 2) + 1,
    );

    let mut stack = Vec::new();

    let start_index = us_get_cell_index(start, maze_size);
    visited[start_index] = true;
    maze[start_index] = MazeCellState::Path;

    stack.push(start);

    while let Some(&current) = stack.last() {
        let neighbors = get_rooms_over_offsets(current, maze_size, &visited);

        if neighbors.is_empty() {
            stack.pop();
            continue;
        }

        let (next_room, wall_between) = neighbors[rng.random_range(0..neighbors.len())];

        let room_index = us_get_cell_index(next_room, maze_size);
        let wall_index = us_get_cell_index(wall_between, maze_size);

        visited[room_index] = true;

        maze[wall_index] = MazeCellState::Path;
        maze[room_index] = MazeCellState::Path;

        stack.push(next_room);
    }

    maze
}

fn get_rooms_over_offsets(
    pos: USizeVec2,
    maze_size: usize,
    visited: &[bool],
) -> SmallVec<[(USizeVec2, USizeVec2); 4]> {
    const OFFSETS: [IVec2; 4] = [IVec2::Y, IVec2::X, IVec2::NEG_Y, IVec2::NEG_X];

    let pos = pos.as_ivec2();
    let maze_size_i32 = maze_size as i32;

    let mut neighbors = SmallVec::new();

    for dir in OFFSETS {
        let room = pos + dir * 2;

        if room.x <= 0 || room.y <= 0 || room.x >= maze_size_i32 || room.y >= maze_size_i32 {
            continue;
        }

        let room = room.as_usizevec2();

        if visited[us_get_cell_index(room, maze_size)] {
            continue;
        }

        let wall = (pos + dir).as_usizevec2();

        neighbors.push((room, wall));
    }

    neighbors
}

fn print_maze(maze_matrix: &[MazeCellState], maze_size: usize) {
    let mut x = 0;

    for cell in maze_matrix.iter() {
        if cell.is_path() {
            print!("O");
        } else {
            print!("X");
        }

        if x == maze_size - 1 {
            println!();
            x = 0;
            continue;
        }

        x += 1;
    }
}
