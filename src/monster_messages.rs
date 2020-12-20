#[macro_use]
extern crate scan_rules;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path;

#[derive(Debug)]
enum Rule {
    Character(char),
    Sequence(Vec<usize>),
    Or((Vec<usize>, Vec<usize>)),
}

fn consume<'a>(msg: &'a str, rule: &Rule, all_rules: &HashMap<usize, Rule>) -> Option<&'a str> {
    match rule {
        Rule::Character(c) => {
            let len = msg.chars().count();
            if len > 0 && msg.chars().next().unwrap() == *c {
                Some(&msg[1..])
            } else {
                None
            }
        }
        Rule::Sequence(seq) => {
            let mut work_msg = msg;
            for rule_id in seq {
                if let Some(tail) = consume(work_msg, &all_rules[rule_id], all_rules) {
                    work_msg = tail;
                } else {
                    return None;
                }
            }
            Some(work_msg)
        }
        Rule::Or((rule_a, rule_b)) => {
            if let Some(tail_a) = consume(msg, &Rule::Sequence(rule_a.clone()), all_rules) {
                Some(tail_a)
            } else if let Some(tail_b) = consume(msg, &Rule::Sequence(rule_b.clone()), all_rules) {
                Some(tail_b)
            } else {
                None
            }
        }
    }
}

fn parse_rule(line: &str) -> (usize, Rule) {
    let s: String = line.replace(":", " :");
    scan!(
        &s;
        (let id: usize, ":", let letter: String) => (id, Rule::Character(letter.chars().next().unwrap())),
        (let id: usize, ":", [let rules: usize]+) => (id, Rule::Sequence(rules)),
        (let id: usize, ":", [let rules_a: usize]+, "|", [let rules_b: usize]+) => (id, Rule::Or((rules_a, rules_b))),
    ).unwrap()
}

fn read_input() -> Result<(Vec<(usize, Rule)>, Vec<String>), String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file =
        File::open("in/MonsterMessages.in").map_err(|_| "Input file not found".to_string())?;
    let input: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut split_it = input.split(|s| s.is_empty());
    let rules = split_it
        .next()
        .unwrap()
        .iter()
        .map(|rule| parse_rule(rule))
        .collect();
    let messages = split_it.next().unwrap();

    Ok((rules, messages.to_vec()))
}

fn main() {
    if let Ok((rules, messages)) = read_input() {
        let all_rules: HashMap<usize, Rule> = HashMap::from_iter(rules);

        let n_valid_messages = messages
            .iter()
            .filter(|input| Some("") == consume(input, &all_rules[&0], &all_rules))
            .count();
        println!("Part 1. Number of valid messages: {}", n_valid_messages);
    }
}
