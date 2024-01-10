pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct SearchLimits {
    pub perft: usize,
}

impl SearchLimits {
    pub fn new() -> SearchLimits {
        SearchLimits { perft: 0 }
    }
}
