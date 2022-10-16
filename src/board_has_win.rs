use crate::{board::Board, cell::Cell, cell_iterator::CellIterator, row_iterator::RowIterator};

pub fn board_has_win(board: &Board) -> Cell {
    for row in RowIterator::new(&board.cells) {
        let cell = row_has_win(row);
        if !matches!(cell, Cell::Unmarked) {
            return cell;
        }
    }
    Cell::Unmarked
}

pub fn row_has_win(mut iter: CellIterator) -> Cell {
    let winner = *iter.next().unwrap();
    for cell in iter {
        if !matches!(*cell, _winner) {
            return Cell::Unmarked;
        }
    }
    winner
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::Board;

    #[test]
    fn test_board_with_win() {
        let board = "OOOXXOXXO".to_string().parse::<Board>().unwrap();
        assert!(matches!(board_has_win(&board), Cell::O));
    }
}
