use std::fs;

use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ch3/json.pest"]
struct JsonParser;

fn parse_json_file(file:&str)->Result<JSONValue,Error<Rule>>{
    let json =JsonParser::parse(Rule::json,file)?.next().unwrap();

    use pest::iterators::Pair;
    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            Rule::json
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => unreachable!(),
        }
    }
    Ok(parse_value(json))

}


fn main() {
    let unparsed_file = fs::read_to_string("data.json").expect("cannot read file");

    let json: JSONValue = parse_json_file(&unparsed_file).expect("unsuccessful parse");

    println!("{}", serialize_jsonvalue(&json));
}

enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn serialize_jsonvalue(val: &JSONValue) -> String {
    use JSONValue::*;
    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, serialize_jsonvalue(v)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(a) => {
            let contents:Vec<_> = a.iter().map(serialize_jsonvalue).collect();
            format!("[{}]", contents.join(","))
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", b),
        Null => format!("null"),
    }
}
