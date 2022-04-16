pub struct LexError {
    line: u32,
    span_start: u32,
    span_end: u32,
    message: String
}

impl LexError {
    pub fn at(line: u32, span_start: u32, span_end: u32, message: &str) -> LexError {
        LexError { span_start, span_end, line, message: String::from(message) }
    }
    pub fn report(&self) {
        print_error(&format!("line {} [{}:{}] | error while lexing", self.line, self.span_start, self.span_end - 1,), &format!("{}", self.message));
    }
}

fn print_error(error_type: &str, message: &str) {
    
    eprintln!();
    eprintln!("• \x1b[1;31m{}\n  \x1b[0;31m{}\x1b[0m", error_type, message);
    eprintln!();
} 