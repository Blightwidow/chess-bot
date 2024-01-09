pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct SearchLimits {
    pub perft: usize,
    pub fen: String,
}

impl SearchLimits {
    pub fn new() -> SearchLimits {
        SearchLimits {
            perft: 0,
            fen: FEN_START_POSITION.to_string(),
        }
    }
}
