use core::fmt;
use std::str::FromStr;

use crate::cell::Cell;
use crate::cell::ParseCellError;

pub const NUM_COLS: usize = 3;
pub const NUM_ROWS: usize = 3;
pub const NUM_CELLS: usize = NUM_ROWS * NUM_COLS;

#[derive(Debug, Default)]
pub struct Game {
    // we will use iterators to turn cells into
    // coherent rows and columns
    pub cells: [Cell; NUM_CELLS],
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self.cells.iter().map(|cell| cell.to_string()).collect();
        write!(f, "{}", out.join(""))
    }
}

#[derive(Debug)]
pub enum ParseGameError {
    Empty,
    BadLen,
    BadChars(Vec<(usize, char)>),
}

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(game_str: &str) -> Result<Self, Self::Err> {
        match game_str {
            "" => Err(Self::Err::Empty),
            g if g.len() != NUM_CELLS => Err(Self::Err::BadLen),
            _ => {
                let mut game = Game::default();
                let mut errs: Vec<(usize, char)> = vec![];

                game_str.chars().enumerate().for_each(|(i, cell_char)| {
                    let cell_maybe = cell_char.to_string().parse::<Cell>();
                    match cell_maybe {
                        Ok(cell) => game.cells[i] = cell,
                        Err(ParseCellError::BadChar(c)) => errs.push((i, c)),
                        _ => panic!("unexpected unknown error"),
                    }
                });

                match &errs[..] {
                    [] => Ok(game),
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
    fn test_bad_game_to_from_string() {
        {
            let game = "".to_string().parse::<Game>();
            assert!(matches!(game, Err(ParseGameError::Empty)));
        }
        {
            let game = "X OOOXXO".to_string().parse::<Game>();
            assert!(matches!(game, Err(ParseGameError::BadLen)));
        }
        {
            let game = "XXOOOXXXXO".to_string().parse::<Game>();
            assert!(matches!(game, Err(ParseGameError::BadLen)));
        }
        {
            let game = "XXOOODXXO".to_string().parse::<Game>();
            match game {
                Err(ParseGameError::BadChars(a)) => match &a[..] {
                    [(5, 'D')] => assert!(true),
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_game_to_from_string() {
        let good_games = ["XOXOXOOXO"];
        for expected_game in good_games {
            let game = expected_game.to_string().parse::<Game>();
            assert!(matches!(game, Ok(_)));
            let result_game = game.unwrap().to_string();
            assert_eq!(expected_game, result_game);
        }
    }
}