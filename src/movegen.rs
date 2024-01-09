pub mod defs;

use crate::{
    bitboards::{defs::*, Bitboards},
    defs::*,
    misc::bits,
    position::Position,
};

use self::defs::*;

pub struct Movegen {
    bitboards: Bitboards,
}

impl Movegen {
    pub fn new(bitboards: Bitboards) -> Self {
        Self { bitboards }
    }

    pub fn legal_moves(&self, position: &Position) -> Vec<Move> {
        let us = position.side_to_move;
        let movelist = self.generate(position, us);

        return movelist
            .iter()
            .map(|mv| *mv)
            .filter(|mv: &Move| position.legal(*mv))
            .collect();
    }

    fn generate(&self, position: &Position, us: Side) -> Vec<Move> {
        let mut movelist: Vec<Move> = Vec::new();

        if position.checkers(us).len() <= 1 {
            self.generate_pawns(position, &mut movelist, us);
            self.generate_piece(position, &mut movelist, PieceType::KNIGHT, us);
            self.generate_piece(position, &mut movelist, PieceType::BISHOP, us);
            self.generate_piece(position, &mut movelist, PieceType::ROOK, us);
            self.generate_piece(position, &mut movelist, PieceType::QUEEN, us);
        }

        self.generate_piece(position, &mut movelist, PieceType::KING, us);

        // TODO: Add castling

        return movelist;
    }

    fn generate_pawns(&self, position: &Position, movelist: &mut Vec<Move>, us: Side) {
        let them: Side = us ^ 1;
        let up: Direction = match us {
            Sides::WHITE => Directions::UP,
            Sides::BLACK => Directions::DOWN,
            _ => panic!("Invalid side"),
        };
        let rank_7bb: Bitboard = match us {
            Sides::WHITE => RANK_7BB,
            Sides::BLACK => RANK_2BB,
            _ => panic!("Invalid side"),
        };
        let rank_3bb: Bitboard = match us {
            Sides::WHITE => RANK_3BB,
            Sides::BLACK => RANK_6BB,
            _ => panic!("Invalid side"),
        };
        let empty_squares: Bitboard = !position.by_color_bb[Sides::BOTH];
        let pawns_on_rank_7: Bitboard = position.by_type_bb[us][PieceType::PAWN] & rank_7bb;
        let pawns_outside_rank_7: Bitboard = position.by_type_bb[us][PieceType::PAWN] & !rank_7bb;

        let mut single_bb: Bitboard = shift(pawns_outside_rank_7, up) & empty_squares;
        // We generate double pawn pushes from the first push to take blockers on the 3rd rank into account
        let mut double_bb: Bitboard = shift(single_bb & rank_3bb, up) & empty_squares;

        while single_bb != EMPTY {
            let to: Square = bits::pop(&mut single_bb);
            movelist.push(Move::with_from_to((to as isize - up) as usize, to))
        }

        while double_bb != EMPTY {
            let to: Square = bits::pop(&mut double_bb);
            movelist.push(Move::with_from_to((to as isize - up - up) as usize, to))
        }

        if pawns_on_rank_7 != EMPTY {
            let mut promotion_bb: Bitboard = shift(pawns_on_rank_7, up) & empty_squares;

            while promotion_bb != EMPTY {
                let to: Square = bits::pop(&mut promotion_bb);

                for piece in [PieceType::KNIGHT, PieceType::BISHOP, PieceType::ROOK, PieceType::QUEEN] {
                    movelist.push(Move::make((to as isize - up) as usize, to, piece, MoveTypes::PROMOTION))
                }
            }

            let mut attackers_bb: Bitboard = pawns_on_rank_7;

            while attackers_bb != EMPTY {
                let from: Square = bits::pop(&mut attackers_bb);
                let mut attack_bb: Bitboard =
                    self.bitboards.attack_bb(PieceType::PAWN, from, EMPTY) & (position.by_color_bb[them]);

                while attack_bb != EMPTY {
                    let to: Square = bits::pop(&mut attack_bb);

                    for piece in [PieceType::KNIGHT, PieceType::BISHOP, PieceType::ROOK, PieceType::QUEEN] {
                        movelist.push(Move::make(from, to, piece, MoveTypes::PROMOTION))
                    }
                }
            }
        }

        let mut attackers_bb: Bitboard = pawns_on_rank_7;

        while attackers_bb != EMPTY {
            let from: Square = bits::pop(&mut attackers_bb);
            let state = position.states.last().unwrap();
            let en_passant_bb: Bitboard = match state.en_passant_square {
                NONE_SQUARE => EMPTY,
                square => square_bb(square),
            };
            let mut attack_bb: Bitboard =
                self.bitboards.attack_bb(PieceType::PAWN, from, EMPTY) & (position.by_color_bb[them] | en_passant_bb);

            while attack_bb != EMPTY {
                let to: Square = bits::pop(&mut attack_bb);
                let move_type = match to == state.en_passant_square {
                    true => MoveTypes::EN_PASSANT,
                    false => MoveTypes::NORMAL,
                };

                movelist.push(Move::make(from, to, PieceType::NONE, move_type));
            }
        }
    }

    fn generate_piece(&self, position: &Position, movelist: &mut Vec<Move>, piece: Piece, us: Side) {
        let piece_type = type_of_piece(piece);
        assert!(piece_type != PieceType::PAWN, "Invalid piece");

        let mut bitboard: Bitboard = position.by_type_bb[us][piece_type];

        while bitboard != EMPTY {
            let from = bits::pop(&mut bitboard);
            let mut attack_bb =
                self.bitboards.attack_bb(piece, from, position.by_color_bb[Sides::BOTH]) & !position.by_color_bb[us];

            while attack_bb != EMPTY {
                let to = bits::pop(&mut attack_bb);
                movelist.push(Move::with_from_to(from, to));
            }
        }
    }
}
