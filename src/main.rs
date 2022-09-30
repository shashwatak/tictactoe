use core::fmt;

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
struct Game {
    // we will use iterators to turn these 9 cells into
    // coherent rows and columns
    cells: [Cell; 9],
    // TODO abstract to any NxN (or NxM?) board
}

fn display_cell_in_middle(cell: &Cell) -> String {
    format!(" {} |", cell)
}

fn display_cell_at_end(cell: &Cell) -> String {
    format!(" {} \n", cell)
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out: String = "".to_string();
        for i in 0..9 {
            match i {
                n if n % 3 == 2 => out.push_str(&display_cell_at_end(&self.cells[i])),
                _ => out.push_str(&display_cell_in_middle(&self.cells[i])),
            }
        }
        write!(f, "{out}")
    }
}
fn main() {
    let mut game = Game::default();
    game.cells[0] = Cell::X;
    game.cells[4] = Cell::O;
    game.cells[8] = Cell::X;
    println!("{game}");
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
