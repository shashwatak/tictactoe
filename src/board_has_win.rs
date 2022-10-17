use crate::{board::Board, cell::Cell, cell_iterator::CellIterator};

pub fn board_has_win(board: &Board) -> Cell {
    let mut potential_winner = Cell::Unmarked;
    board.rows().map(|cells| iter_has_win(cells)).for_each(|cell| {
        if let Cell::X | Cell::O = cell {
            potential_winner = cell;
        }
    });
    board.columns().map(|cells| iter_has_win(cells)).for_each(|cell| {
        if let Cell::X | Cell::O = cell {
            potential_winner = cell;
        }
    });
    board.diagonals().map(|cells| iter_has_win(cells)).for_each(|cell| {
        if let Cell::X | Cell::O = cell {
            potential_winner = cell;
        }
    });
    potential_winner
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
        let board = "XOOX OXX ".to_string().parse::<Board>().unwrap();
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
