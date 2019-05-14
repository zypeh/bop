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
    let x = parse(&unparsed_file).expect("unsuccessful parse");
    dbg!(x);
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
                    match statement.as_rule() {
                        Rule::EOI => {},
                        Rule::production => ast.push(build_ast_from_production(statement)),
                        _ => unreachable!("statement parse()"),
                    }
                }
            },
            Rule::EOI => {},
            _ => unreachable!("parse()")
        }
    }
    Ok(ast)
}

fn build_ast_from_production(production: pest::iterators::Pair<Rule>) -> ParseTree {
    let mut pairs = production.into_inner();
    let identifier = pairs.next().unwrap().as_str();

    match pairs.peek() {
        None => ParseTree::NonTerminalDefinition(identifier, Box::new(ParseTree::Empty)),
        Some(_) => {        
            let expression = build_ast_from_expression(pairs.next().unwrap());
            ParseTree::NonTerminalDefinition(identifier, Box::new(expression))
        }
    }
}

fn build_ast_from_expression(pair: pest::iterators::Pair<Rule>) -> ParseTree {
    let mut nodes = vec![];

    let alternatives = pair.into_inner();
    for alternative in alternatives {
        let inner_pairs = alternative.into_inner();
        for pair in inner_pairs {
            let ast = match pair.as_rule() {
                Rule::string => {
                    match pair.into_inner().next().unwrap().as_str() {
                        "" => ParseTree::Empty,
                        x => ParseTree::Terminal(x),
                    }
                },
                Rule::group => build_ast_from_expression(pair.into_inner().next().unwrap()),
                Rule::option => ParseTree::Optional(Box::new(build_ast_from_expression(pair.into_inner().next().unwrap()))),
                Rule::repetition => ParseTree::Many(Box::new(build_ast_from_expression(pair.into_inner().next().unwrap()))),
                _ => unreachable!(pair),
            };
            nodes.push(ast);
        }
    }

    match nodes.len() {
        0 => unreachable!("empty nodes in bop file"),
        1 => nodes[0].clone(),
        _ => ParseTree::Choice(nodes),
    }
}
