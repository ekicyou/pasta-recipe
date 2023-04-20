use pasta_parser_pest::*;

#[test]
fn serif() {
    let rule = Rule::serif;
    let input = "役者　セリフ１セリフ１\n\n　　セリフ２セリフ２";
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: input,
        rule: Rule::serif,
        tokens:[
            serif(0, 65, [
                actor(0, 6),
                talk(9, 33, [
                    normal_talk(9, 33)
                ]),
                blank_lines(34, 35),
                talk(41, 65, [
                    normal_talk(41, 65)
                ]),
                EOI(65, 65)
            ])
        ]
    };
}

#[test]
fn togaki() {
    let rule = Rule::togaki;
    let input = "　　：表情￥￥こめんと";
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: input,
        rule: Rule::togaki,
        tokens: [
            togaki(0, 33, [
                togaki_attr(9, 15, [
                    emote(9, 15)
                ]),
                comment(15, 33, [comment_word(21, 33)]),
                EOI(33, 33)
            ])
        ]
    };
}

#[test]
fn hashira() {
    let rule = Rule::hashira;
    let input = "   \n\n＠柱　属性＠１￥￥こめんと\n----";
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: input,
        rule: Rule::hashira,
        tokens:[hashira(0, 49, [blank_lines(0, 5), hashira_line(5, 45, [id_attr(8, 26, [id(8, 11), attrs(11, 26, [attr(14, 26, [id(14, 20), expr(23, 26, [num(23, 26, [NUM1(23, 26)])])])])]), comment(26, 44, [comment_word(32, 44)])]), cut(45, 49, [cut_line(45, 49, [EOI(49, 49)])])])]
    };
}

#[test]
fn hashira_line() {
    let rule = Rule::hashira_line;
    let input = "＠柱　属性＠１￥￥こめんと";
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: input,
        rule: Rule::hashira_line,
        tokens: [
            hashira_line(0, 39, [
                id_attr(3, 21, [id(3, 6), attrs(6, 21, [attr(9, 21, [id(9, 15), expr(18, 21, [num(18, 21, [NUM1(18, 21)])])])])]),
                comment(21, 39, [comment_word(27, 39)]),
                EOI(39, 39)
            ])]
    };
}

#[test]
fn hashira_line2() {
    let rule = Rule::hashira_line;
    let input = "＠";
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: input,
        rule: Rule::hashira_line,
        tokens: [hashira_line(0, 3, [EOI(3, 3)])]
    };
}

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
        tokens: [scene_head(15, 48, [id_attr(15, 48, [id(15, 27), attrs(27, 48, [attr(30, 48, [id(30, 39), expr(42, 48, [num(42, 48, [NUM1(42, 45), NUM0(45, 48)])])])])])]), comment(51, 69, [comment_word(57, 69)]), EOI(69, 69)]
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
        tokens: [scene_head(6, 39, [id_attr(6, 39, [id(6, 18), attrs(18, 39, [attr(21, 39, [id(21, 30), expr(33, 39, [num(33, 39, [NUM1(33, 36), NUM0(36, 39)])])])])])]), comment(42, 60, [comment_word(48, 60)]), EOI(60, 60)]
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
        println!("pairs:\n{}\n", pairs);
    }
    {
        let input = "   //comment";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    {
        let input = "   //comment\n";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    {
        let input = "   //comment\n\n";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    {
        let input = "//comment";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    {
        let input = "￥￥こめんと";
        let pairs =
            PastaParser::parse(Rule::blank_line1, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    {
        let input = "\n";
        let pairs =
            PastaParser::parse(Rule::blank_line2, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }
    {
        let input = "\r\n";
        let pairs =
            PastaParser::parse(Rule::blank_line2, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
    }

    {
        let input = "   　";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
        parses_to! {
            parser: PastaParser,
            input: input,
            rule: Rule::blank_line,
            tokens: [blank_line(0, 6)]
        };
    }
    {
        let input = "   //comment";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
        parses_to! {
            parser: PastaParser,
            input: input,
            rule: Rule::blank_line,
            tokens: [blank_line(0, 12)]
        };
    }
    {
        let input = "   //comment\n";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
        parses_to! {
            parser: PastaParser,
            input: input,
            rule: Rule::blank_line,
            tokens: [blank_line(0, 13)]
        };
    }
    {
        let input = "   //comment\n\n";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
        parses_to! {
            parser: PastaParser,
            input: input,
            rule: Rule::blank_line,
            tokens: [blank_line(0, 13)]
        };
    }
    {
        let input = "//comment";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
        parses_to! {
            parser: PastaParser,
            input: input,
            rule: Rule::blank_line,
            tokens: [blank_line(0, 9)]
        };
    }
    {
        let input = "￥￥こめんと";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
        parses_to! {
            parser: PastaParser,
            input: input,
            rule: Rule::blank_line,
            tokens: [blank_line(0, 18)]
        };
    }
    {
        let input = "\n";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
        parses_to! {
            parser: PastaParser,
            input: input,
            rule: Rule::blank_line,
            tokens: [blank_line(0, 1)]
        };
    }
    {
        let input = "\r\n";
        let pairs = PastaParser::parse(Rule::blank_line, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{}\n", pairs);
        parses_to! {
            parser: PastaParser,
            input: input,
            rule: Rule::blank_line,
            tokens: [blank_line(0, 2)]
        };
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
