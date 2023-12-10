use crate::lexer;
use thiserror;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
struct Token<'a> {
    start: usize,
    token: lexer::Token<'a>,
    text: &'a str,
    end: usize,
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Hash)]
pub enum ParserError {
    #[error("lexer error {0}")]
    Lexer(#[from] lexer::LexerError),
}
