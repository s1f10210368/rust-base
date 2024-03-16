use rand::prelude::*; // Ensure rand crate is included in Cargo.toml
use std::collections::VecDeque;

fn main() {
    let width = 20;
    let height = 10;
    let maze = generate_maze(width, height);
    print_maze(&maze);
}

fn generate_maze(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut maze = vec![vec!['#'; width]; height];
    let mut stack = VecDeque::new();
    let mut rng = thread_rng(); // Ensure rand is in scope and Cargo.toml

    let start_pos = (1, 1);
    maze[start_pos.1][start_pos.0] = ' ';
    stack.push_back(start_pos);

    while let Some(pos) = stack.pop_back() {
        let mut directions = vec![(2, 0), (0, 2), (-2, 0), (0, -2)];
        directions.shuffle(&mut rng); // Make sure rand::seq::SliceRandom is in scope

        for &(dx, dy) in &directions {
            // Convert from isize back to usize carefully to avoid underflow
            let new_x = pos.0 as isize + dx;
            let new_y = pos.1 as isize + dy;

            if new_x > 0 && new_x < width as isize - 1 && new_y > 0 && new_y < height as isize - 1 {
                let ux = new_x as usize;
                let uy = new_y as usize;
                let mx = (pos.0 as isize + dx / 2) as usize;
                let my = (pos.1 as isize + dy / 2) as usize;

                if maze[uy][ux] == '#' {
                    maze[my][mx] = ' ';
                    maze[uy][ux] = ' ';
                    stack.push_back((ux, uy));
                }
            }
        }
    }
    maze
}

fn print_maze(maze: &Vec<Vec<char>>) {
    for row in maze {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
