use crate::board::NUM_CELLS;
use crate::board_has_win::iter_has_win;
use crate::cell::Cell;
use crate::column_iterator::ColumnIterator;
use crate::diagonal_iterator::DiagonalIterator;
use crate::player::Player;
use crate::row_iterator::RowIterator;

#[derive(Debug)]
pub enum CellsImpossibleError {
    TooManyXs,
    TooManyOs,
    XPlayAfterOWin,
    OPlayAfterXWin,
}

fn count_xs_and_os(cells: &[Cell; NUM_CELLS]) -> (usize, usize) {
    let mut num_xs: usize = 0;
    let mut num_os: usize = 0;
    for cell in cells {
        if let Cell::Player(p) = cell {
            match p {
                Player::X => num_xs += 1,
                Player::O => num_os += 1,
            }
        }
    }
    (num_xs, num_os)
}

pub fn cells_are_valid(cells: &[Cell; NUM_CELLS]) -> Result<(), CellsImpossibleError> {
    let (num_xs, num_os) = count_xs_and_os(cells);
    if num_xs > num_os + 1 {
        return Err(CellsImpossibleError::TooManyXs);
    } else if num_os > num_xs {
        return Err(CellsImpossibleError::TooManyOs);
    }
    let mut x_win = false;
    let mut o_win = false;

    let tracks = RowIterator::new(cells)
        .chain(ColumnIterator::new(cells))
        .chain(DiagonalIterator::new(cells));
    for track in tracks {
        let cell = iter_has_win(track);
        if let Cell::Player(Player::X) = cell {
            x_win = true;
        }
        if let Cell::Player(Player::O) = cell {
            o_win = true;
        }
    }

    if x_win && num_os == num_xs {
        return Err(CellsImpossibleError::OPlayAfterXWin);
    }
    if o_win && num_xs == num_os + 1 {
        return Err(CellsImpossibleError::XPlayAfterOWin);
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    fn make_cells(cells_str: &str) -> [Cell; NUM_CELLS] {
        assert_eq!(cells_str.len(), NUM_CELLS);
        let mut cells = [Cell::Unmarked; NUM_CELLS];
        cells_str
            .chars()
            .enumerate()
            .for_each(
                |(cell_idx, cell_char)| match cell_char.to_string().parse::<Cell>() {
                    Ok(cell) => cells[cell_idx] = cell,
                    _ => panic!("oops"),
                },
            );
        cells
    }

    #[test]
    fn test_cells_are_possible() {
        let cells = make_cells("         ");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
        let cells = make_cells("X        ");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
        let cells = make_cells("O     X  ");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
        let cells = make_cells("XOXXOXOXO");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
        let cells = make_cells(" OX X XO ");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
        let cells = make_cells("OXOXOXOX ");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
    }

    #[test]
    fn test_cells_are_illogical() {
        let too_many_xs = [
            make_cells("XX       "),
            make_cells("XX OO XX "),
            make_cells("XXXOOOXX "),
        ];
        for cells in too_many_xs {
            assert!(matches!(
                cells_are_valid(&cells),
                Err(CellsImpossibleError::TooManyXs)
            ));
        }

        let too_many_os = [
            make_cells("O        "),
            make_cells("XX OO O  "),
            make_cells("XXXOOOO  "),
        ];
        for cells in too_many_os {
            assert!(matches!(
                cells_are_valid(&cells),
                Err(CellsImpossibleError::TooManyOs)
            ));
        }

        let o_play_after_x_win = [
            make_cells("XXXOOO   "),
            make_cells("XO XO XO "),
            make_cells("XXXOO O  "),
            make_cells("XO OXO  X"),
        ];
        for cells in o_play_after_x_win {
            assert!(matches!(
                cells_are_valid(&cells),
                Err(CellsImpossibleError::OPlayAfterXWin)
            ));
        }

        let x_play_after_o_win = [
            make_cells("OOOXX XX "),
            make_cells("OXXOX O X"),
            make_cells("OOOXXXX  "),
            make_cells("OX XOX XO"),
        ];
        for cells in x_play_after_o_win {
            assert!(matches!(
                cells_are_valid(&cells),
                Err(CellsImpossibleError::XPlayAfterOWin)
            ));
        }
    }
}
