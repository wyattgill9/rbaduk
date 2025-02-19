use crate::board::Board;

#[allow(non_snake_case)]
pub struct Position {
    Board: Board,
    Value: f32,
}

impl Position {
    pub fn new() -> Self {
        Self {
            Board: Board::new(),
            Value: 0.0,
        }
    }

    pub fn get_value(&self) -> f32 {
        self.Value
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
