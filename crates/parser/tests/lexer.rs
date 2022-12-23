use logos::Logos;
use pasta_parser::lexer::*;

const TEXT1: &str = include_str!("sample/text1.pasta");

#[test]
fn text1() {
    let source = TEXT1;
    let mut lex = Token::lexer(source);
    let mut x = || lex.next();

    assert_eq!(x().unwrap(), Token::Colon4);
    assert_eq!(x().unwrap(), Token::Text(&"これはタイトルです"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Colon3);
    assert_eq!(x().unwrap(), Token::Text(&"これはサブタイトルです"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Colon2);
    assert_eq!(x().unwrap(), Token::Text(&"これは章です"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::LineComment(&"// コメント"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::VerticalLine);
    assert_eq!(x().unwrap(), Token::Text(&"識別子"));
    assert_eq!(x().unwrap(), Token::LeftDoubleAngleBracket);
    assert_eq!(x().unwrap(), Token::Text(&"しきべつし"));
    assert_eq!(x().unwrap(), Token::RightDoubleAngleBracket);
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Spaces(4));
    assert_eq!(
        x().unwrap(),
        (Token::Text(&"識別子の説明には行頭に空白を入れる。"))
    );
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Spaces(4));
    assert_eq!(
        x().unwrap(),
        (Token::Text(&"行頭空白以外はMarkdown文法とする。"))
    );
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::LineComment(&"// キーワード宣言"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::VerticalLine);
    assert_eq!(x().unwrap(), Token::Text(&"おじさん"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Spaces(4));
    assert_eq!(x().unwrap(), Token::Text(&"何の変哲もないサラリーマン。"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::VerticalLine);
    assert_eq!(x().unwrap(), Token::Text(&"ぱすた"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Spaces(4));
    assert_eq!(x().unwrap(), Token::Text(&"窓の中の世界にいる女の子。"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Newline);
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Colon);
    assert_eq!(x().unwrap(), Token::Text(&"これは柱です"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Spaces(4));
    assert_eq!(x().unwrap(), Token::Text(&"柱の説明は行頭に空白を入れる。"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Newline);
    assert_eq!(x().unwrap(), Token::Text(&"本文。識別子を使いました。"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Text(&"おじさん"));
    assert_eq!(x().unwrap(), Token::Spaces(4));
    assert_eq!(x().unwrap(), Token::At);
    assert_eq!(x().unwrap(), Token::Text(&"通常"));
    assert_eq!(x().unwrap(), Token::Spaces(2));
    assert_eq!(x().unwrap(), Token::Text(&"いい天気ですね。"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Text(&"ぱすた"));
    assert_eq!(x().unwrap(), Token::Spaces(6));
    assert_eq!(x().unwrap(), Token::At);
    assert_eq!(x().unwrap(), Token::Text(&"疑問"));
    assert_eq!(x().unwrap(), Token::Spaces(2));
    assert_eq!(x().unwrap(), Token::Text(&"そうですか？"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Spaces(12));
    assert_eq!(x().unwrap(), Token::At);
    assert_eq!(x().unwrap(), Token::Text(&"憂鬱"));
    assert_eq!(x().unwrap(), Token::WideSpaces(1));
    assert_eq!(x().unwrap(), Token::At);
    assert_eq!(x().unwrap(), Token::Text(&"悪い天気"));
    assert_eq!(x().unwrap(), Token::WideSpaces(1));
    assert_eq!(x().unwrap(), Token::Text(&"予報ですけど。"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Colon);
    assert_eq!(x().unwrap(), Token::Text(&"会話"));
    assert_eq!(x().unwrap(), Token::Expr(&"％好感度 >= 100"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Text(&"おじさん"));
    assert_eq!(x().unwrap(), Token::Spaces(4));
    assert_eq!(x().unwrap(), Token::At);
    assert_eq!(x().unwrap(), Token::Text(&"通常"));
    assert_eq!(x().unwrap(), Token::WideSpaces(1));
    assert_eq!(
        x().unwrap(),
        Token::Text(&"この仕事が終わったら結婚するんだ‥‥")
    );
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Spaces(12));
    assert_eq!(x().unwrap(), Token::Expr(&"％死亡フラグ=true"));
    assert_eq!(x().unwrap(), Token::Newline);

    assert_eq!(x().unwrap(), Token::Text(&"ぱすた"));
    assert_eq!(x().unwrap(), Token::Spaces(6));
    assert_eq!(x().unwrap(), Token::At);
    assert_eq!(x().unwrap(), Token::Text(&"通常"));
    assert_eq!(x().unwrap(), Token::WideSpaces(1));
    assert_eq!(
        x().unwrap(),
        (Token::Text(&"いま、死亡フラグ立ちましたよ？"))
    );
    assert_eq!(x().unwrap(), Token::Newline);
    assert_eq!(x(), None);
}
