use crate::cell::Cell;
use crate::board::{NUM_CELLS, NUM_ROWS};

pub struct ColIter<'a> {
    col_idx: usize,
    count: usize,
    cells: &'a [Cell; NUM_CELLS],
}

impl<'a> ColIter<'a> {
    pub fn new(cells: &'a [Cell; NUM_CELLS], col_idx: usize) -> ColIter<'a> {
        ColIter {
            col_idx,
            count: 0,
            cells,
        }
    }
}

impl<'a> Iterator for ColIter<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            row_idx if row_idx < NUM_ROWS => {
                let result = &self.cells[row_idx * NUM_ROWS + self.col_idx];
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
    fn test_col_iter() {
        let board = "XOXOXOOXO".to_string().parse::<Board>().unwrap();
        let mut col = ColIter::new(&board.cells, 0);
        assert!(matches!(col.next().unwrap(), Cell::X));
        assert!(matches!(col.next().unwrap(), Cell::O));
        assert!(matches!(col.next().unwrap(), Cell::O));
        assert!(matches!(col.next(), None));
    }
}
