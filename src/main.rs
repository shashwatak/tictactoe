mod board;
mod board_has_win;
mod game;
mod cell;
mod cell_iterator;
mod cells_are_valid;
mod column_iterator;
mod diagonal_iterator;
mod player;
mod row_iterator;

use crate::game::Game;

fn main() {
    let game = Game::new().run();
}
