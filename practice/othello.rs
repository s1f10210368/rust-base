const BOARD_SIZE: usize = 8;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Black,
    White,
}

impl Cell {
    fn opposite(self) -> Self {
        match self {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            Cell::Empty => Cell::Empty,
        }
    }
}

struct Board {
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        let mut cells = [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE];
        cells[3][3] = Cell::White;
        cells[4][4] = Cell::White;
        cells[3][4] = Cell::Black;
        cells[4][3] = Cell::Black;

        Board { cells }
    }

    fn print(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                print!("{} ", match cell {
                    Cell::Empty => ".",
                    Cell::Black => "B",
                    Cell::White => "W",
                });
            }
            println!();
        }
    }

    fn place_stone(&mut self, row: usize, col: usize, color: Cell) -> bool {
        if row >= BOARD_SIZE || col >= BOARD_SIZE || self.cells[row][col] != Cell::Empty {
            return false;
        }

        let mut flipped_any = false;
        for &(dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)] {
            let mut r = row as isize + dr;
            let mut c = col as isize + dc;

            let mut flipped = vec![];

            while r >= 0 && r < BOARD_SIZE as isize && c >= 0 && c < BOARD_SIZE as isize {
                match self.cells[r as usize][c as usize] {
                    cell if cell == color.opposite() => {
                        flipped.push((r as usize, c as usize));
                    },
                    cell if cell == color => {
                        for &(fr, fc) in &flipped {
                            self.cells[fr][fc] = color;
                        }
                        if !flipped.is_empty() {
                            flipped_any = true;
                        }
                        break;
                    },
                    _ => break,
                }

                r += dr;
                c += dc;
            }
        }

        if flipped_any {
            self.cells[row][col] = color;
        }

        flipped_any
    }
}

fn main() {
    let mut board = Board::new();
    board.print();

    // 例として(5, 4)に黒を置く
    if board.place_stone(5, 4, Cell::Black) {
        println!("Placed a stone at (5, 4)");
    } else {
        println!("Failed to place a stone at (5, 4)");
    }
    board.print();
}
