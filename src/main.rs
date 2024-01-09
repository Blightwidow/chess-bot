mod bitboards;
mod defs;
mod misc;
mod movegen;
mod position;
mod search;
mod uci;

use crate::{bitboards::Bitboards, movegen::Movegen, position::Position, search::Search, uci::UCI};

fn main() {
    println!("Oxide v0.1.0 by Theo Dammaretz");

    // FIXME: I feel like using 2 bitboards is a bit overkill
    //        but I could not find a way to share it between
    //        the movegen and the position without the compiler
    //        complaining about multiple mutable borrows and
    //        lifecycle issues.
    let movegen = Movegen::new(Bitboards::new());
    let position = Position::new(Bitboards::new());
    let mut search = Search::new(position, movegen);
    // let eval = eval::Eval::new();

    UCI::main_loop(&mut search);
}
