#[derive(Copy, Clone, std::fmt::Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,
    LAngleBracket, RAngleBracket,
    Slash, Star, Plus, Minus, Percent,
    PlusPlus, Colon, ColonColon,ColonColonArrow,
    Comma, Integer, Double, String, Char,
    Bang, BangEquals, Equals, PoundSign,
    At, Arrow,
    Less, LessEquals, Greater, GreaterEquals,

    Newline,
    EndOfFile
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub range_start: u32,
    pub range_end: u32
}

impl Token {
    pub fn init(ttype: TokenType, lexeme: &str, line: u32, range_start: u32, range_end: u32) -> Token {
        Token {
            token_type: ttype,
            lexeme: String::from(lexeme),
            line: line,
            range_start: range_start,
            range_end: range_end
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let ttype = self.token_type;
        let lexeme = &self.lexeme;
        let l = self.line;
        let rs = self.range_start;
        let re = self.range_end;


        format!("{ttype}: '{lexeme}' {l}[{rs}:{re}]")
    }
}