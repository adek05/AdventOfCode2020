#[macro_use]
extern crate scan_rules;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

struct Field {
    name: String,
    constraints: [(u64, u64); 2],
}

impl Field {
    fn is_valid_value(&self, value: u64) -> bool {
        self.constraints
            .iter()
            .any(|constraint| constraint.0 <= value && value <= constraint.1)
    }
}

struct Ticket(Vec<u64>);

fn error_rate(t: &Ticket, fields: &[Field]) -> u64 {
    t.0.iter()
        .map(|value| {
            if fields.iter().any(|field| field.is_valid_value(*value)) {
                0
            } else {
                *value
            }
        })
        .sum()
}

fn parse_ticket(input: &str) -> Ticket {
    Ticket(
        input
            .split(',')
            .map(|number| number.parse::<u64>().unwrap())
            .collect(),
    )
}

fn read_input() -> Result<(Vec<Field>, Ticket, Vec<Ticket>), String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file =
        File::open("in/TicketTranslation.in").map_err(|_| "Input file not found".to_string())?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let mut iter = lines.split(|line| line.is_empty());

    let constraints: Vec<Field> = iter
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut it = line.split(':');
            let field_name = it.next().unwrap();
            let rest = it.next().unwrap();
            scan!(
            rest;
            (let l1: u64, "-", let r1: u64, " or ", let l2: u64, "-", let r2: u64) => Field{
                name: field_name.to_string(),
                constraints: [(l1, r1), (l2, r2)],
            })
            .unwrap()
        })
        .collect();

    let my_ticket = parse_ticket(&iter.next().unwrap()[1]);

    let nearby_tickets = iter.next().unwrap()[1..]
        .iter()
        .map(|input| parse_ticket(&input))
        .collect();

    Ok((constraints, my_ticket, nearby_tickets))
}

fn main() {
    if let Ok((constraints, my_ticket, nearby_tickets)) = read_input() {
        println!(
            "Part 1. Error rate for nearby tickets: {}",
            nearby_tickets
                .iter()
                .map(|ticket| error_rate(&ticket, &constraints))
                .sum::<u64>()
        );
    } else {
        println!("error in parsing");
    }
}
