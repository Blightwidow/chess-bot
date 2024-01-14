pub mod defs;
mod tables;

use crate::{defs::*, movegen::defs::Move, position::Position, search::defs::VALUE_INFINITE};

use self::{defs::*, tables::*};

pub struct Eval {}

impl Eval {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, position: &Position) -> isize {
        let us: Side = position.side_to_move;
        let them: Side = us ^ 1;
        let mut middle_game: [isize; NrOf::SIDES] = [0; NrOf::SIDES];
        let mut eng_game: [isize; NrOf::SIDES] = [0; NrOf::SIDES];
        let mut phase: isize = 0;

        for square in RangeOf::SQUARES {
            let piece = position.board[square];

            if piece != PieceType::NONE {
                let piece_type: Piece = type_of_piece(piece);
                let color: Side = color_of_piece(piece);
                let piece_index: Square = match color == us {
                    true => square,
                    false => square ^ 56,
                };

                middle_game[color] += PIECE_VALUES_MG[piece_type] + PIECE_SQUARE_MG_TABLES[piece_type][piece_index];
                eng_game[color] += PIECE_VALUES_EG[piece_type] + PIECE_SQUARE_EG_TABLES[piece_type][piece_index];
                phase += GAME_PHASE_INCREMENT[type_of_piece(piece)];
            }
        }

        let mg_score: isize = middle_game[us] - middle_game[them];
        let eg_score: isize = eng_game[us] - eng_game[them];
        let score: isize = match phase >= 24 {
            true => mg_score,
            false => (mg_score * phase + eg_score * (24 - phase)) / 24,
        };

        // Grain of 1/50 of a pawn unit
        let grained_score = (score * PAWN_UNIT / 50) * 50 / PAWN_UNIT;

        return grained_score.min(VALUE_INFINITE).max(-VALUE_INFINITE);
    }

    pub fn order_moves(&self, position: &Position, moves: &mut [Move]) {
        moves.sort_by_key(|mv| self.static_exchange_evaluation(position, *mv));
    }

    fn static_exchange_evaluation(&self, position: &Position, mv: Move) -> isize {
        let captured_piece = position.board[mv.to_sq()];

        if captured_piece == PieceType::NONE {
            return 0;
        }

        let piece = position.board[mv.from_sq()];

        return PIECE_VALUES_INITIAL[type_of_piece(captured_piece)] - PIECE_VALUES_INITIAL[type_of_piece(piece)];
    }
}
