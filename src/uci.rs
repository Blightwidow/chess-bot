use crate::search::{
    defs::{SearchLimits, FEN_START_POSITION},
    Search,
};

pub struct UCI {}

impl UCI {
    pub fn main_loop(search: &mut Search) {
        // Handle stream

        let argc = std::env::args().len();
        let mut buffer: String = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

        loop {
            if argc == 1 {
                let read_result = std::io::stdin().read_line(&mut buffer);

                if read_result.is_err() {
                    buffer = "quit".to_string();
                }
            }

            let cmd: String = buffer.clone();
            let mut args: std::str::SplitWhitespace<'_> = cmd.trim().split_whitespace();
            let mut token = args.next().unwrap_or("");
            buffer.clear();

            if token == "uci" {
                println!("id name Oxide");
                println!("id author Theo Dammaretz");
                println!("uciok");
            } else if token == "xboard" {
                println!("This engine does not support the xboard protocol.");
                token = "quit";
            } else if token == "position" {
                UCI::position(search, &mut args);
            } else if token == "go" {
                UCI::go(search, &mut args);
            } else if token == "help" {
                UCI::help();
            } else if token != "" && token.chars().nth(0).unwrap_or_default() != '#' {
                println!("Unknown command: {}. Type help for more information", token);
            }

            if token == "quit" || argc > 1 {
                break;
            }
        }
    }

    fn position(search: &mut Search, args: &mut std::str::SplitWhitespace<'_>) {
        let token = args.next().unwrap_or("");

        if token == "startpos" {
            search.position.set(FEN_START_POSITION.to_string());
        } else if token == "fen" {
            let fen = args.collect::<Vec<&str>>().join(" ");
            search.position.set(fen);
        } else {
            println!("Unknown position command: {}. Type help for more information", token);
        }
    }

    fn go(search: &mut Search, args: &mut std::str::SplitWhitespace<'_>) {
        let mut limits = SearchLimits::new();
        let mut token = args.next().unwrap_or("");

        while token != "" {
            match token {
                "perft" => {
                    limits.perft = args.next().unwrap_or("1").parse::<usize>().unwrap_or(1);
                }
                _ => println!("Unknown command: {}. Type help for more information", token),
            }

            token = args.next().unwrap_or("");
        }

        let _ = search.run(limits);
    }

    fn help() {
        println!("");
        println!("Oxide is a simple chess engine I built as a learning project.");
        println!("It is UCI compatible and can be used with any UCI compatible GUI.");
        println!("While not very strong yet but I am working on it and hoping to achieve a rating of 2000+.");
        println!("");
    }
}
