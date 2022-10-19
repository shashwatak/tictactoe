use crate::board::{NUM_COLS, NUM_ROWS};

use core::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct CellId {
    pub row: usize,
    pub column: usize,
}

impl CellId {
    pub fn to_idx(&self) -> usize {
        self.row * NUM_COLS + self.column
    }
}

#[derive(Debug)]
pub enum CellIdError {
    Empty,
    UnparseableInput,
    OutOfBounds,
}

impl fmt::Display for CellId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.row, self.column)
    }
}

const MIN_ROW_CHAR: char = '1';
const MAX_ROW_CHAR: char = ('1' as u8 + (NUM_ROWS - 1) as u8) as char;

const MIN_COLUMN_CHAR: char = 'a';
const MAX_COLUMN_CHAR: char = ('a' as u8 + (NUM_COLS - 1) as u8) as char;

impl FromStr for CellId {
    type Err = CellIdError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.chars().collect::<Vec<char>>()[..] {
            [] => Err(Self::Err::Empty),
            [column_char, row_char] => {
                println!("debug {} {}", row_char, column_char);
                if row_char >= MIN_ROW_CHAR
                    && row_char <= MAX_ROW_CHAR
                    && column_char >= MIN_COLUMN_CHAR
                    && column_char <= MAX_COLUMN_CHAR
                {
                    let row = row_char as usize - '1' as usize;
                    let column = column_char as usize - 'a' as usize;
                    Ok(CellId { row, column })
                } else {
                    Err(Self::Err::OutOfBounds)
                }
            }
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

        let result = empty_input.parse::<CellId>();
        assert!(matches!(result, Err(CellIdError::Empty)));

        let garbage_input = "sssss";
        let result = garbage_input.parse::<CellId>();
        assert!(matches!(result, Err(CellIdError::UnparseableInput)));

        let out_of_bounds_inputs = ["a0", "a4", "d1", "d4"];
        for input in out_of_bounds_inputs {
            let result = input.parse::<CellId>();
            assert!(matches!(result, Err(CellIdError::OutOfBounds)));
        }
    }

    #[test]
    fn test_good_inputs() {
        let input = "a1";
        let result: CellId = input.parse().unwrap();
        assert_eq!(result.row, 0);
        assert_eq!(result.column, 0);
        let input = "a3";
        let result: CellId = input.parse().unwrap();
        assert_eq!(result.row, 2);
        assert_eq!(result.column, 0);
        let input = "b2";
        let result: CellId = input.parse().unwrap();
        assert_eq!(result.row, 1);
        assert_eq!(result.column, 1);
        let input = "c1";
        let result: CellId = input.parse().unwrap();
        assert_eq!(result.row, 0);
        assert_eq!(result.column, 2);
    }

    #[test]
    fn test_to_idx() {
        let cell_idx = "a1".to_string().parse::<CellId>().unwrap().to_idx();
        assert_eq!(cell_idx, 0);
        let cell_idx = "a3".to_string().parse::<CellId>().unwrap().to_idx();
        assert_eq!(cell_idx, 6);
        let cell_idx = "b2".to_string().parse::<CellId>().unwrap().to_idx();
        assert_eq!(cell_idx, 4);
        let cell_idx = "c1".to_string().parse::<CellId>().unwrap().to_idx();
        assert_eq!(cell_idx, 2);
        let cell_idx = "c3".to_string().parse::<CellId>().unwrap().to_idx();
        assert_eq!(cell_idx, 8);
    }
}
