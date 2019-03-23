extern crate js_parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    js_parser::parse(&args[1]);
}