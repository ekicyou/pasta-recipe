use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Hash)]
pub enum Token<'a> {
    #[token(":")]
    #[token("：")]
    Colon1,

    #[token("::::")]
    #[token("：：：：")]
    Colon4,

    #[regex(r"\p{XID_Start}\p{XID_Continue}*")]
    Identifier(&'a str),

    #[regex(r"[^ \t\u3000@＠\|｜《》\r\n\p{XID_Start}]+")]
    TextOthers(&'a str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colon4_1() {
        let source = "：：：：";
        let mut lex = Token::lexer(source);
        let mut x = || lex.next();
        let mut y = || x().unwrap().unwrap();

        assert_eq!(y(), Token::Colon4);
        assert_eq!(x(), None);
    }

    #[test]
    fn colon4_2() {
        let source = "：：：：xyz";
        let mut lex = Token::lexer(source);
        let mut x = || lex.next();
        let mut y = || x().unwrap().unwrap();

        assert_eq!(y(), Token::Colon4);
        assert_eq!(y(), Token::Identifier(&"xyz"));
        assert_eq!(x(), None);
    }

    #[test]
    fn colon4_3() {
        let source = "：：：：これ";
        let mut lex = Token::lexer(source);
        let mut x = || lex.next();
        let mut y = || x().unwrap().unwrap();

        assert_eq!(y(), Token::Colon4);
        assert_eq!(y(), Token::Identifier(&"これ"));
        assert_eq!(x(), None);
    }
}
