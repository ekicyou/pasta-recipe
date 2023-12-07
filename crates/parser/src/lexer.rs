use logos::Logos;

const WIDE_SPACE_STR: &str = "\u{3000}";
const WIDE_SPACE_LEN: usize = WIDE_SPACE_STR.len();

#[derive(Logos, Debug, Clone, PartialEq, Hash)]
pub enum Token<'a> {
    //#[error]
    //Error,
    #[regex(r"(\r)?\n")]
    Newline,

    #[regex(r"//[^\r\n]*")]
    LineComment(&'a str),

    #[token("@/")]
    TextSlash,

    #[regex(r" +", |lex| (lex.span().len() as u32))]
    Spaces(u32),

    #[regex(r"\u{3000}+", |lex| ((lex.span().len() / WIDE_SPACE_LEN) as u32))]
    WideSpaces(u32),

    #[regex(r"\t+", |lex| lex.span().len() as u32)]
    Tabs(u32),

    #[token("@:")]
    #[token("＠：")]
    TextColon,

    #[token(":")]
    #[token("：")]
    Colon1,

    #[token("::")]
    #[token("：：")]
    Colon2,

    #[token(":::")]
    #[token("：：：")]
    Colon3,

    #[token("::::")]
    #[token("：：：：")]
    Colon4,

    #[token("@")]
    #[token("＠")]
    At,

    #[token("@@")]
    #[token("＠＠")]
    TextAt,

    #[token("|")]
    #[token("｜")]
    VerticalLine,

    #[token("||")]
    #[token("｜｜")]
    TextVerticalLine,

    #[regex(r"[%％][^\r\n%％]+[%％]?")]
    Expr(&'a str),

    #[token("%%")]
    #[token("％％")]
    TextPercent,

    #[token("＠《")]
    TextLeftDoubleAngleBracket,

    #[token("《")]
    LeftDoubleAngleBracket,

    #[token("＠》")]
    TextRightDoubleAngleBracket,

    #[token("》")]
    RightDoubleAngleBracket,

    #[regex(r"\p{XID_Start}\p{XID_Continue}*")]
    Identifier(&'a str),

    #[regex(r"[^ \t\u3000@＠:：\|｜《》\r\n\p{XID_Start}]+")]
    TextOthers(&'a str),
    //#[regex(r"[^\r\n \t\u{3000}@＠\|｜:：%％/《》]+")]
    //Text(&'a str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colon1() {
        let source = "：";
        let mut lex = Token::lexer(source);
        let mut x = || lex.next();
        let mut y = || x().unwrap().unwrap();

        assert_eq!(y(), Token::Colon1);
        assert_eq!(x(), None);
    }

    #[test]
    fn colon2() {
        let source = "：：";
        let mut lex = Token::lexer(source);
        let mut x = || lex.next();
        let mut y = || x().unwrap().unwrap();

        assert_eq!(y(), Token::Colon2);
        assert_eq!(x(), None);
    }

    #[test]
    fn colon3() {
        let source = "：：：";
        let mut lex = Token::lexer(source);
        let mut x = || lex.next();
        let mut y = || x().unwrap().unwrap();

        assert_eq!(y(), Token::Colon3);
        assert_eq!(x(), None);
    }

    #[test]
    fn colon4() {
        let source = "：：：：";
        let mut lex = Token::lexer(source);
        let mut x = || lex.next();
        let mut y = || x().unwrap().unwrap();

        assert_eq!(y(), Token::Colon4);
        assert_eq!(x(), None);
    }

    #[test]
    fn at_keyword2() {
        let lexer = Token::lexer("＠｜識別子《しきべつし》");
        let source = lexer.source();
        let mut iter = lexer.spanned().map(|a| {
            let token = a.0;
            let range = a.1;
            let text = unsafe { source.get_unchecked(range.clone()) };
            (token, range, text)
        });
        let x = iter.next().unwrap();
        assert_eq!(x.0, Ok(Token::At));
        let x = iter.next().unwrap();
        assert_eq!(x.0, Ok(Token::VerticalLine));
        let x = iter.next().unwrap();
        assert_eq!(x.0, Ok(Token::Identifier(&"識別子")));
        let x = iter.next().unwrap();
        assert_eq!(x.0, Ok(Token::LeftDoubleAngleBracket));
        let x = iter.next().unwrap();
        assert_eq!(x.0, Ok(Token::Identifier(&"しきべつし")));
        let x = iter.next().unwrap();
        assert_eq!(x.0, Ok(Token::RightDoubleAngleBracket));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn at_keyword() {
        let mut lexer = Token::lexer("＠｜識別子＠＠。《しきべつし》");
        assert_eq!(lexer.next(), Some(Ok(Token::At)));
        assert_eq!(lexer.next(), Some(Ok(Token::VerticalLine)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier(&"識別子"))));
        assert_eq!(lexer.next(), Some(Ok(Token::TextAt)));
        assert_eq!(lexer.slice(), "＠＠");
        assert_eq!(lexer.next(), Some(Ok(Token::TextOthers(&"。"))));
        assert_eq!(lexer.next(), Some(Ok(Token::LeftDoubleAngleBracket)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier(&"しきべつし"))));
        assert_eq!(lexer.next(), Some(Ok(Token::RightDoubleAngleBracket)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn at_vl() {
        let mut lexer = Token::lexer("＠ @ | ｜ ＠＠ @@ || ｜｜");
        assert_eq!(lexer.next(), Some(Ok(Token::At)));
        assert_eq!(lexer.span(), 0..3);
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(1))));
        assert_eq!(lexer.span(), 3..4);
        assert_eq!(lexer.next(), Some(Ok(Token::At)));
        assert_eq!(lexer.span(), 4..5);
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(1))));
        assert_eq!(lexer.span(), 5..6);
        assert_eq!(lexer.next(), Some(Ok(Token::VerticalLine)));
        assert_eq!(lexer.span(), 6..7);
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(1))));
        assert_eq!(lexer.span(), 7..8);
        assert_eq!(lexer.next(), Some(Ok(Token::VerticalLine)));
        assert_eq!(lexer.span(), 8..11);
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(1))));
        assert_eq!(lexer.span(), 11..12);
        assert_eq!(lexer.next(), Some(Ok(Token::TextAt)));
        assert_eq!(lexer.span(), 12..18);
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(1))));
        assert_eq!(lexer.span(), 18..19);
        assert_eq!(lexer.next(), Some(Ok(Token::TextAt)));
        assert_eq!(lexer.span(), 19..21);
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(1))));
        assert_eq!(lexer.span(), 21..22);
        assert_eq!(lexer.next(), Some(Ok(Token::TextVerticalLine)));
        assert_eq!(lexer.span(), 22..24);
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(1))));
        assert_eq!(lexer.span(), 24..25);
        assert_eq!(lexer.next(), Some(Ok(Token::TextVerticalLine)));
        assert_eq!(lexer.span(), 25..31);
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn spaces() {
        let mut lexer = Token::lexer("  　　　 \t\t");
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(2))));
        assert_eq!(lexer.span(), 0..2);
        assert_eq!(lexer.next(), Some(Ok(Token::WideSpaces(3))));
        assert_eq!(lexer.span(), 2..11);
        assert_eq!(lexer.next(), Some(Ok(Token::Spaces(1))));
        assert_eq!(lexer.span(), 11..12);
        assert_eq!(lexer.next(), Some(Ok(Token::Tabs(2))));
        assert_eq!(lexer.span(), 12..14);
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn new_line() {
        let mut lexer = Token::lexer("\r\n\n\n");
        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.span(), 0..2);
        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.span(), 2..3);
        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.span(), 3..4);
        assert_eq!(lexer.next(), None);
    }
}
