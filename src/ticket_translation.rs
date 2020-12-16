#[macro_use]
extern crate scan_rules;

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Constraint {
    name: String,
    constraints: [(u64, u64); 2],
}

impl Constraint {
    fn is_valid_value(&self, value: u64) -> bool {
        self.constraints
            .iter()
            .any(|constraint| constraint.0 <= value && value <= constraint.1)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Ticket(Vec<u64>);

fn is_valid(t: &Ticket, constraints: &[Constraint]) -> bool {
    t.0.iter().all(|value| {
        constraints
            .iter()
            .any(|constraint| constraint.is_valid_value(*value))
    })
}

fn error_rate(t: &Ticket, constraints: &[Constraint]) -> u64 {
    t.0.iter()
        .map(|value| {
            if constraints
                .iter()
                .any(|constraint| constraint.is_valid_value(*value))
            {
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

fn parse_constraint(input: &str) -> Constraint {
    let mut it = input.split(':');
    let field_name = it.next().unwrap();
    let rest = it.next().unwrap();
    scan!(
    rest;
    (let l1: u64, "-", let r1: u64, " or ", let l2: u64, "-", let r2: u64) => Constraint{
        name: field_name.to_string(),
        constraints: [(l1, r1), (l2, r2)],
    })
    .unwrap()
}

fn read_input() -> Result<(Vec<Constraint>, Ticket, Vec<Ticket>), String> {
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

    let constraints: Vec<Constraint> = iter
        .next()
        .unwrap()
        .iter()
        .map(|line| parse_constraint(&line))
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

        // Part 2.
        let mut valid_tickets: Vec<Ticket> = nearby_tickets
            .into_iter()
            .filter(|ticket| is_valid(&ticket, &constraints))
            .collect();
        valid_tickets.push(my_ticket.clone());

        let mut valid_fields_for_column: Vec<(usize, Vec<Constraint>)> = vec![];
        for i in 0..valid_tickets[0].0.len() {
            valid_fields_for_column.push((
                i,
                constraints
                    .iter()
                    .cloned()
                    .filter(|constraint| {
                        valid_tickets
                            .iter()
                            .all(|ticket| constraint.is_valid_value(ticket.0[i]))
                    })
                    .collect(),
            ))
        }
        valid_fields_for_column.sort_by_key(|a| a.1.len());
        let mut used_fields: HashSet<&Constraint> = HashSet::new();
        let assigned_fields: Vec<(usize, &Constraint)> = valid_fields_for_column
            .iter()
            .map(|valid_columns| {
                let possible_columns = HashSet::from_iter(valid_columns.1.iter());
                let fields: Vec<&Constraint> =
                    possible_columns.difference(&used_fields).cloned().collect();
                assert_eq!(fields.len(), 1);
                used_fields.insert(&fields[0]);
                (valid_columns.0, fields[0])
            })
            .collect();

        let mut res = 1;
        for constraint in assigned_fields.iter() {
            if constraint.1.name.starts_with("departure ") {
                res *= my_ticket.0[constraint.0]
            }
        }
        println!("Part 2. Product of fields containing departure: {}", res);
    } else {
        println!("error in parsing");
    }
}
