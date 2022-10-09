mod cell;
mod game;


use crate::cell::Cell;

use crate::game::{Game, NUM_ROWS, NUM_COLS, NUM_CELLS};

    fn pretty_print(game : &Game) {
        println!("   a b c ");
        println!("  -------");
        for row_idx in 0..NUM_ROWS {
            print!("{} |", row_idx + 1);
            for cell in Row::new(&game.cells, row_idx) {
                print!("{}|", cell);
            }
            println!("\n  -------");
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
    pretty_print(&game);
}

#[cfg(test)]
mod tests {

    use super::*;

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
