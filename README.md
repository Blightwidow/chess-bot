# Oxide chess bot

A simple Rust chess engine compatible with the [UCI protocol](https://en.wikipedia.org/wiki/Universal_Chess_Interface). It does not come with a GUI. You can dowload a a separate one like [Cute Chess](https://cutechess.com/).

## Usage

You can directly form source

```
cargo run -r -- <command>
```

or build and then run the executable

```
cargo build -r
./taget/release/chessbot <command>
```

## Internal implementation

### Board representation

- Magic bitboards
- Bitboards with Little Endian Rank-File mapping
- 8x8 Board

### Search

- Negamax
- Iterative deepening
- Aspiration window
- Move ordering
    - SEE

### Evaluation

- Centipawn scaling
- Tapered piece square table
- Grain of 1/256 pawn value

## Acknowledgements

- An amazing thanks to @mvanthoor for his work on [Rustic](https://github.com/mvanthoor/rustic) that helped me understand a lot of concepts in Rust.
- Also a big part of my way of thinking was influenced by [Stockfish](https://stockfishchess.org/). It was also a great tool to debug my code.
