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
        let mut expr = self.parse_factor_expr();
        while self.match_next(TokenType::Plus) || self.match_next(TokenType::Minus) {
            let mut op = Op::Add;
            match self.prev().token_type {
                TokenType::Plus => op = Op::Add,
                TokenType::Minus => op = Op::Sub,
                _ => {}
            }

            let right = self.parse_factor_expr();

            expr = Node::binary(op, expr, right);
        }
        expr
    }

    fn parse_factor_expr(&mut self) -> Node {
        let mut expr = self.parse_unary_expr();
        while self.match_next(TokenType::Star) || self.match_next(TokenType::Slash) || self.match_next(TokenType::Percent) {
            let mut op = Op::Add;
            match self.prev().token_type {
                TokenType::Star => op = Op::Mul,
                TokenType::Slash => op = Op::Div,
                TokenType::Percent => op = Op::Mod,
                _ => {}
            }

            let right = self.parse_unary_expr();

            expr = Node::binary(op, expr, right);
        }
        expr
    }

    fn parse_unary_expr(&mut self) -> Node {
        if self.match_next(TokenType::Minus) {
            let mut op = Op::Neg;
            match self.prev().token_type {
                TokenType::Minus => op = Op::Neg,
                _ => {}
            }

            let right = self.parse_primary_expr();

            return Node::unary(op, right);
        }
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Node {
        //self.advance();
        self.match_next(TokenType::Newline);
        if self.match_next(TokenType::Integer) {
            return Node::standalone(&self.prev().lexeme);
        } else if self.match_next(TokenType::Double) {
            return Node::standalone(&self.prev().lexeme);
        } else if self.match_next(TokenType::String) {
            return Node::standalone(&self.prev().lexeme);
        }
        return Node::standalone("temporary");
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek(&self, n: usize) -> &Token {
        &self.tokens[self.current + n]
    }

    fn prev(&self) -> &Token {
        &self.tokens[self.current - 1]
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