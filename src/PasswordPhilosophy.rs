#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

#[derive(Debug)]
struct Input {
    min_cnt: usize,
    max_cnt: usize,
    letter: char,
    password: String,
}

// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
fn read_input() -> Result<Vec<Input>, io::Error> {
    if !path::Path::new("in").exists() {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }
    let file = File::open("in/PasswordPhilosophy.in")?;
    let x = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let value = line.unwrap();
            let_scan!(&value; (let min_cnt: usize, "-", let max_cnt: usize, " ", let letter: char, ": ", let password:Word));

            Input{min_cnt, max_cnt, letter, password: password.to_string()}
        })
        .collect();
    Ok(x)
}

fn is_password_valid(candidate: &Input) -> bool {
    let letter_count: usize = candidate
        .password
        .chars()
        .filter(|l| l == &candidate.letter)
        .count();

    candidate.min_cnt <= letter_count && letter_count <= candidate.max_cnt
}

#[allow(dead_code)]
fn main() {
    let input: Vec<Input> = read_input().unwrap();

    let number_of_valid_passwords = input
        .iter()
        .filter(|input| is_password_valid(input))
        .count();

    println!(
        "Number of valid passwords is: {}",
        number_of_valid_passwords
    );
}
