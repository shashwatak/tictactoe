use crate::cell::Cell;
use crate::board::{NUM_CELLS, NUM_COLS};

pub struct RowIter<'a> {
    row_idx: usize,
    count: usize,
    cells: &'a [Cell; NUM_CELLS],
}

impl<'a> RowIter<'a> {
    pub fn new(cells: &'a [Cell; NUM_CELLS], row_idx: usize) -> RowIter<'a> {
        RowIter {
            row_idx,
            count: 0,
            cells,
        }
    }
}

impl<'a> Iterator for RowIter<'a> {
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::Board;

    #[test]
    fn test_row_iter() {
        let board = "XOXOXOOXO".to_string().parse::<Board>().unwrap();
        let mut row = RowIter::new(&board.cells, 0);
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next().unwrap(), Cell::O));
        assert!(matches!(row.next().unwrap(), Cell::X));
        assert!(matches!(row.next(), None));
    }
}
