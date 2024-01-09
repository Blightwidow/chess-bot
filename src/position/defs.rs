use crate::{defs::*, movegen::defs::CastlingRights};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StateInfo {
    // Copied when making a move
    pub en_passant_square: Square,
    pub captured_piece: Piece,
    pub castling_rights: usize,
}

impl StateInfo {
    pub fn new() -> Self {
        return Self {
            en_passant_square: NONE_SQUARE,
            captured_piece: PieceType::NONE,
            castling_rights: CastlingRights::NONE,
        };
    }
}
