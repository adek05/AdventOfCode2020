/// To run:
///
///   rustc ReportRepair.rs && ./ReportRepair
///
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_input() -> Result<Vec<i64>, io::Error> {
    let file = File::open("../in/ReportRepairs.in")?;
    let x = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    Ok(x)
}

fn two_that_sum_to_target(input: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut frequency: HashMap<i64, i32> = HashMap::new();
    for number in input.iter() {
        frequency
            .entry(*number)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }

    for number in input {
        if let Some(freq) = frequency.get(&(target - number)) {
            if (*number == (target - number) && freq > &1) || (freq == &1) {
                return Some((*number, target - number));
            }
        }
    }

    None
}

fn three_that_sum_to_2020(input: &[i64]) -> Option<(i64, i64, i64)> {
    let input_length = input.len();
    for i in 0..input_length {
        let mut input_copy: Vec<i64> = input.to_vec();
        let value = input_copy.remove(i);
        if let Some((a, b)) = two_that_sum_to_target(input_copy.as_slice(), 2020 - value) {
            return Some((a, b, value));
        }
    }
    None
}

fn main() {
    let input = read_input().unwrap();

    if let Some((a, b)) = two_that_sum_to_target(&input, 2020) {
        println!("Product of two numbers that sum to 2020 is {}", a * b);
    }

    if let Some((a, b, c)) = three_that_sum_to_2020(&input) {
        println!("Product of two numbers that sum to 2020 is {}", a * b * c);
    }
}
