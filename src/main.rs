use core::fmt;

#[derive(Debug, Default) ]
enum Cell {
    #[default]
    Empty,
    X,
    O,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
            Cell::Empty => write!(f, " "),
        }
    }
}

#[derive(Debug, Default) ]
struct Game {
    // we will use iterators to turn these 9 cells into
    // coherent rows and columns
    cells: [Cell; 9],
    // TODO abstract to any NxN (or NxM?) board
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cells[0])
    }
}
fn main() {
    let cell_o = Cell::O;
    let cell_x = Cell::X;
    let cell_empty = Cell::Empty;
    println!("{cell_o} {cell_x} {cell_empty}");

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
