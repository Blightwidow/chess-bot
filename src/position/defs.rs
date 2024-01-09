use crate::{defs::*, movegen::defs::CastlingRights};

#[derive(Clone, Copy)]
pub struct StateInfo {
    // Copied when making a move
    pub en_passant_square: Square,
    pub captured_piece: Piece,
    pub castling_rights: usize,
    // // Copied when making a move
    // Key    materialKey;
    // Key    pawnKey;
    // Value  nonPawnMaterial[COLOR_NB];
    // int    rule50;
    // int    pliesFromNull;

    // // Not copied when making a move (will be recomputed anyhow)
    // Key        key;
    // Bitboard   checkersBB;
    // StateInfo* previous;
    // Bitboard   blockersForKing[COLOR_NB];
    // Bitboard   pinners[COLOR_NB];
    // Bitboard   checkSquares[PIECE_TYPE_NB];
    // Piece      capturedPiece;
    // int        repetition;
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
