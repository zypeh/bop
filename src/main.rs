extern crate pest;
#[macro_use] extern crate pest_derive;

// use std::collections::HashMap;
use std::fs;
use pest::Parser;
use pest::error::Error;

mod ast;
use ast::ast::ParseTree;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EBNFParser;

fn main() {
    let unparsed_file = fs::read_to_string("./src/test.bop").expect("cannot read file");
    // let _file = EBNFParser::parse(Rule::bop, &unparsed_file)
    //     .expect("unsuccessful parse") // unwrap the parse result
    //     .next().unwrap(); // get and unwrap the `file` rule; never fails

    // dbg!(_file);
    parse(&unparsed_file).expect("unsuccessful parse");
    println!("Successfully parsed !");
}

fn parse(source: &str) -> Result<Vec<ParseTree>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = EBNFParser::parse(Rule::bop, &source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::bop => {
                let statements = pair.into_inner();
                for statement in statements {
                    ast.push(build_ast_from_production(statement));
                }
            },
            Rule::EOI => {},
            _ => unreachable!("parse()")
        }
    }

    Ok(ast)
}

fn build_ast_from_production(statement: pest::iterators::Pair<Rule>) -> ParseTree {
    let mut pairs = statement.into_inner();
    let identifier = pairs.next().unwrap().as_str();
    let expression = build_ast_from_expressions(pairs.next().unwrap().into_inner());

    dbg!(ParseTree::NonTerminalDefinition(identifier, Box::new(expression.clone())));
    ParseTree::NonTerminalDefinition(identifier, Box::new(expression))
}

// alternative = { string | group | option | repetition }
fn build_ast_from_expressions(pairs: pest::iterators::Pairs<Rule>) -> ParseTree {
    let mut alternatives = vec![];
    for pair in pairs {

        let ast = match pair.as_rule() {
            Rule::string => {
                match pair.into_inner().next().unwrap().as_str() {
                    "" => ParseTree::Empty,
                    x => ParseTree::Terminal(x),
                }
            },
            Rule::expression => build_ast_from_expressions(pair.into_inner()),
            Rule::alternative => build_ast_from_expressions(pair.into_inner()),
            Rule::group => build_ast_from_expressions(pair.into_inner()),
            Rule::option => ParseTree::Optional(Box::new(build_ast_from_expressions(pair.into_inner()))),
            Rule::repetition => ParseTree::Many(Box::new(build_ast_from_expressions(pair.into_inner()))),
            _ => unreachable!(pair),
        };
        alternatives.push(ast);
    };
    ParseTree::Choice(alternatives)
}
