use crate::utils::input::get_input;

struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    fn is_group_valid(group: impl Iterator<Item = u8>) -> bool {
        let mut digits = vec![false; 9];
        for digit in group {
            if digit > 0 {
                let d = digit as usize - 1;
                if digits[d] {
                    return false;
                } else {
                    digits[d] = true;
                }
            }
        }
        true
    }

    fn answer(&self) -> usize {
        (self.board[0][0] as usize) * 100
            + (self.board[0][1] as usize) * 10
            + (self.board[0][2] as usize)
    }

    fn update(&mut self, pos: (usize, usize), d: u8) {
        self.board[pos.0][pos.1] = d;
    }

    fn lookup(&self, pos: (usize, usize)) -> u8 {
        self.board[pos.0][pos.1]
    }

    fn is_valid(&self) -> bool {
        // Rows
        for i in 0..9 {
            if !Sudoku::is_group_valid((0..9).map(|j| self.board[i][j])) {
                return false;
            }
        }

        // Columns
        for i in 0..9 {
            if !Sudoku::is_group_valid((0..9).map(|j| self.board[j][i])) {
                return false;
            }
        }

        // Small squares
        for i in 0..3 {
            for j in 0..3 {
                let mut digits = Vec::new();
                for k in 0..3 {
                    for l in 0..3 {
                        digits.push(self.board[3 * i + k][3 * j + l]);
                    }
                }
                if !Sudoku::is_group_valid(digits.into_iter()) {
                    return false;
                }
            }
        }

        true
    }

    fn next_position((y, x): (usize, usize)) -> Option<(usize, usize)> {
        if x == 8 {
            if y == 8 {
                None
            } else {
                Some((y + 1, 0))
            }
        } else {
            Some((y, x + 1))
        }
    }

    fn solve_at(&mut self, pos: Option<(usize, usize)>) -> bool {
        match pos {
            None => true,
            Some(pos) => {
                let next_pos = Sudoku::next_position(pos);
                if self.lookup(pos) == 0 {
                    for d in 1..=9 {
                        self.update(pos, d);
                        if self.is_valid() {
                            if self.solve_at(next_pos) {
                                return true;
                            }
                        }
                    }
                    self.update(pos, 0);
                    false
                } else {
                    self.solve_at(next_pos)
                }
            }
        }
    }

    // Try to solve the sudoku.
    // If it has a solution, the sudoku is modified and the function returns true.
    // If not, the sudoku remains unchanged and the function returns true.
    fn solve(&mut self) -> bool {
        self.solve_at(Some((0, 0)))
    }
}

fn parse_sudoku(sudoku: &[&str]) -> Sudoku {
    let mut board = [[0u8; 9]; 9];
    for i in 0..9 {
        let digits = sudoku[i + 1]
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        for j in 0..9 {
            board[i][j] = digits[j];
        }
    }
    Sudoku { board }
}

pub fn euler096() -> String {
    get_input(96)
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(10) // grid number + 9 lines of sudoku
        .map(parse_sudoku)
        .map(|mut sudoku| {
            sudoku.solve();
            sudoku.answer()
        })
        .sum::<usize>()
        .to_string()
}
