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
            "X" => Ok(Cell::X),
            "O" => Ok(Cell::O),
            " " => Ok(Cell::Unmarked),
            "" => Err(Self::Err::Empty),
            c if c.len() >= 2 => Err(Self::Err::BadLen),
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
            .map(|val| 
            match val {
                (i, cell) if i % 3 == 2 => cell.to_string() + "\n", 
                (_, cell) => cell.to_string(),
            })
            .collect();
        // for i in 0..9 {
        //     let cell = &self.cells[i].to_string();
        //     match i {
        //         n if n % 3 == 2 => out.push_str(cell),
        //         _ => out.push_str(cell),
        //     }
        // }
        write!(f, "{}", out.join(""))
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
    fn test() {
        assert_eq!(1, 1);
    }
}
