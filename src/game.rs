use crate::board::{Board, NUM_CELLS, NUM_COLS};
use crate::board_has_win::board_has_win;
use crate::cell::Cell;
use crate::player::Player;
use crate::cell_id::CellId;
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

    pub fn run(&self) {
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
            let input = std::io::stdin().read_line(&mut line).unwrap();
            println!("you said: {}", input);
        }
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

#[derive(Debug)]
enum GameUpdateError {
    CellIsFull,
}

impl Game {
    fn update(&mut self, cell_id: CellId) -> Result<(), GameUpdateError> {
        let cell_idx = cell_id.to_idx();
        if let Cell::Player(_) = self.board.cells[cell_idx] {
            return Err(GameUpdateError::CellIsFull);
        }
        self.board.cells[cell_idx] = Cell::Player(self.player);
        if let Player::X = self.player {
            self.player = Player::O;
        } else if let Player::O = self.player {
            self.player = Player::X
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game() {
        let mut game = Game::new();
        print!("{}", game);

        let cell_id = "a1".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));

        let cell_id = "a1".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Err(GameUpdateError::CellIsFull)));

        let cell_id = "a2".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));

        
        let cell_id = "a3".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));

        let cell_id = "b2".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));


        let cell_id = "b1".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));

        let cell_id = "c1".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));


        let cell_id = "c2".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));


        let cell_id = "b3".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));

        let cell_id = "c3".to_string().parse::<CellId>().unwrap();
        let result = game.update(cell_id);
        assert!(matches!(result, Ok(_)));

        println!("{}", game);
    }
}
