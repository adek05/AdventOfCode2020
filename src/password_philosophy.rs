#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

#[derive(Clone, Debug)]
struct Input {
    min_cnt: usize,
    max_cnt: usize,
    letter: char,
    password: String,
}

struct InputPartTwo {
    first_position: usize,
    second_position: usize,
    letter: char,
    password: String,
}

impl From<Input> for InputPartTwo {
    fn from(input: Input) -> InputPartTwo {
        InputPartTwo {
            first_position: input.min_cnt - 1,
            second_position: input.max_cnt - 1,
            letter: input.letter,
            password: input.password,
        }
    }
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

fn is_password_valid_part_one(candidate: &Input) -> bool {
    let letter_count: usize = candidate
        .password
        .chars()
        .filter(|l| l == &candidate.letter)
        .count();

    candidate.min_cnt <= letter_count && letter_count <= candidate.max_cnt
}

fn is_password_valid_part_two(candidate: &InputPartTwo) -> bool {
    let password = &candidate.password;

    (((password.chars().nth(candidate.first_position).unwrap() == candidate.letter) as u32)
        + ((password.chars().nth(candidate.second_position).unwrap() == candidate.letter) as u32))
        == 1
}

#[allow(dead_code)]
fn main() {
    let input: Vec<Input> = read_input().unwrap();

    let number_of_valid_passwords_part_1 = input
        .iter()
        .filter(|input| is_password_valid_part_one(input))
        .count();
    let number_of_valid_passwords_part_2 = input
        .into_iter()
        .filter(|input| is_password_valid_part_two(&InputPartTwo::from(input.clone())))
        .count();

    println!(
        "Number of valid passwords in Part 1 is: {}",
        number_of_valid_passwords_part_1
    );

    println!(
        "Number of valid passwords in Part 2 is: {}",
        number_of_valid_passwords_part_2
    );
}
