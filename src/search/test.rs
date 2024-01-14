#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{
        bitboards::Bitboards,
        movegen::Movegen,
        position::Position,
        search::{defs::FEN_START_POSITION, Search}, evaluate::Eval,
    };

    #[test]
    fn perft_startpos() {
        let bitboards = Rc::new(Bitboards::new());
        let movegen = Movegen::new(Rc::clone(&bitboards));
        let position = Position::new(Rc::clone(&bitboards));
        let mut search = Search::new(position, movegen, Eval::new());

        search.position.set(FEN_START_POSITION.to_string());

        assert_eq!(search.perft(1, true), 20);
        assert_eq!(search.perft(2, true), 400);
        assert_eq!(search.perft(3, true), 8902);
        assert_eq!(search.perft(4, true), 197281);
        assert_eq!(search.perft(5, true), 4865609);
    }

    #[test]
    fn perft_kiwipete() {
        let bitboards = Rc::new(Bitboards::new());
        let movegen = Movegen::new(Rc::clone(&bitboards));
        let position = Position::new(Rc::clone(&bitboards));
        let mut search = Search::new(position, movegen, Eval::new());

        search
            .position
            .set("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string());

        assert_eq!(search.perft(1, true), 48);
        assert_eq!(search.perft(2, true), 2039);
        assert_eq!(search.perft(3, true), 97862);
        assert_eq!(search.perft(4, true), 4085603);
        assert_eq!(search.perft(5, true), 193690690);
    }

    #[test]
    fn perft_edwards() {
        let bitboards = Rc::new(Bitboards::new());
        let movegen = Movegen::new(Rc::clone(&bitboards));
        let position = Position::new(Rc::clone(&bitboards));
        let mut search = Search::new(position, movegen, Eval::new());

        search
            .position
            .set("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string());

        assert_eq!(search.perft(1, true), 44);
        assert_eq!(search.perft(2, true), 1486);
        assert_eq!(search.perft(3, true), 62379);
        assert_eq!(search.perft(4, true), 2103487);
        assert_eq!(search.perft(5, true), 89941194);
    }
}
