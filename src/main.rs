use std::io;
use rand::Rng;
use std::collections::HashSet;

const SIZE: usize = 10; // ゲームボードのサイズ
const MINES: usize = 10; // 地雷の数

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

        // 地雷をランダムに配置
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

        // 各セルの隣接地雷数を計算
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

    // ゲームの状態を表示するメソッドなど、追加の機能をここに実装
}

fn main() {
    let game = Game::new();
    // ゲームループ、ユーザー入力の処理、ゲーム状態の表示などをここに実装
}
