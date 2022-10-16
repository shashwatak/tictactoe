mod board;
mod board_has_win;
mod cell;
mod cell_iterator;
mod column_iterator;
mod row_iterator;

use crate::board::{Board, NUM_COLS, NUM_ROWS};
use crate::board_has_win::board_has_win;
use crate::cell::Cell;
use crate::column_iterator::ColumnIterator;
use crate::row_iterator::RowIterator;

fn pretty_print(board: &Board) {
    println!("Row Major:");
    print!("  ");
    for column_idx in 0..NUM_COLS {
        print!(" {}", (column_idx + 'a' as usize) as u8 as char);
    }
    println!("\n  -------");
    for (row_idx, row) in RowIterator::new(&board.cells).enumerate() {
        print!("{} |", row_idx + 1);
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
    for (column_idx, column) in ColumnIterator::new(&board.cells).enumerate() {
        print!("{} |", (column_idx + 'a' as usize) as u8 as char);
        for cell in column {
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
