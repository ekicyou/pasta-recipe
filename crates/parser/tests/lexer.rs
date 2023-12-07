use logos::Logos;
use pasta_parser::lexer::*;

#[test]
fn text1() {
    const TEXT: &str = include_str!("sample/text1.pasta");
    let source = TEXT;
    let mut lex = Token::lexer(source);
    let mut x = || lex.next();
    let mut y = || x().unwrap().unwrap();

    assert_eq!(y(), Token::Colon4);
    assert_eq!(y(), Token::Identifier(&"これはタイトルです"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Colon3);
    assert_eq!(y(), Token::Identifier(&"これはサブタイトルです"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Colon2);
    assert_eq!(y(), Token::Identifier(&"これは章です"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::LineComment(&"// コメント"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::VerticalLine);
    assert_eq!(y(), Token::Identifier(&"識別子"));
    assert_eq!(y(), Token::LeftDoubleAngleBracket);
    assert_eq!(y(), Token::Identifier(&"しきべつし"));
    assert_eq!(y(), Token::RightDoubleAngleBracket);
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Spaces(4));
    assert_eq!(
        y(),
        (Token::Identifier(&"識別子の説明には行頭に空白を入れる"))
    );
    assert_eq!(y(), (Token::TextOthers(&"。")));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Spaces(4));
    assert_eq!(
        y(),
        (Token::Identifier(&"行頭空白以外はMarkdown文法とする"))
    );
    assert_eq!(y(), (Token::TextOthers(&"。")));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::LineComment(&"// キーワード宣言"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::VerticalLine);
    assert_eq!(y(), Token::Identifier(&"おじさん"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Spaces(4));
    assert_eq!(y(), Token::Identifier(&"何の変哲もないサラリーマン"));
    assert_eq!(y(), Token::TextOthers(&"。"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::VerticalLine);
    assert_eq!(y(), Token::Identifier(&"ぱすた"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Spaces(4));
    assert_eq!(y(), Token::Identifier(&"窓の中の世界にいる女の子"));
    assert_eq!(y(), Token::TextOthers(&"窓。"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Newline);
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Colon1);
    assert_eq!(y(), Token::Identifier(&"これは柱です"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Spaces(4));
    assert_eq!(y(), Token::Identifier(&"柱の説明は行頭に空白を入れる"));
    assert_eq!(y(), Token::TextOthers(&"。"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Newline);
    assert_eq!(y(), Token::Identifier(&"本文"));
    assert_eq!(y(), Token::TextOthers(&"。"));
    assert_eq!(y(), Token::Identifier(&"識別子を使いました"));
    assert_eq!(y(), Token::TextOthers(&"。"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Identifier(&"おじさん"));
    assert_eq!(y(), Token::Spaces(4));
    assert_eq!(y(), Token::At);
    assert_eq!(y(), Token::Identifier(&"通常"));
    assert_eq!(y(), Token::Spaces(2));
    assert_eq!(y(), Token::Identifier(&"いい天気ですね"));
    assert_eq!(y(), Token::TextOthers(&"。"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Identifier(&"ぱすた"));
    assert_eq!(y(), Token::Spaces(6));
    assert_eq!(y(), Token::At);
    assert_eq!(y(), Token::Identifier(&"疑問"));
    assert_eq!(y(), Token::Spaces(2));
    assert_eq!(y(), Token::Identifier(&"そうですか"));
    assert_eq!(y(), Token::TextOthers(&"？"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Spaces(12));
    assert_eq!(y(), Token::At);
    assert_eq!(y(), Token::Identifier(&"憂鬱"));
    assert_eq!(y(), Token::WideSpaces(1));
    assert_eq!(y(), Token::At);
    assert_eq!(y(), Token::Identifier(&"悪い天気"));
    assert_eq!(y(), Token::WideSpaces(1));
    assert_eq!(y(), Token::Identifier(&"予報ですけど"));
    assert_eq!(y(), Token::TextOthers(&"。"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Colon1);
    assert_eq!(y(), Token::Identifier(&"会話"));
    assert_eq!(y(), Token::Expr(&"％好感度 >= 100"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Identifier(&"おじさん"));
    assert_eq!(y(), Token::Spaces(4));
    assert_eq!(y(), Token::At);
    assert_eq!(y(), Token::Identifier(&"通常"));
    assert_eq!(y(), Token::WideSpaces(1));
    assert_eq!(y(), Token::Identifier(&"この仕事が終わったら結婚するんだ"));
    assert_eq!(y(), Token::TextOthers(&"‥‥"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Spaces(12));
    assert_eq!(y(), Token::Expr(&"％死亡フラグ=true"));
    assert_eq!(y(), Token::Newline);

    assert_eq!(y(), Token::Identifier(&"ぱすた"));
    assert_eq!(y(), Token::Spaces(6));
    assert_eq!(y(), Token::At);
    assert_eq!(y(), Token::Identifier(&"通常"));
    assert_eq!(y(), Token::WideSpaces(1));
    assert_eq!(y(), (Token::Identifier(&"いま")));
    assert_eq!(y(), (Token::TextOthers(&"、")));
    assert_eq!(y(), (Token::Identifier(&"死亡フラグ立ちましたよ")));
    assert_eq!(y(), (Token::TextOthers(&"？")));
    assert_eq!(y(), Token::Newline);
    assert_eq!(x(), None);
}

#[test]
fn text2() {
    let source = include_str!("sample/text2.pasta");
    let mut lex = Token::lexer(source);
    let mut x = || lex.next();
    let mut y = || x().unwrap().unwrap();

    assert_eq!(y(), Token::Colon1);
    assert_eq!(y(), Token::Identifier(&"これはタイトルです"));
    assert_eq!(y(), Token::Newline);
    assert_eq!(x(), None);
}
