pub mod defs;

use crate::bitboards::defs::EMPTY;
use crate::bitboards::Bitboards;
use crate::defs::*;
use crate::misc::bits;
use crate::movegen::defs::{pawn_push, CastlingRight, CastlingRights, Move, MoveType, MoveTypes};

use self::defs::*;

pub struct Position {
    pub attacks_bb: [Bitboard; NrOf::SIDES],
    pub by_type_bb: [[Bitboard; NrOf::PIECE_TYPES]; NrOf::SIDES],
    pub by_color_bb: [Bitboard; NrOf::SIDES],
    pub pinned_bb: [Bitboard; NrOf::SIDES],
    pub board: [Piece; NrOf::SQUARES],
    pub game_ply: usize,
    pub side_to_move: Side,
    pub states: Vec<StateInfo>,
    bitboards: Bitboards,
    castling_masks: [CastlingRight; NrOf::SQUARES],
}

impl Position {
    pub fn new(bitboards: Bitboards) -> Self {
        return Self {
            bitboards: bitboards,
            attacks_bb: [EMPTY; NrOf::SIDES],
            by_type_bb: [[EMPTY; NrOf::PIECE_TYPES]; NrOf::SIDES],
            by_color_bb: [EMPTY; NrOf::SIDES],
            pinned_bb: [EMPTY; NrOf::SIDES],
            board: [PieceType::NONE; NrOf::SQUARES],
            game_ply: 0,
            side_to_move: Sides::WHITE,
            states: vec![StateInfo::new()],
            castling_masks: Position::castling_masks(),
        };
    }

    pub fn set(&mut self, fen: String) {
        let fen_parts: Vec<&str> = fen.split(' ').collect::<Vec<&str>>();

        assert!(fen_parts.len() == 6);

        let mut square: usize = 0;
        for c in fen_parts[0].split('/').rev().collect::<Vec<&str>>().join("").chars() {
            if c.is_digit(10) {
                square += c.to_digit(10).unwrap() as usize;
            } else {
                let piece_type: Piece = match c.to_ascii_lowercase() {
                    'p' => PieceType::PAWN,
                    'n' => PieceType::KNIGHT,
                    'b' => PieceType::BISHOP,
                    'r' => PieceType::ROOK,
                    'q' => PieceType::QUEEN,
                    'k' => PieceType::KING,
                    _ => panic!("Invalid piece in FEN {}", c),
                };
                let side: Side = match c.is_ascii_lowercase() {
                    true => Sides::BLACK,
                    false => Sides::WHITE,
                };

                self.put_piece(make_piece(side, piece_type), square);
                square += 1;
            }
        }

        self.side_to_move = match fen_parts[1].to_ascii_lowercase().as_str() {
            "w" => Sides::WHITE,
            "b" => Sides::BLACK,
            _ => panic!("Invalid side to move in FEN {}", fen_parts[1]),
        };

        for c in fen_parts[2].chars() {
            match c {
                'K' => self.states.last_mut().unwrap().castling_rights |= CastlingRights::WHITE_KINGSIDE,
                'Q' => self.states.last_mut().unwrap().castling_rights |= CastlingRights::WHITE_QUEENSIDE,
                'k' => self.states.last_mut().unwrap().castling_rights |= CastlingRights::BLACK_KINGSIDE,
                'q' => self.states.last_mut().unwrap().castling_rights |= CastlingRights::BLACK_QUEENSIDE,
                '-' => (),
                _ => panic!("Invalid castling rights in FEN {}", fen_parts[2]),
            }
        }

        if fen_parts[3] != "-" {
            self.states.last_mut().unwrap().en_passant_square = fen_parts[3].parse::<Square>().unwrap();
        }

        // TODO: Parse last two parts of FEN
    }

    // This assume that the move is legal
    // Illegal moves should be filtered out by the move generator before calling this function
    pub fn do_move(&mut self, mv: Move) {
        assert!(mv.is_ok());

        self.game_ply += 1; // TODO: Handle 50 move rule
        let us: Side = self.side_to_move;
        let them: Side = self.side_to_move ^ 1;
        let from: Square = mv.from_sq();
        let to: Square = mv.to_sq();
        let mut piece: Piece = self.piece_on(from);
        let move_type = mv.type_of();
        let captured: Piece = match move_type {
            MoveTypes::EN_PASSANT => self.piece_on((to as isize - pawn_push(us)) as usize),
            MoveTypes::CASTLING => PieceType::NONE,
            _ => self.piece_on(to),
        };
        let mut new_state = *self.states.last().unwrap();

        assert!(color_of_piece(piece) == us);
        assert!(captured == PieceType::NONE || color_of_piece(captured) == them);
        assert!(type_of_piece(captured) != PieceType::KING);

        if captured != PieceType::NONE {
            let captured_square: Square = match move_type {
                MoveTypes::EN_PASSANT => (to as isize - pawn_push(us)) as usize,
                _ => to,
            };

            self.by_type_bb[them][type_of_piece(captured)] &= !square_bb(captured_square);
            self.by_type_bb[Sides::BOTH][type_of_piece(captured)] &= !square_bb(captured_square);
            self.by_color_bb[color_of_piece(captured)] &= !square_bb(captured_square);
            self.board[captured_square] = PieceType::NONE;
        }

        if move_type == MoveTypes::PROMOTION {
            let promoted_piece: Piece = match mv.promotion_type() {
                PieceType::KNIGHT => make_piece(us, PieceType::KNIGHT),
                PieceType::BISHOP => make_piece(us, PieceType::BISHOP),
                PieceType::ROOK => make_piece(us, PieceType::ROOK),
                PieceType::QUEEN => make_piece(us, PieceType::QUEEN),
                _ => panic!("Invalid promotion piece"),
            };

            piece = make_piece(us, promoted_piece);
        }

        if move_type == MoveTypes::CASTLING {
            self.castle(self.castling_masks[to])
        } else if new_state.castling_rights & self.castling_masks[from] != 0
            || new_state.castling_rights & self.castling_masks[to] != 0
        {
            new_state.castling_rights &= !self.castling_masks[from];
            new_state.castling_rights &= !self.castling_masks[to];
        }

        self.move_piece(piece, from, to);

        for side in [us, them] {
            self.attacks_bb[side] = self.attacks_bb(side);
            self.pinned_bb[side] = self.pinned_bb(side);
        }

        if type_of_piece(piece) == PieceType::PAWN && distance(from, to) == 2 {
            new_state.en_passant_square = match us {
                Sides::WHITE => from + 8,
                Sides::BLACK => from - 8,
                _ => panic!("Invalid side"),
            };
        } else {
            new_state.en_passant_square = 0;
        }

        self.side_to_move = them;
        new_state.captured_piece = captured;
        self.states.push(new_state);
    }

    pub fn undo_move(&mut self, mv: Move) {
        assert!(mv.is_ok());

        self.side_to_move = self.side_to_move ^ 1;
        let us: Side = self.side_to_move;
        let them: Side = self.side_to_move ^ 1;
        let from: Square = mv.from_sq();
        let to: Square = mv.to_sq();
        let mut piece: Piece = self.piece_on(to);
        let move_type: MoveType = mv.type_of();
        let last_state: StateInfo = self.states.pop().unwrap();

        assert!(self.board[from] == PieceType::NONE || move_type == MoveTypes::CASTLING);
        assert!(type_of_piece(last_state.captured_piece) != PieceType::KING);

        if move_type == MoveTypes::PROMOTION {
            assert!(type_of_piece(piece) == mv.promotion_type());
            assert!(type_of_piece(piece) >= PieceType::KNIGHT && type_of_piece(piece) < PieceType::KING);

            self.remove_piece(piece, to);
            piece = make_piece(us, PieceType::PAWN);
            self.put_piece(piece, to);
        }

        for side in [us, them] {
            self.attacks_bb[side] = self.attacks_bb(side);
            self.pinned_bb[side] = self.pinned_bb(side);
        }

        if move_type == MoveTypes::CASTLING {
            // TODO: Undo castling
        } else {
            self.move_piece(piece, to, from);
            if last_state.captured_piece != PieceType::NONE {
                if move_type == MoveTypes::EN_PASSANT {
                    let captured_square: Square = (to as isize - pawn_push(us)) as usize;
                    self.put_piece(last_state.captured_piece, captured_square);
                } else {
                    self.put_piece(last_state.captured_piece, to);
                }
            }
        }
    }

    pub fn piece_on(&self, square: Square) -> Piece {
        assert!(is_ok(square));
        return self.board[square];
    }

    fn put_piece(&mut self, piece: Piece, square: Square) {
        let bb: Bitboard = square_bb(square);
        let side = color_of_piece(piece);

        self.board[square] = piece;
        self.by_type_bb[side][type_of_piece(piece)] |= bb;
        self.by_type_bb[Sides::BOTH][type_of_piece(piece)] |= bb;
        self.by_color_bb[side] |= bb;
        self.by_color_bb[Sides::BOTH] |= bb;
    }

    fn remove_piece(&mut self, piece: Piece, square: Square) {
        let bb: Bitboard = square_bb(square);
        let side = color_of_piece(piece);

        self.board[square] = PieceType::NONE;
        self.by_type_bb[side][type_of_piece(piece)] &= !bb;
        self.by_type_bb[Sides::BOTH][type_of_piece(piece)] &= !bb;
        self.by_color_bb[side] &= !bb;
        self.by_color_bb[Sides::BOTH] &= !bb;
    }

    // This function is only for moving and does not handle captures
    fn move_piece(&mut self, piece: Piece, from: Square, to: Square) {
        assert!(is_ok(from));
        assert!(is_ok(to));
        assert!(self.board[from] == piece);
        assert!(self.board[to] == PieceType::NONE);

        let bb_from: Bitboard = square_bb(from);
        let bb_to: Bitboard = square_bb(to);
        let side: Side = color_of_piece(piece);

        self.board[from] = PieceType::NONE;
        self.board[to] = piece;
        self.by_type_bb[side][type_of_piece(piece)] ^= bb_from ^ bb_to;
        self.by_type_bb[Sides::BOTH][type_of_piece(piece)] ^= bb_from ^ bb_to;
        self.by_color_bb[side] ^= bb_from ^ bb_to;
        self.by_color_bb[Sides::BOTH] ^= bb_from ^ bb_to;
    }

    fn castle(&self, side: CastlingRight) {
        assert!(side == Sides::WHITE || side == Sides::BLACK);

        // TODO: Handle castling
    }

    pub fn checkers(&self, defending_side: Side) -> Vec<Square> {
        assert!(defending_side == Sides::WHITE || defending_side == Sides::BLACK);

        let mut checkers: Vec<Square> = Vec::new();
        let mut attackers_bb: Bitboard = self.attacks_bb[defending_side ^ 1];
        let kind_bb: Bitboard = self.by_type_bb[defending_side][PieceType::KING];

        while attackers_bb != EMPTY {
            let square: Square = bits::pop(&mut attackers_bb);
            let attack_bb: Bitboard =
                self.bitboards
                    .attack_bb(self.piece_on(square), square, self.by_color_bb[Sides::BOTH]);

            if attack_bb & kind_bb != EMPTY {
                checkers.push(square);
            }
        }

        return checkers;
    }

    fn attacks_bb(&self, side: Side) -> Bitboard {
        let mut attacks_bb: Bitboard = EMPTY;
        let mut opponents: Bitboard = self.by_color_bb[side]; // FIXME: Take out pinned pieces

        while opponents != EMPTY {
            let square: Square = bits::pop(&mut opponents);

            attacks_bb |= self
                .bitboards
                .attack_bb(self.piece_on(square), square, self.by_color_bb[Sides::BOTH]);
        }

        return attacks_bb;
    }

    fn pinned_bb(&self, side: Side) -> Bitboard {
        assert!(side == Sides::WHITE || side == Sides::BLACK);

        let mut pinned_bb: Bitboard = EMPTY;
        let king: Square = bits::lsb(self.by_type_bb[side][PieceType::KING]);
        let mut attackers_bb: Bitboard = self.by_color_bb[side ^ 1];

        while attackers_bb != EMPTY {
            let square: Square = bits::pop(&mut attackers_bb);
            let line_bb: Bitboard = self.bitboards.line_bb[square][king];
            let aligned_pieces_bb: Bitboard = self.by_color_bb[side] & line_bb;

            if aligned_pieces_bb.count_ones() == 1 {
                pinned_bb |= aligned_pieces_bb;
            }
        }

        return pinned_bb;
    }

    pub fn legal(&self, mv: Move) -> bool {
        assert!(mv.is_ok());

        let us: Side = self.side_to_move;
        let them: Side = us ^ 1;
        let from: Square = mv.from_sq();
        let to: Square = mv.to_sq();
        let piece: Piece = self.piece_on(from);
        let move_type = mv.type_of();

        assert!(color_of_piece(piece) == us);

        // En passant captures are a tricky special case. Because they are rather
        // uncommon, we do it simply by testing whether the king is attacked after
        // the move is made.
        if move_type == MoveTypes::EN_PASSANT {
            let king_bb: Bitboard = self.by_type_bb[us][PieceType::KING];
            let captured_square: Square = (to as isize - pawn_push(us)) as usize;
            let occupied =
                (self.by_color_bb[Sides::BOTH] ^ square_bb(from) ^ square_bb(captured_square)) | square_bb(to);
            let mut attackers = self.by_type_bb[them][PieceType::ROOK]
                | self.by_type_bb[them][PieceType::QUEEN]
                | self.by_type_bb[them][PieceType::BISHOP];

            assert!(self.piece_on(captured_square) == make_piece(them, PieceType::PAWN));
            assert!(self.piece_on(to) == PieceType::NONE);
            assert!(self.piece_on(from) == make_piece(us, PieceType::PAWN));

            while attackers != EMPTY {
                let attacker_square: Square = bits::pop(&mut attackers);
                let attack_bb: Bitboard =
                    self.bitboards
                        .attack_bb(self.piece_on(attacker_square), attacker_square, occupied);

                if attack_bb & king_bb != EMPTY {
                    return false;
                }
            }

            return true;
        }

        // Castling moves generation does not check if the castling path is clear of
        // enemy attacks, it is delayed at a later time: now!
        if move_type == MoveTypes::CASTLING {
            // TODO: Handle castling legality

            return true;
        }

        // If the moving piece is a king, check whether the destination square is
        // attacked by the opponent.
        if type_of_piece(piece) == PieceType::KING {
            return self.attacks_bb[them] & square_bb(to) == EMPTY;
        }

        // A non-king move is legal if and only if it is not pinned or it
        // is moving along the ray towards or away from the king.
        return self.pinned_bb[us] & square_bb(from) == EMPTY
            || self
                .bitboards
                .aligned(to, from, bits::lsb(self.by_type_bb[us][PieceType::KING]));
    }

    fn castling_masks() -> [usize; NrOf::SQUARES] {
        let mut masks: [usize; NrOf::SQUARES] = [0; NrOf::SQUARES];

        masks[square_of(0, 0)] = CastlingRights::WHITE_QUEENSIDE;
        masks[square_of(7, 0)] = CastlingRights::WHITE_KINGSIDE;
        masks[square_of(0, 7)] = CastlingRights::BLACK_QUEENSIDE;
        masks[square_of(7, 7)] = CastlingRights::BLACK_KINGSIDE;
        masks[square_of(4, 0)] = CastlingRights::WHITE;
        masks[square_of(4, 7)] = CastlingRights::BLACK;

        return masks;
    }
}
