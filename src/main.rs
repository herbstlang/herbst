mod backend;
mod frontend;
use frontend::Lexer;

fn main() {
    let mut lexer = Lexer::from_source("var #str: \"hahaha\"\nv2 #integer: 5\nv3 #double: 1.256\nv2 :: +1");
    for tok in lexer.lex() {
        println!("{}: {}", tok.token_type, tok.lexeme);
    }
}
