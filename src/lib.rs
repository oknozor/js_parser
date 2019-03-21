extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::fs;

use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::prec_climber::PrecClimber;
use pest::Parser;
use pest::RuleType;
use pest::prec_climber::Operator;
use pest::prec_climber::Assoc;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JSParser;


fn parse_value(pair: Pair<Rule>) -> () {}

pub fn parse(js_filename: &str) {
    let unparsed_file = fs::read_to_string(js_filename)
        .expect(&format!("Error reading {}", js_filename));

    let js = JSParser::parse(Rule::program, &unparsed_file);
    match js {
        Ok(j) => println!("{:?}", j),
        Err(e) => println!("{:?}", e)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
