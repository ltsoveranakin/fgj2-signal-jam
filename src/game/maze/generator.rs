use bevy::math::USizeVec2;
use bevy::prelude::*;
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};
use smallvec::SmallVec;
use std::collections::LinkedList;

pub(super) struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GenerateMazeMessage>()
            .add_systems(Startup, generate_maze);
    }
}

#[derive(Message)]
pub(crate) struct GenerateMazeMessage(usize);

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

// https://www.youtube.com/watch?v=TaQly10bDME
fn generate_maze(mut generate_maze_message: MessageReader<GenerateMazeMessage>) {
    for maze_msg in generate_maze_message.read() {
        let mut rng = SmallRng::seed_from_u64(10);
        let maze_size = maze_msg.0;
        let half_maze_size = maze_size / 2;
        let mut maze_matrix = vec![MazeCellState::Wall; maze_size * maze_size];
        let mut visited = vec![false; maze_size * maze_size];

        for y in 0..maze_size {
            if y % 2 == 1 {
                continue;
            }

            for x in 0..maze_size {
                if x % 2 == 1 {
                    continue;
                }

                maze_matrix[get_cell_index(x, y, maze_size)] = MazeCellState::Path;
            }
        }

        print_maze(&maze_matrix, maze_size);

        let mut stack = LinkedList::new();

        let start = USizeVec2::new(
            rng.random_range(0..half_maze_size) * 2,
            rng.random_range(0..half_maze_size) * 2,
        );

        stack.push_back(start);

        while let Some(cell) = stack.pop_front() {
            let room_offsets = get_rooms_over_offsets(cell, maze_size, &visited);

            if room_offsets.is_empty() {
            } else {
                let (next_room, next_wall) = room_offsets[rng.random_range(0..room_offsets.len())];

                maze_matrix[us_get_cell_index(next_wall, maze_size)] = MazeCellState::Path;

                visited[us_get_cell_index(next_room, maze_size)] = true;
                stack.push_back(next_room);
            }
        }

        // while let Some(cell) = queue.pop_front() {}

        print_maze(&maze_matrix, maze_size);

        // }
    }
}

fn us_get_cell_index(usize_vec2: USizeVec2, size: usize) -> usize {
    get_cell_index(usize_vec2.x, usize_vec2.y, size)
}

fn get_cell_index(x: usize, y: usize, size: usize) -> usize {
    x + (y * size)
}

// const fn get_doubled(mut offsets: [IVec2; 4]) -> [IVec2; 4] {
//     let mut i = 0;
//
//     while i <  offsets.len() {
//         offsets[i].x *= 2;
//         offsets[i].y *= 2;
//
//         i+= 1;
//     }
//
//     offsets
// }

fn get_rooms_over_offsets(
    pos: USizeVec2,
    size_u: usize,
    visited: &[bool],
) -> SmallVec<[(USizeVec2, USizeVec2); 4]> {
    const OFFSETS: [IVec2; 4] = [
        IVec2::new(0, 1),
        IVec2::new(1, 0),
        IVec2::new(0, -1),
        IVec2::new(-1, 0),
    ];

    let size = size_u as i32;
    let pos = pos.as_ivec2();

    let mut valid_offsets = SmallVec::<_>::new();

    for offset in OFFSETS {
        let new_pos = pos + (offset * 2);

        if new_pos.x >= size || new_pos.y >= size || new_pos.x < 0 || new_pos.y < 0 {
            continue;
        }

        let new_pos = new_pos.as_usizevec2();

        if visited[us_get_cell_index(new_pos, size_u)] {
            continue;
        }

        let wall_between = (pos + offset).as_usizevec2();

        valid_offsets.push((new_pos, wall_between));
    }

    valid_offsets
}

// fn get_valid_adj_cells(pos: USizeVec2, size: i32, rng: &mut SmallRng) -> SmallVec<[USizeVec2; 4]> {
//     let mut cells = SmallVec::new();
//
//     for x in -1..=1 {
//         for y in -1..=1 {
//             if (x == 0 && y == 0) || (x != 0 && y != 0) {
//                 continue;
//             }
//
//             let new_pos = pos.as_ivec2() + IVec2::new(x, y);
//
//             if new_pos.x >= size || new_pos.y >= size || new_pos.x < 0 || new_pos.y < 0 {
//                 continue;
//             }
//
//             cells.push(new_pos.as_uvec2());
//         }
//     }
//
//     cells.shuffle(rng);
//
//     cells
// }

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
