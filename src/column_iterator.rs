use crate::board::{Board, NUM_CELLS, NUM_ROWS as COL_SIZE};
use crate::cell::Cell;
use crate::cell_iterator::CellIterator;

pub struct ColumnIterator<'a> {
    count: usize,
    cells: &'a [Cell; NUM_CELLS],
}

impl<'a> ColumnIterator<'a> {
    pub fn new(cells: &'a [Cell; NUM_CELLS]) -> ColumnIterator<'a> {
        ColumnIterator { count: 0, cells }
    }
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = CellIterator<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= COL_SIZE {
            None
        } else {
            let next_column_cell_idx =
                |column_idx: usize, count: usize| column_idx + (COL_SIZE * count);
            let column_iterator = CellIterator::new(self.cells, self.count, next_column_cell_idx);
            self.count += 1;
            Some(column_iterator)
        }
    }
}

impl Board {
    pub fn columns(&self) -> ColumnIterator{
        ColumnIterator::new(&self.cells)
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::Board;

    #[test]
    fn test_column_iterator() {
        let board = "XOXOXOXXO".to_string().parse::<Board>().unwrap();

        let mut columns = board.columns();

        let mut column = columns.next().unwrap();
        assert!(matches!(column.next().unwrap(), Cell::X));
        assert!(matches!(column.next().unwrap(), Cell::O));
        assert!(matches!(column.next().unwrap(), Cell::X));
        assert!(matches!(column.next(), None));

        let mut column = columns.next().unwrap();
        assert!(matches!(column.next().unwrap(), Cell::O));
        assert!(matches!(column.next().unwrap(), Cell::X));
        assert!(matches!(column.next().unwrap(), Cell::X));
        assert!(matches!(column.next(), None));

        let mut column = columns.next().unwrap();
        assert!(matches!(column.next().unwrap(), Cell::X));
        assert!(matches!(column.next().unwrap(), Cell::O));
        assert!(matches!(column.next().unwrap(), Cell::O));
        assert!(matches!(column.next(), None));
    }
}
