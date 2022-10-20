use crate::cell_id::CellId;
enum GameUpdateError {
    CellId(CellIdError),
    PlayerOutOfTurn,
    GameIsOver,
}

