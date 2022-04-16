mod backend;
mod frontend;
use frontend::Lexer;
use frontend::Parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("repl");
    } else if args.len() == 3 {
        println!("{} {}", args[1], args[2]);
    } else {
        for i in 0..45 {
            print!("-");
        }
        println!();
        println!("usage:\n\nrepl\n  • herbst\n\ncompile or interpret\n  • herbst [interpret/compile] [file address]");
        for i in 0..45 {
            print!("-");
        }
        println!();
    }
}
