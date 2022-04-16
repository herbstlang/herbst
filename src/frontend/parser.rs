use super::token::Token;
use super::token::TokenType;
use super::ast::Node;
use super::ast::Op;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn from_tokens(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0
        }
    }

    pub fn print(&mut self) {
        println!("STMTS:");
        for stmt in self.parse() {
            println!("{:?}", stmt);
        }
    }

    fn at_end(&self) -> bool {
        self.peek(0).token_type == TokenType::EndOfFile
    }

    fn parse_stmt(&mut self) -> Node {
        self.parse_expr_stmt()
    }

    fn parse_expr_stmt(&mut self) -> Node {
        self.parse_assign_expr()
    }

    fn parse_assign_expr(&mut self) -> Node {
        self.parse_term_expr()
    }

    fn parse_term_expr(&mut self) -> Node {
        let left = self.parse_factor_expr();
        if self.match_next(TokenType::Plus) {
            let right = self.parse_factor_expr();
            return Node::binary(Op::Add, left, right);
        }
        left
    }

    fn parse_factor_expr(&mut self) -> Node {
        let left = self.parse_unary_expr();
        if self.match_next(TokenType::Star) {
            let right = self.parse_unary_expr();
            return Node::binary(Op::Mul, left, right);
        }
        left
    }

    fn parse_unary_expr(&mut self) -> Node {
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Node {
        //self.advance();
        match self.peek(0).token_type {
            TokenType::Integer => {
                self.advance();
                return Node::standalone(&self.peek(0).lexeme)
            },
            TokenType::Identifier => {
                self.advance();
                return Node::standalone(&self.peek(0).lexeme)
            }
            _ => {
                self.advance();
                return Node::standalone("x")
            }
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek(&self, n: usize) -> &Token {
        &self.tokens[self.current + n]
    }

    fn match_next(&mut self, ttype: TokenType) -> bool {
        if self.peek(0).token_type == ttype {
            self.advance(); 
            true
        } else { false }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut stmts: Vec<Node> = vec![];
        while !self.at_end() {
            stmts.push(self.parse_stmt());
        }
        stmts
    }
}