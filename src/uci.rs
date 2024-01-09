use crate::search::{defs::SearchLimits, Search};

pub struct UCI {}

impl UCI {
    pub fn main_loop(search: &mut Search) {
        // Handle stream

        let mut args = std::env::args();
        // let stream =
        let token = args.nth(1);

        if token.is_none() {
            // UCI::wait_for_command();
        } else {
            let cmd = token.unwrap();
            match cmd.as_str() {
                "go" => UCI::go(search),
                "help" => UCI::help(),
                _ => println!("Unknown command: {}. Type help for more information", cmd),
            }
        }
    }

    fn go(search: &mut Search) {
        let token = std::env::args().nth(2).unwrap();
        let mut limits = SearchLimits::new();

        match token.as_str() {
            "perft" => {
                let depth_paramater = std::env::args().nth(3);
                limits.perft = depth_paramater.unwrap().parse::<usize>().unwrap_or(1);
            }
            _ => println!("Unknown command: {}. Type help for more information", token),
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
