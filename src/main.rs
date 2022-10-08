use core::fmt;
use std::str::FromStr;

#[derive(Debug, Default)]
enum Cell {
    #[default]
    Unmarked,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
            Cell::Unmarked => write!(f, " "),
        }
    }
}

#[derive(Debug)]
enum ParseCellError {
    Empty,
    BadLen,
    BadChar(char),
}

impl FromStr for Cell {
    type Err = ParseCellError;
    fn from_str(cell_str: &str) -> Result<Self, Self::Err> {
        match cell_str {
            "" => Err(Self::Err::Empty),
            c if c.len() >= 2 => Err(Self::Err::BadLen),
            "X" => Ok(Cell::X),
            "O" => Ok(Cell::O),
            " " => Ok(Cell::Unmarked),
            c => Err(Self::Err::BadChar(c.chars().next().unwrap())),
        }
    }
}

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
        println!("-------");
        for row_idx in 0..NUM_ROWS {
            print!("|");
            for cell in Row::new(&self.cells, row_idx) {
                print!("{}|", cell);
            }
            println!("");
            println!("-------");
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
    println!("{}", game);
    game.pretty_print();
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bad_cell_to_from_string() {
        {
            let cell = "".to_string().parse::<Cell>();
            assert!(matches!(cell, Err(ParseCellError::Empty)));
        }
        {
            let cell = "OO".to_string().parse::<Cell>();
            assert!(matches!(cell, Err(ParseCellError::BadLen)));
        }
        {
            let cell = "D".to_string().parse::<Cell>();
            assert!(matches!(cell, Err(ParseCellError::BadChar(_))));
        }
    }

    #[test]
    fn test_good_cell_to_from_string() {
        let good_cells = ["X", "O", " "];
        for expected_cell in good_cells {
            let cell = expected_cell.to_string().parse::<Cell>();
            assert!(matches!(cell, Ok(_)));
            let result_cell = cell.unwrap().to_string();
            assert_eq!(expected_cell, result_cell);
        }
    }

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
