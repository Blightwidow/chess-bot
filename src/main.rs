mod bitboards;
mod defs;
mod misc;
mod movegen;
mod position;
mod search;
mod uci;

use std::rc::Rc;

use crate::{bitboards::Bitboards, movegen::Movegen, position::Position, search::Search, uci::UCI};

fn main() {
    println!("Oxide v0.1.0 by Theo Dammaretz");

    let bitboards = Rc::new(Bitboards::new());
    let movegen = Movegen::new(Rc::clone(&bitboards));
    let position = Position::new(Rc::clone(&bitboards));
    let mut search = Search::new(position, movegen);
    // let eval = eval::Eval::new();

    UCI::main_loop(&mut search);
}
