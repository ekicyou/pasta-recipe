use crate::lexer;

struct Token<'a> {
    start: usize,
    token: lexer::Token<'a>,
    text: &'a str,
    end: usize,
}
