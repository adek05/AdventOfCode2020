use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path;

struct Group {
    pub answers: Vec<String>,
}

fn read_input() -> Result<Vec<String>, io::Error> {
    if !path::Path::new("in").exists() {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }
    let file = File::open("in/CustomCustoms.in")?;
    let x = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    Ok(x)
}

fn parse_groups(input: &[String]) -> Vec<Group> {
    input
        .split(|line| line.is_empty())
        .map(|answers| Group {
            answers: answers.to_vec(),
        })
        .collect()
}

fn get_answers_for_group(group: &Group) -> HashSet<char> {
    HashSet::from_iter(group.answers.iter().flat_map(|answer| answer.chars()))
}

#[allow(dead_code)]
fn main() {
    let input = read_input().unwrap();

    let sum_of_answers: usize = parse_groups(&input)
        .iter()
        .map(|group| get_answers_for_group(group).len())
        .sum();

    println!("Part 1: Sum of number of questions each group answered yes: {}", sum_of_answers);
}
