extern crate pest;
#[macro_use] extern crate pest_derive;

// use std::collections::HashMap;
use std::fs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EBNFParser;

fn main() {
    let unparsed_file = fs::read_to_string("./src/test.bop").expect("cannot read file");

    let _file = EBNFParser::parse(Rule::bop, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    dbg!(_file);
}