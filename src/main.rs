mod board;
mod board_has_win;
mod cell_iterator;
mod cell;

use crate::board::{Board, NUM_COLS, NUM_ROWS};
use crate::board_has_win::board_has_win;
use crate::cell_iterator::{next_col_cell_idx, next_row_cell_idx, CellIterator};
use crate::cell::Cell;

fn pretty_print(board: &Board) {
    println!("Row Major:");
    print!("  ");
    for col_idx in 0..NUM_COLS {
        print!(" {}", (col_idx + 'a' as usize) as u8 as char);
    }
    println!("\n  -------");
    for row_idx in 0..NUM_ROWS {
        print!("{} |", row_idx + 1);
        let row = CellIterator::new(&board.cells, row_idx, next_row_cell_idx);
        for cell in row {
            print!("{}|", cell);
        }
        println!("\n  -------");
    }

    println!("Column Major:");
    print!("  ");
    for row_idx in 0..NUM_ROWS {
        print!(" {}", row_idx + 1);
    }
    println!("\n  -------");
    for col_idx in 0..NUM_COLS {
        print!("{} |", (col_idx + 'a' as usize) as u8 as char);
        let col = CellIterator::new(&board.cells, col_idx, next_col_cell_idx);
        for cell in col {
            print!("{}|", cell);
        }
        println!("\n  -------");
    }
    println!("is win? {}", board_has_win(board));
}

fn main() {
    let mut board = Board::default();
    board.cells[0] = Cell::X;
    board.cells[5] = Cell::O;
    board.cells[8] = Cell::X;
    pretty_print(&board);
}
