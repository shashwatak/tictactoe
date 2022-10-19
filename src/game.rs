use crate::board::{Board, NUM_COLS};
use crate::player::Player;
use core::fmt;

#[derive(Debug)]
pub struct Game {
    board: Board,
    player: Player,
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

    pub fn print(&self) {
        print!("  ");
        for column_idx in 0..NUM_COLS {
            print!(" {}", (column_idx + 'a' as usize) as u8 as char);
        }
        println!("\n  -------");
        for (row_idx, row) in self.board.rows().enumerate() {
            print!("{} |", row_idx + 1);
            for cell in row {
                print!("{}|", cell);
            }
            println!("\n  -------");
        }
    }
}



impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out : Vec<String> = Vec::new(); 
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
        write!(f, "{}", out.join(""))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game() {
        let game = Game::new();
        print!("{}", game);
    }
}
