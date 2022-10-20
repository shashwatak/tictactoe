use crate::board::NUM_CELLS;
use crate::cell::Cell;
use crate::cell_id::CellId;
use crate::player::Player;

#[derive(Debug)]
pub struct CellIsMarked;

pub fn check_cell(cells: &[Cell; NUM_CELLS], cell_id: CellId) -> Result<(), CellIsMarked> {
    let cell_idx = cell_id.to_idx();
    if let Cell::Player(_) = cells[cell_idx] {
        return Err(CellIsMarked);
    }
    Ok(())
}

pub fn next_player(player: Player) -> Player {
    if let Player::X = player {
        return Player::O;
    }
    Player::X
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_cell() {
        let mut cells = [Cell::Unmarked; NUM_CELLS];

        let cell_id = "a1".to_string().parse::<CellId>().unwrap();
        let result = check_cell(&cells, cell_id);
        assert!(matches!(result, Ok(_)));

        let cell_id = "a1".to_string().parse::<CellId>().unwrap();
        cells[cell_id.to_idx()] = Cell::Player(Player::X);

        let result = check_cell(&cells, cell_id);
        assert!(matches!(result, Err(CellIsMarked)));
    }
}
