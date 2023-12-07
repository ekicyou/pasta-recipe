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

    #[regex(r"[^ \t\u3000@＠\|｜《》\r\n\p{XID_Start}]+")]
    TextOthers(&'a str),
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn colon4_2() {
        let source = "：：：：これ";
        let mut lex = Token::lexer(source);
        let mut x = || lex.next();
        let mut y = || x().unwrap().unwrap();

        assert_eq!(y(), Token::Colon4);
        assert_eq!(y(), Token::Identifier(&"これ"));
        assert_eq!(x(), None);
    }
}
