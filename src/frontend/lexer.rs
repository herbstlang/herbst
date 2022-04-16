use super::token::Token;
use super::token::TokenType;
use super::error::LexError;

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: u32,
    line_offset_start: u32,
    line_offset_end: u32,
    had_error: bool,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn from_source(source: &str) -> Lexer {
        Lexer {
            source: String::from(source),
            start: 0,
            current: 0,
            line: 1,
            line_offset_start: 1,
            line_offset_end: 1,
            had_error: false,
            tokens: vec![]
        }
    }

    fn create_token(&mut self, ttype: TokenType) {
        self.tokens.push(Token::init(ttype, &self.source[self.start..self.current], self.line, self.line_offset_start, self.line_offset_end))
    }

    

    pub fn print(&self) {
        println!("source:\n{}\n\nstart: {}\ncurrent: {}\nline: {}\nhad_error: {}", self.source, self.start, self.current, self.line, self.had_error);
    }

    fn advance(&mut self) {
        self.current += 1;
        self.line_offset_end += 1;
    }

    fn lex_error(&mut self, message: &str) {
        eprintln!("{}", message);
        self.had_error = true;
    }

    fn peek(&self, n: usize) -> char {
        self.source.chars().nth(self.current + n).unwrap()
    }

    fn match_next(&mut self, c: char) -> bool {
        if self.at_end() { return false; }
        
        if self.peek(0) == c {
            self.advance();
            true
        } else { false }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.line_offset_start = self.line_offset_end;
            self.scan_token();
        }

        self.tokens.push(Token::init(TokenType::EndOfFile, "", self.line, self.line_offset_start + 1, self.line_offset_end));
        self.tokens.to_vec()
    }

    fn consume_multiple(&mut self, condition_fn: fn(char) -> bool) {
        while !self.at_end() && condition_fn(self.peek(0)) {
            self.advance();
        }
    }

    fn report_error(&mut self, message: &str) {
        self.had_error = true;
        let err = LexError::at(self.line, self.line_offset_start, self.line_offset_end, message);
        err.report();
    }

    pub fn scan_token(&mut self) {
        let c = self.peek(0);
        self.advance();
    
        match c {
            '{' => self.create_token(TokenType::LBrace),
            '}' => self.create_token(TokenType::RBrace),
            '(' => self.create_token(TokenType::LParen),
            ')' => self.create_token(TokenType::RParen),
            '[' => self.create_token(TokenType::LBracket),
            ']' => self.create_token(TokenType::RBracket),
            '*' => self.create_token(TokenType::Star),
            '%' => self.create_token(TokenType::Percent),
            '/' => self.create_token(TokenType::Slash),
            '=' => self.create_token(TokenType::PoundSign),
            ',' => self.create_token(TokenType::Comma),
            '@' => self.create_token(TokenType::At),
            '#' => self.create_token(TokenType::PoundSign),
            ':' => {
                if self.match_next(':') {
                    if self.match_next('>') {
                        self.create_token(TokenType::ColonColonArrow);
                    } else {
                        self.create_token(TokenType::ColonColon);
                    }
                } else {
                    self.create_token(TokenType::Colon);
                }
            }
            '+' => {
                if self.match_next('+') {
                    self.create_token(TokenType::PlusPlus);
                } else {
                    self.create_token(TokenType::Plus);
                }
            },
            '-' => {
                if self.match_next('-') {
                    //comment
                    while self.peek(0) != '\n' {
                        self.advance();
                    }
                } else {
                    self.create_token(TokenType::Minus);
                }
            },
            '!' => {
                if self.match_next('=') {
                    self.create_token(TokenType::BangEquals);
                } else {
                    self.create_token(TokenType::Bang);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.create_token(TokenType::LessEquals);
                } else {
                    self.create_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.create_token(TokenType::GreaterEquals);
                } else {
                    self.create_token(TokenType::Greater);
                }
            }
            '"' => {
                while! self.at_end() && self.peek(0) != '"' {
                    self.advance();
                }

                if self.match_next('"') {
                    self.create_token(TokenType::String);
                } else {
                    self.report_error("reached end of file, expected: closing '\"' after string literal")
                }
            }
            '\n' => {
                self.line += 1;
                self.line_offset_start = 1;
                self.line_offset_end = 1;
                self.create_token(TokenType::Newline);
            }
            ' ' | '\t' | '\r' => {}
            _ => {
                if is_alpha(c) {
                    self.consume_multiple(is_alphanumeric);
                    self.create_token(TokenType::Identifier);
                } else if is_numeric(c) {
                    self.consume_multiple(is_numeric);
                    if self.match_next('.') {
                        self.consume_multiple(is_numeric);
                        self.create_token(TokenType::Double);
                    } else {
                        self.create_token(TokenType::Integer);
                    }
                } else {
                    self.report_error(&format!("unexpected character: '{}'", c));
                }
            }
        };
    }
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' ||
    c >= 'A' && c <= 'Z' ||
    c == '_'
}

fn is_numeric(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_numeric(c)
}