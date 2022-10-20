use crate::board::{Board, NUM_CELLS, NUM_COLS};
use crate::board_has_win::board_has_win;
use crate::cell::Cell;
use crate::player::Player;
use core::fmt;
use std::io::BufRead;
use crate::cell_id::CellId;
use crate::game_update::{check_cell, next_player};

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub player: Player,
    num_cells_played: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::default(),
            player: Player::X,
            num_cells_played: 0,
        }
    }

    pub fn run<Input: BufRead>(&self, f: &mut Input) {
        loop {
            print!("{}", self);
            let winner = board_has_win(&self.board);
            if let Cell::Player(p) = winner {
                println!("Winner! {}", p);
                break;
            }
            if self.num_cells_played >= NUM_CELLS {
                println!("Draw.");
                break;
            }

            let mut line = String::new();
            let input = f.read_line(&mut line).unwrap();
            println!("you said: {}", input);
        }
    }
    pub fn update(&mut self, cell_id: CellId) {
        self.board.cells[cell_id.to_idx()] = Cell::Player(self.player);
        self.player = next_player(self.player);
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out: Vec<String> = Vec::new();
        out.push("  ".to_string());
        for column_idx in 0..NUM_COLS {
            out.push(format!(" {}", (column_idx + 'a' as usize) as u8 as char));
        }

        out.push("\n  -------\n".to_string());
        for (row_idx, row) in self.board.rows().enumerate() {
            out.push(format!("{} |", row_idx + 1));
            for cell in row {
                out.push(format!("{}|", cell));
            }
            out.push("\n  -------\n".to_string());
        }

        out.push(format!("Next Turn: {}\n", self.player));
        write!(f, "{}", out.join(""))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game_update() {

    }
}
