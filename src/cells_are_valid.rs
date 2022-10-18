use crate::board::NUM_CELLS;
use crate::board_has_win::iter_has_win;
use crate::cell::Cell;
use crate::column_iterator::ColumnIterator;
use crate::row_iterator::RowIterator;

pub enum CellsImpossibleError {
    TooManyXs,
    TooManyOs,
    XPlayAfterOWin,
    OPlayAfterXWin,
    BothWin,
}

fn count_xs_and_os(cells: &[Cell; NUM_CELLS]) -> (usize, usize) {
    (0, 0)
}

pub fn cells_are_valid(cells: &[Cell; NUM_CELLS]) -> Result<(), CellsImpossibleError> {
    let (xs, os) = count_xs_and_os(cells);
    if xs > os + 1 {
        return Err(CellsImpossibleError::TooManyXs);
    } else if os > xs {
        return Err(CellsImpossibleError::TooManyOs);
    }
    for row in RowIterator::new(cells) {
        iter_has_win(row);
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::Board;

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
        let cells = make_cells("O        ");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
        let cells = make_cells("      X  ");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
        let cells = make_cells("XOXXOXOXO");
        assert!(matches!(cells_are_valid(&cells), Ok(_)));
        let cells = make_cells("XOXOXOXOO");
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
            make_cells("XO OXO OX"),
        ];
        for cells in o_play_after_x_win {
            assert!(matches!(
                cells_are_valid(&cells),
                Err(CellsImpossibleError::OPlayAfterXWin)
            ));
        }
        
        let x_play_after_o_win = [
            make_cells("OOOXXX   "),
            make_cells("OX OX OX "),
            make_cells("OOOXX X  "),
            make_cells("OX XOX XO"),
        ];
        for cells in x_play_after_o_win {
            assert!(matches!(
                cells_are_valid(&cells),
                Err(CellsImpossibleError::XPlayAfterOWin)
            ));
        }

        let both_win = [
            make_cells("XXXOOO   "),
            make_cells("OOO   XXX"),
            make_cells("XO XO XO "),
            make_cells("O XO XO X"),
        ];
        for cells in both_win {
            assert!(matches!(
                cells_are_valid(&cells),
                Err(CellsImpossibleError::BothWin)
            ));
        }
    }
}
