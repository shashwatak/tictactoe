use crate::board::{NUM_CELLS, NUM_COLS as ROW_SIZE, NUM_ROWS as COL_SIZE};
use crate::cell::Cell;

fn next_row_cell_idx(row_idx: usize, count: usize) -> usize {
    row_idx * ROW_SIZE + count
}

fn next_col_cell_idx(col_idx: usize, count: usize) -> usize {
    COL_SIZE * count + col_idx
}

pub struct BoardIter<'a, NextCell> {
    count: usize,
    cells: &'a [Cell; NUM_CELLS],
    dimension_index: usize,
    next_cell: NextCell,
}

impl<'a, NextCell> BoardIter<'a, NextCell>
where
    NextCell: Fn(usize, usize) -> usize,
{
    pub fn new(
        cells: &'a [Cell; NUM_CELLS],
        dimension_index: usize,
        next_cell: NextCell,
    ) -> BoardIter<'a, NextCell> {
        BoardIter {
            count: 0,
            cells,
            dimension_index,
            next_cell,
        }
    }
}

impl<'a, NextCell> Iterator for BoardIter<'a, NextCell>
where
    NextCell: Fn(usize, usize) -> usize,
{
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

        let mut board = BoardIter::new(&board.cells, 0, next_row_cell_idx);
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next(), None));

        let mut board = BoardIter::new(&board.cells, 1, next_row_cell_idx);
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next(), None));

        let mut board = BoardIter::new(&board.cells, 2, next_row_cell_idx);
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next(), None));

        let mut board = BoardIter::new(&board.cells, 0, next_col_cell_idx);
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next(), None));

        let mut board = BoardIter::new(&board.cells, 1, next_col_cell_idx);
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next(), None));

        let mut board = BoardIter::new(&board.cells, 2, next_col_cell_idx);
        assert!(matches!(board.next().unwrap(), Cell::X));
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next().unwrap(), Cell::O));
        assert!(matches!(board.next(), None));
    }
}
