use std::fmt;

use crate::defs::*;

pub type MoveType = u16;
pub struct MoveTypes {}

impl MoveTypes {
    pub const NORMAL: u16 = 0;
    pub const PROMOTION: u16 = 1 << 14;
    pub const EN_PASSANT: u16 = 2 << 14;
    pub const CASTLING: u16 = 3 << 14;
}

pub type CastlingRight = usize;
pub struct CastlingRights {}
impl CastlingRights {
    pub const NONE: CastlingRight = 0;
    pub const WHITE_KINGSIDE: CastlingRight = 1;
    pub const WHITE_QUEENSIDE: CastlingRight = 1 << 1;
    pub const BLACK_KINGSIDE: CastlingRight = 1 << 2;
    pub const BLACK_QUEENSIDE: CastlingRight = 1 << 3;
    pub const WHITE: CastlingRight = CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE;
    pub const BLACK: CastlingRight = CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE;
}

pub fn pawn_push(side: Side) -> Direction {
    return match side {
        Sides::WHITE => Directions::UP,
        Sides::BLACK => Directions::DOWN,
        _ => panic!("Invalid side"),
    };
}

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    data: u16,
}
impl Move {
    pub fn new(data: u16) -> Self {
        return Self { data: data };
    }

    pub fn with_from_to(from: Square, to: Square) -> Self {
        return Self::new(((from << 6) + to) as u16);
    }

    pub fn make(from: Square, to: Square, promotion_type: Piece, movetype: MoveType) -> Self {
        let promotion_value = match promotion_type {
            PieceType::KNIGHT => PieceType::KNIGHT - PieceType::KNIGHT,
            PieceType::BISHOP => PieceType::BISHOP - PieceType::KNIGHT,
            PieceType::ROOK => PieceType::ROOK - PieceType::KNIGHT,
            PieceType::QUEEN => PieceType::QUEEN - PieceType::KNIGHT,
            _ => 0,
        };
        return Self::new(movetype + (promotion_value << 12) as u16 + (from << 6) as u16 + to as u16);
    }

    pub fn from_sq(&self) -> Square {
        return (self.data >> 6) as Square;
    }

    pub fn to_sq(&self) -> Square {
        return (self.data & 0x3F) as Square;
    }

    pub fn type_of(&self) -> MoveType {
        return self.data & (3 << 14);
    }

    pub fn promotion_type(&self) -> Piece {
        if self.type_of() != MoveTypes::PROMOTION {
            return PieceType::NONE;
        }
        return ((self.data >> 12) & 0b11) as usize + PieceType::KNIGHT;
    }

    pub fn is_ok(&self) -> bool {
        return Self::none().data != self.data && Self::null().data != self.data;
    }

    pub fn null() -> Self {
        return Self { data: 65 };
    }

    pub fn none() -> Self {
        return Self { data: 0 };
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let promotion_string = match self.promotion_type() {
            PieceType::KNIGHT => "=K",
            PieceType::BISHOP => "=B",
            PieceType::ROOK => "=R",
            PieceType::QUEEN => "=Q",
            PieceType::NONE => "",
            _ => panic!("Invalid promotion type"),
        };
        return write!(
            f,
            "{}{}{}",
            pretty_square(self.from_sq()),
            pretty_square(self.to_sq()),
            promotion_string
        );
    }
}
