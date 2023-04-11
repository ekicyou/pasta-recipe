use pasta_parser_pest::*;

#[test]
fn scene4_line() {
    let rule = Rule::scene4_line;
    let input = "：：：：：シーン４　属性１＠１０　￥￥こめんと";
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: "：：：：：シーン４　属性１＠１０　￥￥こめんと",
        rule: Rule::scene4_line,
        tokens: [scene4_line(0, 69, [id_attr(15, 48, [id(15, 27), attrs(27, 48, [attr(30, 48, [id(30, 39), expr(42, 48, [num(42, 48, [NUM1(42, 45), NUM0(45, 48)])])])])]), comment(51, 69, [comment_word(57, 69)]), EOI(69, 69)])]
    };
}

#[test]
fn scene1_line() {
    let rule = Rule::scene1_line;
    let input = "：：シーン１　属性１＠１０　￥￥こめんと";
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: "：：シーン１　属性１＠１０　￥￥こめんと",
        rule: Rule::scene1_line,
        tokens: [scene1_line(0, 60, [id_attr(6, 39, [id(6, 18), attrs(18, 39, [attr(21, 39, [id(21, 30), expr(33, 39, [num(33, 39, [NUM1(33, 36), NUM0(36, 39)])])])])]), comment(42, 60, [comment_word(48, 60)]), EOI(60, 60)])]
    };
}

#[test]
fn scene1_mark() {
    let rule = Rule::SCENE1;
    let input = "::";
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{}", pairs);
    }
}

#[test]
fn id_attr_test() {
    parses_to! {
        parser: PastaParser,
        input: "abc",
        rule: Rule::id_attr,
        tokens: [
            id_attr(0, 3, [
                id(0, 3, []),
                attrs(3, 3, []),
            ]),
        ]
    };
    {
        let input = "abc x@10 y@str";
        let pairs = PastaParser::parse(Rule::id_attr, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: "abc x@10 y@str",
        rule: Rule::id_attr,
        tokens: [
            id_attr(0, 14, [
                id(0, 3), attrs(3, 14, [
                    attr(4, 8, [
                        id(4, 5),
                        expr(6, 8, [
                            num(6, 8, [NUM1(6, 7), NUM0(7, 8)])
                        ])
                    ]),
                    attr(9, 14, [
                        id(9, 10),
                        expr(11, 14, [id(11, 14)])
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn parses_to_test() {
    {
        let input = "//comment";
        let pairs = PastaParser::parse(Rule::comment, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: "//comment",
        rule: Rule::comment,
        tokens: [comment(0, 9, [comment_word(2, 9)])]
    };
}

#[test]
fn blank_line() {
    {
        let input = "   　";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment\n";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment\n\n";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "//comment";
        let pairs =
            PastaParser::parse(Rule::blank_line2, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "￥￥こめんと";
        let pairs =
            PastaParser::parse(Rule::blank_line2, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "\n";
        let pairs =
            PastaParser::parse(Rule::blank_line3, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "\r\n";
        let pairs =
            PastaParser::parse(Rule::blank_line3, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }

    {
        let input = "   　";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment\n";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment\n\n";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "//comment";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "￥￥こめんと";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "\n";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "\r\n";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
}

#[test]
fn eol() {
    {
        let input = "";
        let pairs = PastaParser::parse(Rule::EOL, input).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(1, pairs.count());
    }
    {
        let input = "\n";
        let pairs = PastaParser::parse(Rule::EOL, input).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(0, pairs.count());
    }
}

#[test]
#[should_panic]
fn eol_panic() {
    {
        let input = "123";
        _ = PastaParser::parse(Rule::EOL, input).unwrap_or_else(|e| panic!("{}", e));
    }
}
