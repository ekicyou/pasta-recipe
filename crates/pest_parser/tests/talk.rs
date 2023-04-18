use pasta_parser_pest::*;

#[test]
fn talk() {
    let rule = Rule::pasta;
    let input = include_str!("talk.pasta");
    {
        let pairs = PastaParser::parse(rule, input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:\n{pairs}\n");
        assert!(false);
    }
}
