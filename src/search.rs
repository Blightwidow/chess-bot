pub mod defs;

use crate::{
    movegen::{defs::Move, Movegen},
    position::Position,
};

use self::defs::*;

pub struct Search {
    position: Position,
    movegen: Movegen,
}

impl Search {
    pub fn new(position: Position, movegen: Movegen) -> Search {
        Self {
            position: position,
            movegen: movegen,
        }
    }

    pub fn run(&mut self, limits: SearchLimits) {
        self.position.set(limits.fen);

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
    use super::*;
    use crate::bitboards::Bitboards;

    #[test]
    fn test_perft_startpos() {
        let movegen = Movegen::new(Bitboards::new());
        let position = Position::new(Bitboards::new());
        let mut search = Search::new(position, movegen);

        assert_eq!(search.perft(1), 20);
        assert_eq!(search.perft(2), 400);
        assert_eq!(search.perft(3), 8902);
        assert_eq!(search.perft(4), 197281);
        assert_eq!(search.perft(5), 4865609);
    }

    #[test]
    fn test_perft_debug() {
        let movegen = Movegen::new(Bitboards::new());
        let position = Position::new(Bitboards::new());
        let mut search = Search::new(position, movegen);

        assert_eq!(search.perft(1), 20);
        assert_eq!(search.perft(2), 400);
        assert_eq!(search.perft(3), 8902);
        assert_eq!(search.perft(4), 197281);
        assert_eq!(search.perft(5), 4865609);
    }
}
