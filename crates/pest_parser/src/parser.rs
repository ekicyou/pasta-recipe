use pest_derive::*;

#[derive(Parser)]
#[grammar = "pasta.pest"]
pub struct PastaParser;

#[cfg(test)]
mod tests {
    use super::*;
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
    }


}
