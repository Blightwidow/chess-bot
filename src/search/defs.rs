use crate::defs::{Side, Sides};

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct SearchLimits {
    pub perft: usize,
    pub depth: usize,
    pub ponder: bool,
    pub white_time: usize,
    pub black_time: usize,
    pub white_inc: usize,
    pub black_inc: usize,
    pub moves_to_go: usize,
    pub nodes: usize,
    pub mate: usize,
    pub movetime: usize,
}

impl SearchLimits {
    pub fn default() -> SearchLimits {
        SearchLimits {
            perft: 0,
            depth: 13,
            ponder: false,
            white_time: usize::MAX,
            black_time: usize::MAX,
            white_inc: 0,
            black_inc: 0,
            moves_to_go: 0,
            nodes: usize::MAX,
            mate: 0,
            movetime: usize::MAX,
        }
    }

    pub fn time(&self, side: Side) -> usize {
        return match side {
            Sides::WHITE => self.white_time,
            Sides::BLACK => self.black_time,
            _ => panic!("Invalid side"),
        };
    }
}

pub const VALUE_ZERO: isize = 0;
pub const VALUE_DRAW: isize = VALUE_ZERO;
pub const VALUE_MATE: isize = 32000;
pub const VALUE_INFINITE: isize = 32001;
pub const VALUE_NONE: isize = 32002;