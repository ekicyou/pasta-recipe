use pasta_parser_pest::*;

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
        println!("pairs:{}", pairs);
    }
    parses_to! {
        parser: PastaParser,
        input: "abc x@10 y@str",
        rule: Rule::id_attr,
        tokens: [
            id_attr(0, 14, [
                id(0, 3),
                attrs(3, 14, [
                    attr(3, 8, [
                        id(4, 5),
                        expr(6, 8, [
                            num(6, 8, [NUM1(6, 7), NUM0(7, 8)])
                        ])
                    ]),
                    attr(8, 14, [
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
    parses_to! {
        parser: PastaParser,
        input: "//comment",
        rule: Rule::comment,
        tokens: [
            comment(0, 9, []),
        ]
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
