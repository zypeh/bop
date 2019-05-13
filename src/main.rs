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
                    match statement.as_rule() {
                        Rule::EOI => {},
                        _ => ast.push(build_ast_from_production(statement)),
                    }
                }
            },
            Rule::EOI => {},
            _ => unreachable!("parse()")
        }
    }

    dbg!(ast.clone());
    Ok(ast)
}

fn build_ast_from_production(statement: pest::iterators::Pair<Rule>) -> ParseTree {
    let mut pairs = statement.into_inner();
    let identifier = pairs.next().unwrap().as_str();
    let expression = build_ast_from_expressions(pairs.next().unwrap().into_inner());
    ParseTree::NonTerminalDefinition(identifier, Box::new(expression))
}

// expression = { alternative ~ (("|" ~ alternative)*)? }
fn build_ast_from_expressions(pairs: pest::iterators::Pairs<Rule>) -> ParseTree {
    let mut alternatives = vec![];
    for pair in pairs {
        dbg!(pair.clone());
        let ast = match pair.as_rule() {
            Rule::alternative => build_ast_from_alternative(pair.into_inner().next().unwrap()),
            Rule::expression => build_ast_from_expressions(pair.into_inner()),
            _ => unreachable!(pair),
        };
        alternatives.push(ast);
    };
    ParseTree::Choice(alternatives)
}

// alternative = { string | group | option | repetition }
fn build_ast_from_alternative(pair: pest::iterators::Pair<Rule>) -> ParseTree {
    match pair.as_rule() {
        Rule::string => {
            match pair.into_inner().next().unwrap().as_str() {
                "" => ParseTree::Empty,
                x => ParseTree::Terminal(x),
            }
        },
        Rule::group => build_ast_from_expressions(pair.into_inner()),
        Rule::option => ParseTree::Optional(Box::new(build_ast_from_expressions(pair.into_inner()))),
        Rule::repetition => ParseTree::Many(Box::new(build_ast_from_expressions(pair.into_inner()))),
        _ => unreachable!("build_ast_from_alternative"),
    }
}
