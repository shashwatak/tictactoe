use crate::player::Player;
use crate::board::{NUM_ROWS, NUM_COLS};


pub struct UserInput {
    pub player: Player,
    pub row: usize,
    pub column: usize,
}

pub enum UserInputError {
    OutOfBounds,
    CellAlreadyMarked,
    UnparseableInput,
}

