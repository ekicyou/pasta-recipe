use pest_derive::*;

#[derive(Parser)]
#[grammar = "pasta.pest"]
pub struct PastaParser;

#[cfg(test)]
mod tests{
    use pest::Parser;
    use super::*;

# [test]
fn eol(){
    {
        let input = "";
        let pairs = PastaParser::parse(Rule::EOL , input).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(1, pairs.count());
    }
    {
        let input = "\n";
        let pairs = PastaParser::parse(Rule::EOL , input).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(0, pairs.count());
    }
}

#[test]
#[should_panic]
fn eol_panic(){
    {
        let input = "123";
        _ = PastaParser::parse(Rule::EOL , input).unwrap_or_else(|e| panic!("{}", e));
    }

}

# [test]
fn blank_line(){
    {
        let input = "   　";
        let pairs = PastaParser::parse(Rule::blank_line1 , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment";
        let pairs = PastaParser::parse(Rule::blank_line1 , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment\n";
        let pairs = PastaParser::parse(Rule::blank_line1 , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment\n\n";
        let pairs = PastaParser::parse(Rule::blank_line1 , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "//comment";
        let pairs = PastaParser::parse(Rule::blank_line2 , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "￥￥こめんと";
        let pairs = PastaParser::parse(Rule::blank_line2 , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "\n";
        let pairs = PastaParser::parse(Rule::blank_line3 , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "\r\n";
        let pairs = PastaParser::parse(Rule::blank_line3 , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }

    {
        let input = "   　";
        let pairs = PastaParser::parse(Rule::blank_line , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment";
        let pairs = PastaParser::parse(Rule::blank_line , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment\n";
        let pairs = PastaParser::parse(Rule::blank_line , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "   //comment\n\n";
        let pairs = PastaParser::parse(Rule::blank_line , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "//comment";
        let pairs = PastaParser::parse(Rule::blank_line , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "￥￥こめんと";
        let pairs = PastaParser::parse(Rule::blank_line , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "\n";
        let pairs = PastaParser::parse(Rule::blank_line , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
    {
        let input = "\r\n";
        let pairs = PastaParser::parse(Rule::blank_line , input).unwrap_or_else(|e| panic!("{}", e));
        println!("pairs:{:?}", pairs);
    }
}


}

