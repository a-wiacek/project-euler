use cached::proc_macro::cached;
// Board is encoded using 9 * 12 initial bits in u128

const X: u32 = 9;
const Y: u32 = 12;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Board(u128);
const INIT_BOARD: Board = Board(0);
const FINAL_BOARD: Board = Board((1u128 << (X * Y)) - 1);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell(u32);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Triomino(Cell, Cell, Cell);

fn triominos_on_position(cell_safe: Cell) -> Vec<Triomino> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct CellUnsafe(i32);
    fn x_plus(cell: CellUnsafe) -> CellUnsafe {
        CellUnsafe(cell.0 + 1)
    }
    fn x_minus(cell: CellUnsafe) -> CellUnsafe {
        CellUnsafe(cell.0 - 1)
    }
    fn y_plus(cell: CellUnsafe) -> CellUnsafe {
        CellUnsafe(cell.0 + X as i32)
    }
    fn y_minus(cell: CellUnsafe) -> CellUnsafe {
        CellUnsafe(cell.0 - X as i32)
    }
    fn sanitize(u: CellUnsafe) -> Option<Cell> {
        if u.0 >= 0 && u.0 < (X * Y) as i32 {
            Some(Cell(u.0 as u32))
        } else {
            None
        }
    }

    let cell = CellUnsafe(cell_safe.0 as i32);

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct TriominoUnsafe(CellUnsafe, CellUnsafe, CellUnsafe);
    fn sanitize_t(tu: TriominoUnsafe) -> Option<Triomino> {
        if let Some(c0) = sanitize(tu.0) {
            if let Some(c1) = sanitize(tu.1) {
                if let Some(c2) = sanitize(tu.2) {
                    return Some(Triomino(c0, c1, c2));
                }
            }
        }
        None
    }

    fn triomino_connected(&t: &Triomino) -> bool {
        let cells_connected = |cell1: Cell, cell2: Cell| {
            let min = std::cmp::min(cell1.0, cell2.0);
            let max = std::cmp::max(cell1.0, cell2.0);
            let horizontal_conn = max - min == 1 && max % X != 0;
            let vertical_conn = max - min == X;
            horizontal_conn || vertical_conn
        };
        cells_connected(t.0, t.1) && cells_connected(t.1, t.2)
    }

    vec![
        // 3 x 1 triominos
        TriominoUnsafe(x_plus(x_plus(cell)), x_plus(cell), cell),
        TriominoUnsafe(x_plus(cell), cell, x_minus(cell)),
        TriominoUnsafe(cell, x_minus(cell), x_minus(x_minus(cell))),
        TriominoUnsafe(y_plus(y_plus(cell)), y_plus(cell), cell),
        TriominoUnsafe(y_plus(cell), cell, y_minus(cell)),
        TriominoUnsafe(cell, y_minus(cell), y_minus(y_minus(cell))),
        // L-triominoes, cell is in the center
        TriominoUnsafe(x_plus(cell), cell, y_minus(cell)),
        TriominoUnsafe(x_plus(cell), cell, y_plus(cell)),
        TriominoUnsafe(x_minus(cell), cell, y_minus(cell)),
        TriominoUnsafe(x_minus(cell), cell, y_plus(cell)),
        // L-triominoes, cell is not in the center
        TriominoUnsafe(cell, x_plus(cell), y_plus(x_plus(cell))),
        TriominoUnsafe(cell, x_plus(cell), y_minus(x_plus(cell))),
        TriominoUnsafe(cell, x_minus(cell), y_minus(x_minus(cell))),
        TriominoUnsafe(cell, x_minus(cell), y_plus(x_minus(cell))),
        TriominoUnsafe(cell, y_plus(cell), x_minus(y_plus(cell))),
        TriominoUnsafe(cell, y_plus(cell), x_plus(y_plus(cell))),
        TriominoUnsafe(cell, y_minus(cell), x_minus(y_minus(cell))),
        TriominoUnsafe(cell, y_minus(cell), x_plus(y_minus(cell))),
    ]
    .into_iter()
    .flat_map(sanitize_t)
    .filter(triomino_connected)
    .collect()
}

fn put_triomino_on_board(triomino: Triomino, board: Board) -> Option<Board> {
    let fill = |board: Board, cell: Cell| {
        let cell_lifted = 1 << cell.0;
        if board.0 & cell_lifted == 0 {
            Some(Board(board.0 | cell_lifted))
        } else {
            None
        }
    };
    Some(board)
        .and_then(|board| fill(board, triomino.0))
        .and_then(|board| fill(board, triomino.1))
        .and_then(|board| fill(board, triomino.2))
}

#[cached]
fn count_possibilities(board: Board) -> u64 {
    if board == FINAL_BOARD {
        1
    } else {
        let free_pos = Cell((!board.0).trailing_zeros());
        triominos_on_position(free_pos)
            .into_iter()
            .filter_map(|triomino| put_triomino_on_board(triomino, board))
            .map(count_possibilities)
            .sum()
    }
}

pub fn euler161() -> String {
    count_possibilities(INIT_BOARD).to_string()
}
