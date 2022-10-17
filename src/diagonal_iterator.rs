use crate::board::{Board, NUM_CELLS, NUM_ROWS as COL_SIZE};
use crate::cell::Cell;
use crate::cell_iterator::CellIterator;

pub struct DiagonalIterator<'a> {
    diagonal_idx: usize,
    cells: &'a [Cell; NUM_CELLS],
}

impl<'a> DiagonalIterator<'a> {
    pub fn new(cells: &'a [Cell; NUM_CELLS]) -> DiagonalIterator<'a> {
        DiagonalIterator { diagonal_idx: 0, cells }
    }
}

impl<'a> Iterator for DiagonalIterator<'a> {
    type Item = CellIterator<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.diagonal_idx == 0 {
            let increasing_diagonal = |_: usize, count: usize| count + (COL_SIZE * count);
            let diagonal_iterator = CellIterator::new(self.cells, self.diagonal_idx, increasing_diagonal);
            self.diagonal_idx += 1;
            Some(diagonal_iterator)
        } else if self.diagonal_idx == 1 {
            let decreasing_diagonal =
                |_: usize, count: usize| (COL_SIZE - count - 1) + (COL_SIZE * count);
            let diagonal_iterator = CellIterator::new(self.cells, self.diagonal_idx, decreasing_diagonal);
            self.diagonal_idx += 1;
            Some(diagonal_iterator)
        } else {
            None
        }
    }
}

impl Board {
    pub fn diagonals(&self) -> DiagonalIterator {
        DiagonalIterator::new(&self.cells)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::Board;

    #[test]
    fn test_diagonal_iterator() {
        let board = "X O X O X".to_string().parse::<Board>().unwrap();

        let mut diagonals = DiagonalIterator::new(&board.cells);

        let mut diagonal = diagonals.next().unwrap();
        assert!(matches!(diagonal.next().unwrap(), Cell::X));
        assert!(matches!(diagonal.next().unwrap(), Cell::X));
        assert!(matches!(diagonal.next().unwrap(), Cell::X));
        assert!(matches!(diagonal.next(), None));

        let mut diagonal = diagonals.next().unwrap();
        assert!(matches!(diagonal.next().unwrap(), Cell::O));
        assert!(matches!(diagonal.next().unwrap(), Cell::X));
        assert!(matches!(diagonal.next().unwrap(), Cell::O));
        assert!(matches!(diagonal.next(), None));
    }
}
