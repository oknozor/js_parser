#![feature(box_syntax)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use pest::iterators::Pair;
use pest::Parser;
use crate::ast::*;

mod ast;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JSParser;

fn parse_program(pair: Pair<Rule>) -> JSProgram {
    match pair.as_rule() {
        Rule::program => JSProgram {
            body: pair.into_inner().map(parse_statement).collect(),
        },
        _ => panic!("parse program unexpected token {:?}", pair)
    }
}

fn parse_statement(pair: Pair<Rule>) -> JSStatement {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::expression_statement => JSStatement::ExpressionStatement(JSExpressionStatement {
                expression: parse_expression(inner),
            }),
        Rule::declaration_statement => JSStatement::Declaration(parse_declaration(inner)),
        _ => panic!("parse statement unexpected token {:?}", inner)
    }
}

fn parse_declaration(pair: Pair<Rule>) -> JSDeclaration {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::variable_declaration => {
            let declarations = pair.into_inner()
                                   .map(|pair| {
                                       let mut inner_rules = pair
                                           .into_inner();

                                       let id = inner_rules
                                           .next()
                                           .unwrap()
                                           .as_str();

//Todo: handle optional value in the grammar
                                       let mut up = inner_rules.next().unwrap().into_inner();
                                       let expression = up.next().unwrap();
                                       let init = Some(parse_expression(expression));
                                       VariableDeclarator { id, init }
                                   }).collect();

            JSDeclaration::VariableDeclaration(JSVariableDeclaration {
                declarations,
                kind: Kind::Var,
            })
        }
        _ => panic!("parse declaration unexpected token {:?}", pair)
    }
}


fn parse_expression(pair: Pair<Rule>) -> JSExpression {
    match pair.as_rule() {
        Rule::literal => JSExpression::Literal(parse_literal(pair.into_inner().next().unwrap())),
        Rule::assignment_expression => {
            let mut inner_rules = pair.into_inner();
            let left = Box::new(
                parse_expression(inner_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                )
            );
            let operator = inner_rules
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .as_str();
            let right = Box::new(
                parse_expression(inner_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                )
            );
            JSExpression::AssignmentExpression(JSAssignmentExpression {
                left,
                operator: AssignmentOperator::from(operator),
                right,
            })
        }
        Rule::binary_expression => {
            println!("yep {:?}", pair.clone());
            JSExpression::BinaryExpression(JSBinaryExpression {
                operator: BinaryOperator::Add,
                left: box JSExpression::Literal(JSLiteral::String("a")),
                right: box JSExpression::Literal(JSLiteral::String("b")),
            })
        }
        _ => panic!("parse expresssion unexpected token {:?}", pair)
    }
}

fn parse_literal(pair: Pair<Rule>) -> JSLiteral {
    match pair.as_rule() {
        Rule::number => {
            let inner_rules = pair.into_inner().next().unwrap();
            match inner_rules.as_rule() {
                Rule::integer =>
                    JSLiteral::Number(JSNumber::Int(inner_rules.as_str().parse::<i64>().unwrap())),
                Rule::float =>
                    JSLiteral::Number(JSNumber::Float(inner_rules.as_str().parse::<f64>().unwrap())),
                _ => panic!(" parse literal unexpected token {:?}", inner_rules)
            }
        }
        Rule::string => JSLiteral::String(pair.as_str()),
        Rule::bool_true => JSLiteral::Boolean(true),
        Rule::bool_false => JSLiteral::Boolean(false),
        _ => panic!(" parse literal unexpected token {:?}", pair)
    }
}

pub fn parse(js_filename: &str) {
    let unparsed_file = fs::read_to_string(js_filename)
        .expect(&format!("Error reading {}", js_filename));

    let js = JSParser::parse(Rule::program, &unparsed_file).unwrap().next().unwrap();
    let ast = parse_program(js);
    println!("{:?}", ast);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}