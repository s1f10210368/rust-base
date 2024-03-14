use rand::prelude::*;
use std::collections::VecDeque;

fn main() {
    let width = 20;
    let height = 10;
    let mut maze = generate_maze(width, height);
    print_maze(&maze);
}

fn generate_maze(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut maze = vec![vec!['#'; width]; height];
    let mut stack = VecDeque::new();
    let mut rng = thread_rng();

    let start_pos = (1, 1);
    maze[start_pos.1][start_pos.0] = ' ';
    stack.push_back(start_pos);

    while let Some(pos) = stack.pop_back() {
        let mut directions = vec![(2, 0), (0, 2), (-2, 0), (0, -2)];
        directions.shuffle(&mut rng);

        for &(dx, dy) in &directions {
            let new_x = (pos.0 as isize + dx) as usize;
            let new_y = (pos.1 as isize + dy) as usize;
            if new_x > 0 && new_x < width - 1 && new_y > 0 && new_y < height - 1 && maze[new_y][new_x] == '#' {
                maze[(pos.1 as isize + dy / 2) as usize][(pos.0 as isize + dx / 2) as usize] = ' ';
                maze[new_y][new_x] = ' ';
                stack.push_back((new_x, new_y));
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
        println!("");
    }
}