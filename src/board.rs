use core::fmt;
use std::str::FromStr;

use crate::cell::Cell;
use crate::cell::ParseCellError;

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
    TooManyXs,
    TooManyOs,
}

impl FromStr for Board {
    type Err = ParseBoardError;
    fn from_str(board_str: &str) -> Result<Self, Self::Err> {
        match board_str {
            "" => Err(Self::Err::Empty),
            g if g.len() != NUM_CELLS => Err(Self::Err::BadLen),
            _ => {
                let mut board = Board::default();
                let mut xs: Vec<usize> = vec!();
                let mut os: Vec<usize> = vec!(); 
                let mut errs: Vec<(usize, char)> = vec![];

                board_str.chars().enumerate().for_each(|(i, cell_char)| {
                    let cell_maybe = cell_char.to_string().parse::<Cell>();
                    match cell_maybe {
                        Ok(cell) => {
                            board.cells[i] = cell;
                            match cell {
                                Cell::X => xs.push(i),
                                Cell::O => os.push(i),
                                _ => (),
                            }
                        },
                        Err(ParseCellError::BadChar(c)) => errs.push((i, c)),
                        _ => panic!("unexpected unknown error"),
                    }
                });
                if xs.len() > os.len() + 1 {
                     return Err(Self::Err::TooManyXs);
                } else if os.len() > xs.len() {
                     return Err(Self::Err::TooManyOs);
                }
                match &errs[..] {
                    [] => Ok(board),
                    [..] => Err(Self::Err::BadChars(errs)),
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
            let board = "XXOOOXXXX".to_string().parse::<Board>();
            assert!(matches!(board, Err(ParseBoardError::TooManyXs)));
        }
        {
            let board = "XXOOOXXOO".to_string().parse::<Board>();
            assert!(matches!(board, Err(ParseBoardError::TooManyOs)));
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
