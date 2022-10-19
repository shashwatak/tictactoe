use core::fmt;
use std::str::FromStr;

use crate::cell::Cell;
use crate::cell::ParseCellError;
use crate::cells_are_valid::{cells_are_valid, CellsImpossibleError};

pub const NUM_COLS: usize = 3;
pub const NUM_ROWS: usize = 3;
pub const NUM_CELLS: usize = NUM_ROWS * NUM_COLS;

#[derive(Debug, Default)]
pub struct Board {
    // we will use iterators to turn cells into
    // coherent rows and columns
    pub cells: [Cell; NUM_CELLS],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self.cells.iter().map(|cell| cell.to_string()).collect();
        write!(f, "{}", out.join(""))
    }
}

#[derive(Debug)]
pub enum ParseBoardError {
    Empty,
    BadLen,
    BadChars(Vec<(usize, char)>),
    ImpossibleCells(CellsImpossibleError),
}

impl FromStr for Board {
    type Err = ParseBoardError;
    fn from_str(board_str: &str) -> Result<Self, Self::Err> {
        match board_str {
            "" => Err(Self::Err::Empty),
            g if g.len() != NUM_CELLS => Err(Self::Err::BadLen),
            _ => {
                let mut cells: [Cell; NUM_CELLS] = [Cell::Unmarked; NUM_CELLS];
                let mut char_errs: Vec<(usize, char)> = vec![];

                board_str.chars().enumerate().for_each(|(i, cell_char)| {
                    let cell_maybe = cell_char.to_string().parse::<Cell>();
                    match cell_maybe {
                        Ok(cell) => {
                            cells[i] = cell;
                        }
                        Err(ParseCellError::BadChar(c)) => char_errs.push((i, c)),
                        _ => panic!("unexpected unknown error"),
                    }
                });
                match &char_errs[..] {
                    [] => match cells_are_valid(&cells) {
                        Ok(_) => Ok(Board { cells }),
                        Err(err) => Err(Self::Err::ImpossibleCells(err)),
                    },
                    [..] => Err(Self::Err::BadChars(char_errs)),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bad_board_to_from_string() {
        {
            let board = "".to_string().parse::<Board>();
            assert!(matches!(board, Err(ParseBoardError::Empty)));
        }
        {
            let board = "X OOOXXO".to_string().parse::<Board>();
            assert!(matches!(board, Err(ParseBoardError::BadLen)));
        }
        {
            let board = "XXOOOXXXXO".to_string().parse::<Board>();
            assert!(matches!(board, Err(ParseBoardError::BadLen)));
        }
        {
            let board = "XXOOODXXO".to_string().parse::<Board>();
            match board {
                Err(ParseBoardError::BadChars(a)) => match &a[..] {
                    [(5, 'D')] => assert!(true),
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }
        {
            let board = "XXOOOXXOO".to_string().parse::<Board>();
            assert!(matches!(board, Err(ParseBoardError::ImpossibleCells(_))));
        }
    }

    #[test]
    fn test_board_to_from_string() {
        let good_boards = ["XOXXOXOXO"];
        for expected_board in good_boards {
            let board = expected_board.to_string().parse::<Board>();
            assert!(matches!(board, Ok(_)));
            let result_board = board.unwrap().to_string();
            assert_eq!(expected_board, result_board);
        }
    }
}
