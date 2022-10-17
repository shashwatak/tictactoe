use crate::{board::Board, cell::Cell, cell_iterator::CellIterator, row_iterator::RowIterator, column_iterator::ColumnIterator, diagonal_iterator::DiagonalIterator};

pub fn board_has_win(board: &Board) -> Cell {
    for row in RowIterator::new(&board.cells) {
        let cell = iter_has_win(row);
        if !matches!(cell, Cell::Unmarked) {
            return cell;
        }
    }
    for column in ColumnIterator::new(&board.cells) {
        let cell = iter_has_win(column);
        if !matches!(cell, Cell::Unmarked) {
            return cell;
        }
    }
    for column in DiagonalIterator::new(&board.cells) {
        let cell = iter_has_win(column);
        if !matches!(cell, Cell::Unmarked) {
            return cell;
        }
    }
    Cell::Unmarked
}

pub fn iter_has_win(mut iter: CellIterator) -> Cell {
    let first = *iter.next().unwrap();
    for cell in iter {
        if first != *cell {
            return Cell::Unmarked;
        }
    }
    first
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::Board;

    #[test]
    fn test_board_with_win() {
        let board = "XOOXXOXXO".to_string().parse::<Board>().unwrap();
        assert!(matches!(board_has_win(&board), Cell::X));
        let board = "OOO XX X ".to_string().parse::<Board>().unwrap();
        assert!(matches!(board_has_win(&board), Cell::O));
    }

    #[test]
    fn test_board_with_no_win() {
        let board = "         ".to_string().parse::<Board>().unwrap();
        let cell = board_has_win(&board);
        assert!(matches!(cell, Cell::Unmarked));
        let board = "XOXXOXOXO".to_string().parse::<Board>().unwrap();
        let cell = board_has_win(&board);
        assert!(matches!(cell, Cell::Unmarked));
    }
}
