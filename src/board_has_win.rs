use crate::{board::Board, cell::Cell, cell_iterator::CellIterator, player::Player};

pub fn board_has_win(board: &Board) -> Cell {
    let mut potential_winner = Cell::Unmarked;
    board.rows().map(iter_has_win).for_each(|cell| {
        if let Cell::Player(Player::X) | Cell::Player(Player::O) = cell {
            potential_winner = cell;
        }
    });
    board.columns().map(iter_has_win).for_each(|cell| {
        if let Cell::Player(Player::X) | Cell::Player(Player::O) = cell {
            potential_winner = cell;
        }
    });
    board.diagonals().map(iter_has_win).for_each(|cell| {
        if let Cell::Player(Player::X) | Cell::Player(Player::O) = cell {
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
    use crate::player::Player;

    #[test]
    fn test_board_with_win() {
        let board = "XOOX OXX ".to_string().parse::<Board>().unwrap();
        assert!(matches!(board_has_win(&board), Cell::Player(Player::X)));
        let board = "OOO XX X ".to_string().parse::<Board>().unwrap();
        assert!(matches!(board_has_win(&board), Cell::Player(Player::O)));
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
