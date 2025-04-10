use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ch2/ch2.pest"]
struct Ch2Parser;

fn main() {
    let parse_result = Ch2Parser::parse(Rule::sum, "1773 + 1362").unwrap();
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        let pair = Ch2Parser::parse(Rule::enclosed, "(..6472..) and more text")
            .unwrap()
            .next()
            .unwrap();

        assert_eq!(pair.as_rule(), Rule::enclosed);
        assert_eq!(pair.as_str(), "(..6472..)");

        let inner_rules = pair.into_inner();
        println!("{}", inner_rules); // --> [number(3, 7)]
    }
}
