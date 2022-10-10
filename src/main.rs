mod cell;
mod board;
mod col_iter;
mod row_iter;

use crate::cell::Cell;
use crate::board::{Board, NUM_ROWS, NUM_COLS};
use crate::col_iter::ColIter;
use crate::row_iter::RowIter;

fn pretty_print(board: &Board) {
    println!("Row Major:");
    print!("  ");
    for col_idx in 0..NUM_COLS {
        print!(" {}", (col_idx + 'a' as usize) as u8 as char);
    }
    println!("\n  -------");
    for row_idx in 0..NUM_ROWS {
        print!("{} |", row_idx + 1);
        for cell in RowIter::new(&board.cells, row_idx) {
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
        for cell in ColIter::new(&board.cells, col_idx) {
            print!("{}|", cell);
        }
        println!("\n  -------");
    }

}

fn main() {
    let mut board = Board::default();
    board.cells[0] = Cell::X;
    board.cells[5] = Cell::O;
    board.cells[8] = Cell::X;
    pretty_print(&board);
}
