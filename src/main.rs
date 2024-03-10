use rand::Rng;
use std::io;
use std::collections::HashSet;

const SIZE: usize = 10;
const MINES: usize = 10;

#[derive(Clone, Copy)]
enum Cell {
    Mine,
    Number(u8),
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Open,
    Closed,
}

struct Game {
    board: [[Cell; SIZE]; SIZE],
    state: [[State; SIZE]; SIZE],
}

impl Game {
    fn new() -> Game {
        let mut game = Game {
            board: [[Cell::Empty; SIZE]; SIZE],
            state: [[State::Closed; SIZE]; SIZE],
        };

        let mut rng = rand::thread_rng();
        let mut mines_placed = 0;
        while mines_placed < MINES {
            let x = rng.gen_range(0..SIZE);
            let y = rng.gen_range(0..SIZE);
            if let Cell::Empty = game.board[x][y] {
                game.board[x][y] = Cell::Mine;
                mines_placed += 1;
            }
        }

        for x in 0..SIZE {
            for y in 0..SIZE {
                game.board[x][y] = match game.board[x][y] {
                    Cell::Mine => Cell::Mine,
                    _ => {
                        let mut count = 0;
                        for i in 0..3 {
                            for j in 0..3 {
                                if i == 1 && j == 1 { continue; }
                                let nx = x as isize + i - 1;
                                let ny = y as isize + j - 1;
                                if nx >= 0 && nx < SIZE as isize && ny >= 0 && ny < SIZE as isize {
                                    if let Cell::Mine = game.board[nx as usize][ny as usize] {
                                        count += 1;
                                    }
                                }
                            }
                        }
                        if count > 0 { Cell::Number(count) } else { Cell::Empty }
                    },
                };
            }
        }

        game
    }

    fn print(&self) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                match self.state[x][y] {
                    State::Closed => print!("■ "),
                    State::Open => match self.board[x][y] {
                        Cell::Mine => print!("* "),
                        Cell::Number(n) => print!("{} ", n),
                        Cell::Empty => print!(". "),
                    },
                }
            }
            println!("");
        }
    }

    fn open(&mut self, x: usize, y: usize) -> bool {
        if x >= SIZE || y >= SIZE {
            return false; // 追加: 範囲外の座標に対するエラーチェック
        }

        match self.board[x][y] {
            Cell::Mine => false,
            _ => {
                self.state[x][y] = State::Open;
                true
            },
        }
    }

    fn check_win(&self) -> bool {
        for x in 0..SIZE {
            for y in 0..SIZE {
                match self.board[x][y] {
                    Cell::Mine => continue,
                    _ => if self.state[x][y] == State::Closed { return false; },
                }
            }
        }
        true
    }
}

fn main() {
    let mut game = Game::new();
    loop {
        game.print();
        println!("Enter X and Y coordinates to open a cell (e.g., 3 4): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Please enter two numbers.");
            continue;
        }
        let x: usize = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            },
        };
        let y: usize = match parts[1].parse() {
            Ok(num
            )}
        }
    }

