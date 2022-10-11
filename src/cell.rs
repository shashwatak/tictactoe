use core::fmt;
use std::str::FromStr;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
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
pub enum ParseCellError {
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

#[cfg(test)]
mod tests {

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
}
