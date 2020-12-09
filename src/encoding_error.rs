use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

fn read_input() -> Result<Vec<i64>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/EncodingError.in").map_err(|_| "Input file not found".to_string())?;
    io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .parse::<i64>()
                .map_err(|_| "Couldn't parse as i64".to_string())
        })
        .collect()
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
            if freq == &1 {
                return Some((*number, target - number));
            }
        }
    }

    None
}

fn find_weakness(stream: &[i64]) -> Option<i64> {
    let window: usize = 25;
    for i in window..stream.len() {
        if two_that_sum_to_target(&stream[i - window..i], stream[i]).is_none() {
            return Some(stream[i]);
        }
    }
    None
}

fn sequence_that_sums_to(n: i64, stream: &[i64]) -> i64 {
    let mut sum = stream[0];
    let mut left = 0;
    let mut right = 1;

    while sum != n && right < stream.len() {
        if sum < n {
            sum += stream[right];
            right += 1;
        } else {
            sum -= stream[left];
            left += 1;
        }
    }
    // println!("Left: {}, Right: {}", left, right);
    *stream[left..right].iter().max().unwrap() + 
    *stream[left..right].iter().min().unwrap()
    
}

fn main() {
    if let Ok(stream) = read_input() {
        if let Some(weakness) = find_weakness(&stream) {
            println!(
                "First number that is not a sum of two number in last {}",
                weakness
            );

            let res = sequence_that_sums_to(weakness, &stream);
            println!("Encryption weakness: {}", res);
        }
    }
}
