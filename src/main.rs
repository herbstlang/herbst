mod backend;
mod frontend;
use frontend::Lexer;
use frontend::Parser;

use std::env;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl();
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

fn repl() {
    println!("herbst repl\n");
    loop {
        print!("• ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::from_source(&input);
        let mut parser = Parser::from_tokens(lexer.lex());
        parser.print();
    }
}