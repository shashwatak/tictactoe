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
    BadChar,
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
            _ => Err(Self::Err::BadChar),
        }
    }
}

#[derive(Debug, Default)]
struct Game {
    // we will use iterators to turn these 9 cells into
    // coherent rows and columns
    cells: [Cell; 9],
    // TODO abstract to any NxN (or NxM?) board
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self
            .cells
            .iter()
            .enumerate()
            .map(|val| match val {
                (i, cell) if i % 3 == 2 => cell.to_string() + "\n",
                (_, cell) => cell.to_string(),
            })
            .collect();
        write!(f, "{}", out.join(""))
    }
}

impl FromStr for Game {
    type Err = ParseCellError;
    fn from_str(game_str: &str) -> Result<Self, Self::Err> {
        let game_string: String = game_str.chars().filter(|c| *c != '\n').collect();
        match &game_string[..] {
            "" => Err(Self::Err::Empty),
            g if g.len() != 9 => Err(Self::Err::BadLen),
            g => g.chars().enumerate().map(|i, v| ),
        }
    }
}
fn main() {
    let mut game = Game::default();
    game.cells[0] = Cell::X;
    game.cells[4] = Cell::O;
    game.cells[8] = Cell::X;
    println!("{}", game.to_string());
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
            assert!(matches!(cell, Err(ParseCellError::BadChar)));
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
            assert!(matches!(game, Err(ParseCellError::Empty)));
        }
        {
            let game = "XXO\nOO\nXXO".to_string().parse::<Game>();
            assert!(matches!(game, Err(ParseCellError::BadLen)));
        }
        {
            let game = "XXO\nOOX\nXXXO".to_string().parse::<Game>();
            assert!(matches!(game, Err(ParseCellError::BadLen)));
        }
        {
            let game = "XXO\nOOD\nXXO".to_string().parse::<Game>();
            assert!(matches!(game, Err(ParseCellError::BadChar)));
        }
    }

    #[test]
    fn test_game_to_from_string() {
        let expected_game = "XOX\nOXO\nOXO";
        let game = expected_game.to_string().parse::<Game>();
        assert!(matches!(game, Ok(_)));
        let result_game = game.unwrap().to_string();
        assert_eq!(expected_game, result_game);
    }
}
