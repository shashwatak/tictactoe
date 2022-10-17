use crate::board::{Board, NUM_CELLS, NUM_COLS as ROW_SIZE, NUM_ROWS as COL_SIZE};
use crate::cell::Cell;
use crate::cell_iterator::CellIterator;

pub struct RowIterator<'a> {
    count: usize,
    cells: &'a [Cell; NUM_CELLS],
}

impl<'a> RowIterator<'a> {
    pub fn new(cells: &'a [Cell; NUM_CELLS]) -> RowIterator<'a> {
        RowIterator { count: 0, cells }
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = CellIterator<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= COL_SIZE {
            None
        } else {
            let next_row_cell_idx = |row_idx: usize, count: usize| row_idx * ROW_SIZE + count;
            let row_iterator = CellIterator::new(self.cells, self.count, next_row_cell_idx);
            self.count += 1;
            Some(row_iterator)
        }
    }
}

impl Board {
    pub fn rows(&self) -> RowIterator{
        RowIterator::new(&self.cells)
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::Board;

    #[test]
    fn test_row_iterator() {
        let board = "XOXOXOXXO".to_string().parse::<Board>().unwrap();

        let mut rows = board.rows();

        let mut row = rows.next().unwrap();
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next().unwrap(), Cell::O));
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next(), None));

        let mut row = rows.next().unwrap();
        assert!(matches!(row.next().unwrap(), Cell::O));
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next().unwrap(), Cell::O));
        assert!(matches!(row.next(), None));

        let mut row = rows.next().unwrap();
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next().unwrap(), Cell::O));
        assert!(matches!(row.next(), None));
    }
}
