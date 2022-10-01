#[derive(Debug)]
pub struct ChessPosition {
    row: usize,
    col: usize,
}

impl ChessPosition {
    pub fn new(row: i32, col: i32) -> Option<Self> {
        if row < 0 || col < 0 || row > 7 || col > 7 {
            None
        } else {
            Some(ChessPosition { row: row as usize, col: col as usize, })
        }
    }

    pub fn relative_pos(&self, other: &ChessPosition) -> Self {
        ChessPosition {
            row: self.row.abs_diff(other.row),
            col: self.col.abs_diff(other.col),
        }

    }
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self { Queen { position } }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let relative_pos = self.position.relative_pos(&other.position);
        relative_pos.row == 0 || relative_pos.col == 0 || relative_pos.col == relative_pos.row
    }
}
