mod board;
mod cells_are_valid;
mod board_has_win;
mod cell;
mod cell_iterator;
mod column_iterator;
mod row_iterator;
mod diagonal_iterator;

use crate::board::{Board, NUM_COLS, NUM_ROWS};
use crate::board_has_win::board_has_win;

fn pretty_print(board: &Board) {
    println!("Row Major:");
    print!("  ");
    for column_idx in 0..NUM_COLS {
        print!(" {}", (column_idx + 'a' as usize) as u8 as char);
    }
    println!("\n  -------");
    for (row_idx, row) in board.rows().enumerate() {
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
    for (column_idx, column) in board.columns().enumerate() {
        print!("{} |", (column_idx + 'a' as usize) as u8 as char);
        for cell in column {
            print!("{}|", cell);
        }
        println!("\n  -------");
    }
    println!("is win? {}", board_has_win(board));
}

fn main() {
    let board = "X O XOO X".to_string().parse::<Board>().unwrap();
    pretty_print(&board);
}
