use crate::{
    board::{Board, NUM_ROWS},
    cell::Cell,
    board_iterator::{BoardIterator, next_row_cell_idx},
};

pub fn board_has_win(board: &Board) -> Cell {
    for row_idx in 0..NUM_ROWS {
        let cell = row_has_win(board, row_idx);
        if ! matches!(cell, Cell::Unmarked) {
            return cell;
        }
    }
    Cell::Unmarked
}

pub fn row_has_win(board: &Board, row_idx: usize) -> Cell {
    let mut row = BoardIterator::new(&board.cells, row_idx, next_row_cell_idx);
    let winner = *row.next().unwrap();  
    for cell in row {
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
