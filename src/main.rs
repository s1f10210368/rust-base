use rand::prelude::*;
use std::collections::VecDeque;

fn main() {
    let (width, height) = (20, 10);
    let maze = generate_maze(width, height);
    print_maze(&maze);
}

fn generate_maze(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut maze = vec![vec!['#'; width]; height];
    let mut stack = VecDeque::new();
    let mut rng = thread_rng();

    let start_pos = (1, 1);
    maze[start_pos.1][start_pos.0] = ' ';
    stack.push_back(start_pos);

    while let Some((x, y)) = stack.pop_back() {
        let directions = [(2, 0), (0, 2), (-2, 0), (0, -2)];
        for &(dx, dy) in directions.choose_multiple(&mut rng, directions.len()) {
            let (new_x, new_y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if new_x > 0 && new_x < width - 1 && new_y > 0 && new_y < height - 1 && maze[new_y][new_x] == '#' {
                maze[y + dy as usize / 2][x + dx as usize / 2] = ' ';
                maze[new_y][new_x] = ' ';
                stack.push_back((new_x, new_y));
            }
        }
    }

    maze
}

fn print_maze(maze: &[Vec<char>]) {
    maze.iter().for_each(|row| {
        row.iter().for_each(|&cell| print!("{}", cell));
        println!();
    });
}
