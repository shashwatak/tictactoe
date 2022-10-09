mod cell;
mod game;
mod col_iter;
mod row_iter;

use crate::cell::Cell;
use crate::game::{Game, NUM_ROWS, NUM_COLS};
use crate::col_iter::ColIter;
use crate::row_iter::RowIter;

fn pretty_print(game: &Game) {
    println!("Row-wise");
    println!("   a b c ");
    println!("  -------");
    for row_idx in 0..NUM_ROWS {
        print!("{} |", row_idx + 1);
        for cell in RowIter::new(&game.cells, row_idx) {
            print!("{}|", cell);
        }
        println!("\n  -------");
    }

    println!("column-wise");
    println!("   1 2 3 ");
    println!("  -------");
    for col_idx in 0..NUM_COLS {
        print!("{} |", (col_idx + 'a' as usize) as u8 as char);
        for cell in ColIter::new(&game.cells, col_idx) {
            print!("{}|", cell);
        }
        println!("\n  -------");
    }

}

fn main() {
    let mut game = Game::default();
    game.cells[0] = Cell::X;
    game.cells[5] = Cell::O;
    game.cells[8] = Cell::X;
    pretty_print(&game);
}
