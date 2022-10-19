mod board;
mod board_has_win;
mod cell;
mod cell_iterator;
mod cells_are_valid;
mod column_iterator;
mod diagonal_iterator;
mod game;
mod player;
mod user_input;
mod row_iterator;

use crate::game::Game;

fn main() {
    Game::new().run();
}

