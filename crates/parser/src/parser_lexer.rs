use logos::Logos;
use std::iter::Iterator;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    text: &'input str,
    lexer: logos::Lexer<'input, crate::lexer::Token<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str) -> Lexer<'input> {
        let lexer = crate::lexer::Token::lexer(text);
        Lexer { text, lexer }
    }

    pub fn text(&self) -> &'input str {
        self.text
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<crate::lexer::Token<'input>, usize, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(a) => {
                let span = self.lexer.span();
                Some(Ok((span.start, a, span.end)))
            }
            None => None,
        }
    }
}
