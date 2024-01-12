pub mod defs;

use crate::{
    movegen::{defs::Move, Movegen},
    position::Position,
};

use self::defs::*;

pub struct Search {
    pub position: Position,
    pub movegen: Movegen,
}

impl Search {
    pub fn new(position: Position, movegen: Movegen) -> Self {
        let mut search = Self { position, movegen };
        search.position.set(FEN_START_POSITION.to_string());

        return search;
    }

    pub fn run(&mut self, limits: SearchLimits) {
        if limits.perft > 0 {
            let count = self.perft(limits.perft);
            println!("\nNodes searched: {}\n", count);
        } else {
            return;
        }
    }

    fn perft(&mut self, depth: usize) -> usize {
        let mut count: usize;
        let mut nodes: usize = 0;
        let leaf: bool = depth == 2;
        let moves: Vec<Move> = self.movegen.legal_moves(&self.position);

        for mv in moves.iter() {
            if depth <= 1 {
                count = 1;
                nodes += 1;
            } else {
                self.position.do_move(*mv);
                count = match leaf {
                    true => self.movegen.legal_moves(&self.position).len(),
                    false => self.perft_recursive(depth - 1),
                };
                nodes += count;
                self.position.undo_move(*mv);
            }

            println!("{:?}: {}", mv, count);
        }

        return nodes;
    }

    fn perft_recursive(&mut self, depth: usize) -> usize {
        let mut nodes: usize = 0;
        let leaf: bool = depth == 2;
        let moves: Vec<Move> = self.movegen.legal_moves(&self.position);

        for mv in moves.iter() {
            self.position.do_move(*mv);
            nodes += match leaf {
                true => self.movegen.legal_moves(&self.position).len(),
                false => self.perft_recursive(depth - 1),
            };
            self.position.undo_move(*mv);
        }

        return nodes;
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;
    use crate::bitboards::Bitboards;

    #[test]
    fn perft_startpos() {
        let bitboards = Rc::new(Bitboards::new());
        let movegen = Movegen::new(Rc::clone(&bitboards));
        let position = Position::new(Rc::clone(&bitboards));
        let mut search = Search::new(position, movegen);

        search.position.set(FEN_START_POSITION.to_string());

        assert_eq!(search.perft(1), 20);
        assert_eq!(search.perft(2), 400);
        assert_eq!(search.perft(3), 8902);
        assert_eq!(search.perft(4), 197281);
        assert_eq!(search.perft(5), 4865609);
    }

    #[test]
    fn perft_kiwipete() {
        let bitboards = Rc::new(Bitboards::new());
        let movegen = Movegen::new(Rc::clone(&bitboards));
        let position = Position::new(Rc::clone(&bitboards));
        let mut search = Search::new(position, movegen);

        search
            .position
            .set("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string());

        assert_eq!(search.perft(1), 48);
        assert_eq!(search.perft(2), 2039);
        assert_eq!(search.perft(3), 97862);
        assert_eq!(search.perft(4), 4085603);
        assert_eq!(search.perft(5), 193690690);
    }

    #[test]
    fn perft_edwards() {
        let bitboards = Rc::new(Bitboards::new());
        let movegen = Movegen::new(Rc::clone(&bitboards));
        let position = Position::new(Rc::clone(&bitboards));
        let mut search = Search::new(position, movegen);

        search
            .position
            .set("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string());

        assert_eq!(search.perft(1), 44);
        assert_eq!(search.perft(2), 1486);
        assert_eq!(search.perft(3), 62379);
        assert_eq!(search.perft(4), 2103487);
        assert_eq!(search.perft(5), 89941194);
    }
}
