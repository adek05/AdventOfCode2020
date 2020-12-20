use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

#[derive(Debug)]
enum Expression {
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Bracket(Box<Expression>),
}

fn eval(e: &Expression) -> i64 {
    match e {
        Expression::Number(n) => *n,
        Expression::Add(e1, e2) => eval(e1) + eval(e2),
        Expression::Multiply(e1, e2) => eval(e1) * eval(e2),
        Expression::Bracket(expression) => eval(expression),
    }
}

fn read_input() -> Result<Vec<String>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file =
        File::open("in/OperationOrder.in").map_err(|_| "Input file not found".to_string())?;
    io::BufReader::new(file)
        .lines()
        .map(|l| l.map_err(|_| "err".to_string()))
        .collect()
}

fn parse_op(left: Expression, it: &mut dyn Iterator<Item = &str>) -> Expression {
    match it.next() {
        None => left,
        Some("+") => Expression::Add(Box::new(left), Box::new(parse_expression(it))),
        Some("*") => Expression::Multiply(Box::new(left), Box::new(parse_expression(it))),
        Some("(") => left,
        x => panic!("Unexptected operation {:?}", x),
    }
}

fn parse_expression(it: &mut dyn Iterator<Item = &str>) -> Expression {
    if let Some(next) = it.next() {
        match next {
            ")" => parse_op(Expression::Bracket(Box::new(parse_expression(it))), it),
            "(" => panic!("Should not get ')'"),
            number => parse_op(Expression::Number(number.parse::<i64>().unwrap()), it),
        }
    } else {
        panic!("Empty expression unexpected at: ---");
    }
}

fn parse_line(line: &str) -> Expression {
    parse_expression(&mut line.replace("(", "( ").replace(")", " )").split(' ').rev())
}

fn main() {
    if let Ok(expressions) = read_input() {
        println!(
            "Part 1. Sum of all expressions: {}",
            expressions
                .iter()
                .map(|expr_line| eval(&parse_line(expr_line)))
                .sum::<i64>()
        );
    }
}
