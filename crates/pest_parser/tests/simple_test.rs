use pasta_parser_pest::*;
use pest::*;

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
