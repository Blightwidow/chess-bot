use crate::defs::NrOf;

pub const GAME_PHASE_INCREMENT: [i16; NrOf::PIECE_TYPES] = [0, 0, 1, 1, 2, 4, 0];
pub const PIECE_VALUES_INITIAL: [i16; NrOf::PIECE_TYPES] = [0, 100, 300, 300, 500, 900, 0];
pub const PIECE_VALUES_MG: [i16; NrOf::PIECE_TYPES] = [0, 82, 337, 365, 477, 1025, 0];
pub const PIECE_VALUES_EG: [i16; NrOf::PIECE_TYPES] = [0, 94, 281, 297, 512, 936, 0];

pub const PIECE_SQUARE_MG_TABLES: [[i16; NrOf::SQUARES]; NrOf::PIECE_TYPES] = [
    [0; NrOf::SQUARES],
    PAWN_MG_TABLE,
    KNIGHT_MG_TABLE,
    BISHOP_MG_TABLE,
    ROOK_MG_TABLE,
    QUEEN_MG_TABLE,
    KING_MG_TABLE,
];
pub const PIECE_SQUARE_EG_TABLES: [[i16; NrOf::SQUARES]; NrOf::PIECE_TYPES] = [
    [0; NrOf::SQUARES],
    PAWN_EG_TABLE,
    KNIGHT_EG_TABLE,
    BISHOP_EG_TABLE,
    ROOK_EG_TABLE,
    QUEEN_EG_TABLE,
    KING_EG_TABLE,
];

#[rustfmt::skip]
const PAWN_MG_TABLE: [i16; NrOf::SQUARES] = [
    0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
    0098, 0134, 0061, 0095, 0068, 0126, 0034, -011,
    -006, 0007, 0026, 0031, 0065, 0056, 0025, -020,
    -014, 0013, 0006, 0021, 0023, 0012, 0017, -023,
    -027, -002, -005, 0012, 0017, 0006, 0010, -025,
    -026, -004, -004, -010, 0003, 0003, 0033, -012,
    -035, -001, -020, -023, -015, 0024, 0038, -022,
    0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
];

#[rustfmt::skip]
const PAWN_EG_TABLE: [i16; NrOf::SQUARES] = [
    0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
    0178, 0173, 0158, 0134, 0147, 0132, 0165, 0187,
    0094, 0100, 0085, 0067, 0056, 0053, 0082, 0084,
    0032, 0024, 0013, 0005, -002, 0004, 0017, 0017,
    0013, 0009, -003, -007, -007, -008, 0003, -001,
    0004, 0007, -006, 0001, 0000, -005, -001, -008,
    0013, 0008, 0008, 0010, 0013, 0000, 0002, -007,
    0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
];

#[rustfmt::skip]
const KNIGHT_MG_TABLE: [i16; NrOf::SQUARES] = [
    -167, -089, -034, -049, 0061, -097, -015, -107,
    -073, -041, 0072, 0036, 0023, 0062, 0007, -017,
    -047, 0060, 0037, 0065, 0084, 0129, 0073, 0044,
    -009, 0017, 0019, 0053, 0037, 0069, 0018, 0022,
    -013, 0004, 0016, 0013, 0028, 0019, 0021, -008,
    -023, -009, 0012, 0010, 0019, 0017, 0025, -016,
    -029, -053, -012, -003, -001, 0018, -014, -019,
    -105, -021, -058, -033, -017, -028, -019, -023,
];

#[rustfmt::skip]
const KNIGHT_EG_TABLE: [i16; NrOf::SQUARES] = [
    -058, -038, -013, -028, -031, -027, -063, -099,
    -025, -008, -025, -002, -009, -025, -024, -052,
    -024, -020, 0010, 0009, -001, -009, -019, -041,
    -017, 0003, 0022, 0022, 0022, 0011, 0008, -018,
    -018, -006, 0016, 0025, 0016, 0017, 0004, -018,
    -023, -003, -001, 0015, 0010, -003, -020, -022,
    -042, -020, -010, -005, -002, -020, -023, -044,
    -029, -051, -023, -015, -022, -018, -050, -064,
];

#[rustfmt::skip]
const BISHOP_MG_TABLE: [i16; NrOf::SQUARES] = [
    -029, 0004, -082, -037, -025, -042, 0007, -008,
    -026, 0016, -018, -013, 0030, 0059, 0018, -047,
    -016, 0037, 0043, 0040, 0035, 0050, 0037, -002,
    -004, 0005, 0019, 0050, 0037, 0037, 0007, -002,
    -006, 0013, 0013, 0026, 0034, 0012, 0010, 0004,
    0000, 0015, 0015, 0015, 0014, 0027, 0018, 0010,
    0004, 0015, 0016, 0000, 0007, 0021, 0033, 0001,
    -033, -003, -014, -021, -013, -012, -039, -021,
];

#[rustfmt::skip]
const BISHOP_EG_TABLE: [i16; NrOf::SQUARES] = [
    -014, -021, -011, -008, -007, -009, -017, -024,
    -008, -004, 0007, -012, -003, -013, -004, -014,
    0002, -008, 0000, -001, -002, 0006, 0000, 0004,
    -003, 0009, 0012, 0009, 0014, 0010, 0003, 0002,
    -006, 0003, 0013, 0019, 0007, 0010, -003, -009,
    -012, -003, 0008, 0010, 0013, 0003, -007, -015,
    -014, -018, -007, -001, 0004, -009, -015, -027,
    -023, -009, -023, -005, -009, -016, -005, -017,
];

#[rustfmt::skip]
const ROOK_MG_TABLE: [i16; NrOf::SQUARES] = [
    0032, 0042, 0032, 0051, 0063, 0009, 0031, 0043,
    0027, 0032, 0058, 0062, 0080, 0067, 0026, 0044,
    -005, 0019, 0026, 0036, 0017, 0045, 0061, 0016,
    -024, -011, 0007, 0026, 0024, 0035, -008, -020,
    -036, -026, -012, -001, 0009, -007, 0006, -023,
    -045, -025, -016, -017, 0003, 0000, -005, -033,
    -044, -016, -020, -009, -001, 0011, -006, -071,
    -019, -013, 0001, 0017, 0016, 0007, -037, -026,
];

#[rustfmt::skip]
const ROOK_EG_TABLE: [i16; NrOf::SQUARES] = [
    0013, 0010, 0018, 0015, 0012, 0012, 0008, 0005,
    0011, 0013, 0013, 0011, -003, 0003, 0008, 0003,
    0007, 0007, 0007, 0005, 0004, -003, -005, -003,
    0004, 0003, 0013, 0001, 0002, 0001, -001, 0002,
    0003, 0005, 0008, 0004, -005, -006, -008, -011,
    -004, 0000, -005, -001, -007, -012, -008, -016,
    -006, -006, 0000, 0002, -009, -009, -011, -003,
    -009, 0002, 0003, -001, -005, -013, 0004, -020,
];

#[rustfmt::skip]
const QUEEN_MG_TABLE: [i16; NrOf::SQUARES] = [
    -028, 0000, 0029, 0012, 0059, 0044, 0043, 0045,
    -024, -039, -005, 0001, -016, 0057, 0028, 0054,
    -013, -017, 0007, 0008, 0029, 0056, 0047, 0057,
    -027, -027, -016, -016, -001, 0017, -002, 0001,
    -009, -026, -009, -010, -002, -004, 0003, -003,
    -014, 0002, -011, -002, -005, 0002, 0014, 0005,
    -035, -008, 0011, 0002, 0008, 0015, -003, 0001,
    -001, -018, -009, 0010, -015, -025, -031, -050,
];

#[rustfmt::skip]
const QUEEN_EG_TABLE: [i16; NrOf::SQUARES] = [
    -009, 0022, 0022, 0027, 0027, 0019, 0010, 0020,
    -017, 0020, 0032, 0041, 0058, 0025, 0030, 0000,
    -020, 0006, 0009, 0049, 0047, 0035, 0019, 0009,
    0003, 0022, 0024, 0045, 0057, 0040, 0057, 0036,
    -018, 0028, 0019, 0047, 0031, 0034, 0039, 0023,
    -016, -027, 0015, 0006, 0009, 0017, 0010, 0005,
    -022, -023, -030, -016, -016, -023, -036, -032,
    -033, -028, -022, -043, -005, -032, -020, -041,
];

#[rustfmt::skip]
const KING_MG_TABLE: [i16; NrOf::SQUARES] = [
    -065, 0023, 0016, -015, -056, -034, 0002, 0013,
    0029, -001, -020, -007, -008, -004, -038, -029,
    -009, 0024, 0002, -016, -020, 0006, 0022, -022,
    -017, -020, -012, -027, -030, -025, -014, -036,
    -049, -001, -027, -039, -046, -044, -033, -051,
    -014, -014, -022, -046, -044, -030, -015, -027,
    0001, 0007, -008, -064, -043, -016, 0009, 0008,
    -015, 0036, 0012, -054, 0008, -028, 0024, 0014,
];

#[rustfmt::skip]
const KING_EG_TABLE: [i16; NrOf::SQUARES] = [
    -074, -035, -018, -018, -011, 0015, 0004, -017,
    -012, 0017, 0014, 0017, 0017, 0038, 0023, 0011,
    0010, 0017, 0023, 0015, 0020, 0045, 0044, 0013,
    -008, 0022, 0024, 0027, 0026, 0033, 0026, 0003,
    -018, -004, 0021, 0024, 0027, 0023, 0009, -011,
    -019, -003, 0011, 0021, 0023, 0016, 0007, -009,
    -027, -011, 0004, 0013, 0014, 0004, -005, -017,
    -053, -034, -021, -011, -028, -014, -024, -043,
];