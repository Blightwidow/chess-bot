pub mod defs;
mod test;

use std::{ops::Add, time};

use crate::{
    evaluate::{defs::PAWN_UNIT, Eval},
    movegen::{defs::Move, Movegen},
    position::Position,
};

use self::defs::*;

pub struct Search {
    pub position: Position,
    pub movegen: Movegen,
    eval: Eval,
    pub best_move: Move,
    nodes_searched: usize,
}

impl Search {
    pub fn new(position: Position, movegen: Movegen, eval: Eval) -> Self {
        let mut search = Self {
            position,
            movegen,
            best_move: Move::none(),
            nodes_searched: 0,
            eval,
        };
        search.position.set(FEN_START_POSITION.to_string());

        return search;
    }

    pub fn run(&mut self, limits: SearchLimits) {
        self.nodes_searched = 0;
        self.best_move = Move::none();

        if limits.perft > 0 {
            let nodes = self.perft(limits.perft, true);
            println!("\nNodes searched: {}\n", nodes);
            return;
        }

        if limits.depth > 0 {
            self.iterative_deepening(limits);
        } else {
            let score = self.eval.evaluate(&self.position);
            println!("info depth 0 score {}", score);
        }
    }

    fn iterative_deepening(&mut self, limits: SearchLimits) {
        let root_moves = self.movegen.legal_moves(&self.position);
        if root_moves.len() == 0 {
            let score = match self.position.checkers(self.position.side_to_move).len() {
                0 => VALUE_DRAW,
                _ => -VALUE_MATE,
            };
            println!("info depth 0 score cp {}", score);
            println!("bestmove 0000");
            return;
        } else if root_moves.len() == 1 {
            println!("bestmove {:?}", root_moves[0]);
            return;
        }

        let mut current_depth: usize = 1;
        let mut alpha: isize = -VALUE_INFINITE;
        let mut beta: isize = VALUE_INFINITE;
        let mut delta: isize = PAWN_UNIT / 4;
        let mut best_score: isize;
        let think_time =
            limits.time(self.position.side_to_move) / (40 - self.position.states.last().unwrap().game_ply * 2).max(2);
        let start_time = time::Instant::now();
        let cutoff = start_time.add(time::Duration::from_millis(think_time as u64));

        while current_depth <= limits.depth {
            self.nodes_searched = 0;

            loop {
                let score = self.negamax(current_depth, alpha, beta, true, cutoff);

                if score.abs() == VALUE_NONE {
                    println!("bestmove {:?}", self.best_move);
                    return;
                }

                if score >= beta {
                    beta = (beta + delta).min(VALUE_INFINITE);
                } else if score <= alpha {
                    alpha = (score - delta).max(-VALUE_INFINITE);
                } else {
                    best_score = score;
                    println!(
                        "info depth {} seldepth {} score cp {} nodes {} time {} pv {:?}",
                        current_depth,
                        current_depth,
                        best_score,
                        self.nodes_searched,
                        start_time.elapsed().as_millis(),
                        self.best_move
                    );
                    break;
                }

                delta += delta;
            }

            current_depth += 1;
        }
    }

    fn negamax(&mut self, depth: usize, alpha: isize, beta: isize, root: bool, cutoff: time::Instant) -> isize {
        if time::Instant::now() > cutoff {
            return VALUE_NONE;
        }

        if depth == 0 {
            return self.eval.evaluate(&self.position);
        }

        let mut alpha = alpha;
        let mut movelist = self.movegen.legal_moves(&self.position);
        self.eval.order_moves(&self.position, &mut movelist);
        self.nodes_searched += movelist.len();

        for mv in movelist {
            self.position.do_move(mv);
            let score = -self.negamax(depth - 1, -beta, -alpha, false, cutoff);
            self.position.undo_move(mv);

            if score.abs() == VALUE_NONE {
                return VALUE_NONE;
            }

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;

                if root {
                    self.best_move = mv;
                }
            }
        }

        return alpha;
    }

    fn perft(&mut self, depth: usize, root: bool) -> u128 {
        let mut count: u128;
        let mut nodes: u128 = 0;
        let leaf: bool = depth == 2;
        let moves: Vec<Move> = self.movegen.legal_moves(&self.position);

        for mv in moves.iter() {
            if depth <= 1 {
                count = 1;
                nodes += 1;
            } else {
                self.position.do_move(*mv);
                count = match leaf {
                    true => self.movegen.legal_moves(&self.position).len() as u128,
                    false => self.perft(depth - 1, false),
                };
                nodes += count;
                self.position.undo_move(*mv);
            }

            if root {
                println!("{:?}: {}", mv, count);
            }
        }

        return nodes;
    }
}
