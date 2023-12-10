use crate::lexer;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
struct Token<'a> {
    start: usize,
    token: lexer::Token<'a>,
    text: &'a str,
    end: usize,
}
