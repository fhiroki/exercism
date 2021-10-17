#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    x: i32,
    y: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank > 7 || file > 7 {
            return None;
        }
        Some(ChessPosition(rank, file))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            x: position.0,
            y: position.1,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.x == other.x
            || self.y == other.y
            || (self.x - other.x).abs() == (self.y - other.y).abs()
        {
            return true;
        }
        false
    }
}
