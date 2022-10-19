use crate::board::{NUM_COLS, NUM_ROWS};

use core::fmt;
use std::str::FromStr;

pub struct UserInput {
    pub row: usize,
    pub column: usize,
}

pub enum UserInputError {
    Empty,
    UnparseableInput,
    OutOfBounds,
}

impl fmt::Display for UserInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.row, self.column)
    }
}

impl FromStr for UserInput {
    type Err = UserInputError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.chars().collect::<Vec<char>>()[..] {
            [] => Err(Self::Err::Empty),
            [row_char, column_char] => {
                let row = row_char as u8 as usize;
                let column = column_char as u8 as usize;
                if row >= NUM_ROWS || column >= NUM_COLS {
                    Err(Self::Err::OutOfBounds)
                } else {
                    Ok(UserInput { row, column })
                }
            },
            _ => Err(Self::Err::UnparseableInput),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bad_input() {
        let empty_input = "";

        let result = empty_input.parse::<UserInput>();
        assert!(matches!(result, Err(UserInputError::Empty)));

        let garbage_input = "sssss";
        let result = garbage_input.parse::<UserInput>();
        assert!(matches!(result, Err(UserInputError::UnparseableInput)));

        let out_of_bounds_input = "44";
        let result = out_of_bounds_input.parse::<UserInput>();
        assert!(matches!(result, Err(UserInputError::OutOfBounds)));
    }
}
