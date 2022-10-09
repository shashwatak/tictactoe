mod cell;
mod game;
mod row_iter;

use crate::cell::Cell;
use crate::game::{Game, NUM_ROWS};
use crate::row_iter::RowIter;

fn pretty_print(game: &Game) {
    println!("   a b c ");
    println!("  -------");
    for row_idx in 0..NUM_ROWS {
        print!("{} |", row_idx + 1);
        for cell in RowIter::new(&game.cells, row_idx) {
            print!("{}|", cell);
        }
        println!("\n  -------");
    }
}

fn main() {
    let mut game = Game::default();
    game.cells[0] = Cell::X;
    game.cells[4] = Cell::O;
    game.cells[8] = Cell::X;
    pretty_print(&game);
}
