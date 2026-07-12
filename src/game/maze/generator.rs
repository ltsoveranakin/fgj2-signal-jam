use crate::game::maze::{MazeMatrix, MazeTileImageAssets, PATH_INDEX, WALL_INDEX};
use crate::game::z_coord::MAZE_Z_COORD;
use bevy::ecs::relationship::OrderedRelationshipSourceCollection;
use bevy::math::USizeVec2;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::Collider;
use rand::RngExt;
use smallvec::SmallVec;

pub(crate) const TILE_SIZE_U32: u32 = 16;
const TILE_SIZE_F32: f32 = TILE_SIZE_U32 as f32;
const HALF_TILE_SIZE_F32: f32 = (TILE_SIZE_U32 / 2) as f32;

pub(super) struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GenerateMazeMessage>()
            .add_message::<MazeReadyMessage>()
            .add_systems(PostUpdate, create_maze); // Allow for maze to be fully spawned in when the event is consumed
    }
}

#[derive(Message)]
pub(super) struct GenerateMazeMessage {
    pub(super) size: usize,
    pub(super) seed: u64,
}

#[derive(Message)]
pub(crate) struct MazeReadyMessage {
    pub(crate) tilemap_entity: Entity,
    pub(crate) maze_size: usize,
}

#[derive(Copy, Clone)]
pub(super) enum MazeCellState {
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

fn create_maze(
    mut commands: Commands,
    maze_tile_image_assets: Res<MazeTileImageAssets>,
    mut generate_maze_message: MessageReader<GenerateMazeMessage>,
    mut maze_ready_message: MessageWriter<MazeReadyMessage>,
) {
    for maze_msg in generate_maze_message.read() {
        let maze_matrix = generate_maze_from_dimensions(maze_msg.size, maze_msg.seed);

        let maze_size = maze_msg.size as u32;

        let map_size = TilemapSize::new(maze_size, maze_size);

        let mut tile_storage = TileStorage::empty(map_size);

        let tilemap_entity = commands.spawn_empty().id();

        for y in 0..maze_size {
            for x in 0..maze_size {
                let tile_pos = TilePos::new(x, y);
                let cell = maze_matrix.get_cell(x as usize, y as usize);

                let image_index = match cell {
                    MazeCellState::Path => PATH_INDEX,

                    MazeCellState::Wall => WALL_INDEX,
                };

                let tp: Vec2 = tile_pos.into();

                let tile_entity = commands
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(image_index as u32),
                            ..default()
                        },
                        Name::new("Tile"),
                    ))
                    .id();

                if cell.is_wall() {
                    commands.entity(tile_entity).insert((
                        Transform::from_translation(tp.extend(0.0) * TILE_SIZE_F32),
                        Collider::cuboid(HALF_TILE_SIZE_F32, HALF_TILE_SIZE_F32),
                    ));
                }

                tile_storage.set(&tile_pos, tile_entity);
                commands.entity(tilemap_entity).add_child(tile_entity);
            }
        }

        let tile_size = TilemapTileSize::new(TILE_SIZE_F32, TILE_SIZE_F32);
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
                transform: Transform::from_xyz(0.0, 0.0, MAZE_Z_COORD),
                ..default()
            },
            maze_matrix,
            Name::new("TileMap"),
        ));

        maze_ready_message.write(MazeReadyMessage {
            tilemap_entity,
            maze_size: maze_size as usize,
        });
    }
}

fn generate_maze_from_dimensions(maze_size: usize, seed: u64) -> MazeMatrix {
    let mut maze_matrix = MazeMatrix::new(maze_size, seed);
    let mut visited = vec![false; maze_size * maze_size];

    let room_count = (maze_size - 1) / 2;

    let start = USizeVec2::new(
        (maze_matrix.rng.random_range(0..room_count) * 2) + 1,
        (maze_matrix.rng.random_range(0..room_count) * 2) + 1,
    );

    let mut stack = Vec::new();

    let start_index = maze_matrix.us_get_cell_index(start);
    visited[start_index] = true;
    maze_matrix.set_cell_at_index(start_index, MazeCellState::Path);

    stack.push(start);

    while let Some(&current) = stack.last() {
        let neighbors = get_rooms_over_offsets(current, maze_size, &visited);

        if neighbors.is_empty() {
            stack.pop();
            continue;
        }

        let (next_room, wall_between) = neighbors[maze_matrix.rng.random_range(0..neighbors.len())];

        let room_index = maze_matrix.us_get_cell_index(next_room);
        let wall_index = maze_matrix.us_get_cell_index(wall_between);

        visited[room_index] = true;

        maze_matrix.set_cell_at_index(room_index, MazeCellState::Path);
        maze_matrix.set_cell_at_index(wall_index, MazeCellState::Path);

        stack.push(next_room);
    }

    maze_matrix
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

        if visited[MazeMatrix::us_compute_index_with_size(room, maze_size)] {
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
