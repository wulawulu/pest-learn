use pest::Parser;
use pest_derive::Parser;
use std::{collections::HashMap, fs};
#[derive(Parser)]
#[grammar = "ch2_1/ini.pest"]
struct IniParser;

fn main() {
    let unparsed_file = fs::read_to_string("config.ini").expect("cannot read file");
    let file = IniParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut properties:HashMap<&str,HashMap<&str,&str>> = HashMap::new();
        
    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                current_section_name = line.into_inner().next().unwrap().as_str();
            }
            Rule::property => {
                let mut property_parts = line.into_inner();

                let name = property_parts.next().unwrap().as_str();
                let value = property_parts.next().unwrap().as_str();

                properties.entry(current_section_name).or_insert(HashMap::new()).insert(name, value);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }    
    println!("{:#?}", properties);
}
