use crate::defs::{Bitboard, NrOf, Square};

#[derive(Debug, Default, Copy, Clone)]
pub struct Magic {
    pub mask: Bitboard, // Occupancy mask for square (exclude the edges).
    pub shift: u8,      // Shift to get index.
    pub offset: u64,    // Offset to add to index.
    pub number: u64,    // Magic number.
}

impl Magic {
    pub fn get_index(&self, occupied: Bitboard) -> usize {
        let blockerboard: u64 = occupied & self.mask;
        return ((blockerboard.wrapping_mul(self.number) >> self.shift) + self.offset) as usize;
    }
}

pub const EMPTY: Bitboard = 0;
pub const FULL: Bitboard = 0xffffffffffffffff;

pub const FILE_ABB: Bitboard = 0x0101010101010101;
pub const FILE_BBB: Bitboard = FILE_ABB << 1;
pub const FILE_CBB: Bitboard = FILE_ABB << 2;
pub const FILE_DBB: Bitboard = FILE_ABB << 3;
pub const FILE_EBB: Bitboard = FILE_ABB << 4;
pub const FILE_FBB: Bitboard = FILE_ABB << 5;
pub const FILE_GBB: Bitboard = FILE_ABB << 6;
pub const FILE_HBB: Bitboard = FILE_ABB << 7;

pub const RANK_1BB: Bitboard = 0xff;
pub const RANK_2BB: Bitboard = RANK_1BB << (8 * 1);
pub const RANK_3BB: Bitboard = RANK_1BB << (8 * 2);
pub const RANK_4BB: Bitboard = RANK_1BB << (8 * 3);
pub const RANK_5BB: Bitboard = RANK_1BB << (8 * 4);
pub const RANK_6BB: Bitboard = RANK_1BB << (8 * 5);
pub const RANK_7BB: Bitboard = RANK_1BB << (8 * 6);
pub const RANK_8BB: Bitboard = RANK_1BB << (8 * 7);

pub fn file_bb(square: Square) -> Bitboard {
    return FILE_ABB << (square % 8);
}

pub fn rank_bb(square: Square) -> Bitboard {
    return RANK_1BB << (square / 8) * 8;
}

pub const ROOK_TABLE_SIZE: usize = 102400;
pub const BISHOP_TABLE_SIZE: usize = 5248;

#[rustfmt::skip]
pub const ROOK_MAGIC_NUMBERS: [Bitboard; NrOf::SQUARES] = [
    4647714953174712000, 6935578610791105000, 36046391428521980,
    2341880604493353000, 9295447257438683000, 144132831936086530,
    144194492650750460, 9835872860466323000, 292874724007002100,
    18155410951528452, 4611826824643617000, 13980862230930473000,
    146929980801294400, 2378041495364174000, 2306124694644457500,
    2594636346123371000, 36170084265312260, 11836744600603673000,
    432768326716891140, 282575028355592, 2613214235685292000,
    9511743700317768000, 10448373125901455000, 9889007588868164,
    588360770948005900, 216207967559634940, 9664727551264686000,
    54329210286116860, 144679280491627550, 2323870604010324000,
    4035230780864594400, 1153784114428527600, 756921671633342600,
    10380797278532141000, 141493411012609, 52781272539392,
    9336103194532514000, 5226990523708868000, 18298106936312080,
    112643604928790660, 72761428582367230, 1196956114501635,
    2900599772446523400, 47578067425591420, 9232660871208042000,
    1126312500789512, 216216775732494340, 2615184181499855000,
    306245324553408640, 211110550601984, 585608826486522000,
    2533863738835200, 6989595426965979000, 288371122231312500, 
    288371221014380700, 72061995314119170, 290519836386198300,   
    2377923695147484000, 2378465959462732000, 175960516198405, 
    6053119408525542000, 10088907607420054000, 14134828914069252000, 
    2329821313415448600
];

#[rustfmt::skip]
pub const BISHOP_MAGIC_NUMBERS: [Bitboard; NrOf::SQUARES] = [
    306245878803398700, 20856369787830280, 10137514459299840, 
    434105336882792450, 19144834174424064, 18297283309236224,  
    3459892720194028000, 288516262127010940, 1729386693746033700, 
    600122612034592900, 1143505518919696, 4629773027687663000,  
    18018865946558980, 4611739379638469000, 9237451285665686000,  
    3450409290064898, 722053821580331000, 884591599030920,   
    4503634582712338, 5764891231414977000, 1130306830599168,   
    1153625226542651400, 1154047509774012400, 2310497383672254500,  
    38315918780041490, 571748730864656, 90358968572101120, 
    1168147541695479800, 1171781427559219200, 722830081520697600,   
    18192519746028030, 37174770531961340, 3481295848108786000,  
    1157743550428350500, 23151488676134976, 577692239686336600,   
    5651507216384256, 9369457833235515000, 184704760438393100,  
    9367773099564532000, 75439984953265150, 82191827205358110,  
    4616755317332641000, 2308095914041934000, 2315978310116443000,   
    9241395920816308000, 19141544527266816, 85568947524477000,  
    289921566840463360, 581036956478608400, 74309608886240260, 
    4415772168449, 2328440332448172000, 4612817553398498000,   
    5026034933117764000, 3386575337586692, 600367985854468,   
    5189066254811927000, 4574695328850962, 9588163611151567000, 
    9587785418156552, 617942992552516, 1152930438211960800,   
    729605285156503600
];