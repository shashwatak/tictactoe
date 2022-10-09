mod cell;

use core::fmt;
use std::str::FromStr;
use crate::cell::Cell;
use crate::cell::ParseCellError;

const NUM_COLS: usize = 3;
const NUM_ROWS: usize = 3;
const NUM_CELLS: usize = NUM_ROWS * NUM_COLS;

#[derive(Debug, Default)]
struct Game {
    // we will use iterators to turn cells into
    // coherent rows and columns
    cells: [Cell; NUM_CELLS],
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self.cells.iter().map(|cell| cell.to_string()).collect();
        write!(f, "{}", out.join(""))
    }
}

impl Game {
    fn pretty_print(&self) {
        println!("   a b c ");
        println!("  -------");
        for row_idx in 0..NUM_ROWS {
            print!("{} |", row_idx + 1);
            for cell in Row::new(&self.cells, row_idx) {
                print!("{}|", cell);
            }
            println!("\n  -------");
        }
    }
}

#[derive(Debug)]
enum ParseGameError {
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

struct Row<'a> {
    row_idx: usize,
    count: usize,
    cells: &'a [Cell; NUM_CELLS],
}

impl<'a> Row<'a> {
    fn new(cells: &'a [Cell; NUM_CELLS], row_idx: usize) -> Row<'a> {
        Row {
            row_idx,
            count: 0,
            cells,
        }
    }
}

impl<'a> Iterator for Row<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let start_idx = self.row_idx * NUM_COLS;
        let max_idx = start_idx + NUM_COLS;
        let idx = start_idx + self.count;
        match idx {
            idx if idx < max_idx => {
                let result = &self.cells[start_idx + self.count];
                self.count += 1;
                Some(result)
            }
            _ => None,
        }
    }
}

fn main() {
    let mut game = Game::default();
    game.cells[0] = Cell::X;
    game.cells[4] = Cell::O;
    game.cells[8] = Cell::X;
    game.pretty_print();
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
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

    #[test]
    fn test_row_iter() {
        let game = "XOXOXOOXO".to_string().parse::<Game>().unwrap();
        let mut row = Row::new(&game.cells, 0);
        println!("aaa {:?}", row.next());
        println!("aaa {:?}", row.next());
        println!("aaa {:?}", row.next());
        println!("aaa {:?}", row.next());
        let mut row = Row::new(&game.cells, 0);
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next().unwrap(), Cell::O));
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next(), None));
    }
}
