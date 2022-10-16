use crate::board::{NUM_CELLS, NUM_COLS as ROW_SIZE};
use crate::cell::Cell;

pub struct CellIterator<'a> {
    count: usize,
    cells: &'a [Cell; NUM_CELLS],
    dimension_index: usize,
    next_cell: fn(usize, usize) -> usize,
}

impl<'a> CellIterator<'a> {
    pub fn new(
        cells: &'a [Cell; NUM_CELLS],
        dimension_index: usize,
        next_cell: fn(usize, usize) -> usize,
    ) -> CellIterator<'a> {
        CellIterator {
            count: 0,
            cells,
            dimension_index,
            next_cell,
        }
    }
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= ROW_SIZE {
            None
        } else {
            let idx = (self.next_cell)(self.dimension_index, self.count);
            let result = &self.cells[idx];
            self.count += 1;
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::Board;

    #[test]
    fn test_board_iter() {
        let board = "XOXOXOXXO".to_string().parse::<Board>().unwrap();
        fn next_cell_idx(_: usize, count: usize) -> usize {
            count
        }

        let mut board = CellIterator::new(&board.cells, 0, next_cell_idx);
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next(), None));
    }
}
