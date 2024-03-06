mod benchmark;
mod bitboards;
mod defs;
mod evaluate;
mod misc;
mod movegen;
mod position;
mod search;
mod uci;
mod time;

use std::rc::Rc;

use crate::{bitboards::Bitboards, evaluate::Eval, movegen::Movegen, position::Position, search::Search, uci::UCI};

fn main() {
    println!("Oxide v0.1.0 by Theo Dammaretz");

    let bitboards = Rc::new(Bitboards::new());
    let movegen = Movegen::new(Rc::clone(&bitboards));
    let position = Position::new(Rc::clone(&bitboards));
    let eval = Eval::new();
    let mut search = Search::new(position, movegen, eval);

    UCI::main_loop(&mut search);
}
