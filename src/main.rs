mod board;
mod board_has_win;
mod cell;
mod cell_id;
mod cell_iterator;
mod cells_are_valid;
mod column_iterator;
mod diagonal_iterator;
mod game;
mod game_update;
mod player;
mod row_iterator;

use crate::game::Game;

fn main() {
    Game::new().run(&mut std::io::stdin().lock());
}
