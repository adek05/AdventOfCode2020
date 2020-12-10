use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

fn read_input() -> Result<Vec<i64>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/AdapterArray.in").map_err(|_| "Input file not found".to_string())?;
    io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .parse::<i64>()
                .map_err(|_| "Couldn't parse as i64".to_string())
        })
        .collect()
}

fn histogram(input: &[i64]) -> (i64, i64) {
    let (mut ones, mut threes) = (0, 0);
    for i in 1..input.len() {
        let jolt_difference = input[i] - input[i - 1];
        if jolt_difference == 1 {
            ones += 1;
        } else if jolt_difference == 3 {
            threes += 1;
        } else {
            panic!("Incompatible adapters");
        }
    }

    (ones, threes)
}

fn main() {
    if let Ok(mut input) = read_input() {
        input.push(0);
        input.sort();
        let (ones, threes) = histogram(&input);
        println!("Part 1. #1s * #3s = {}", ones * (threes + 1));
    }
}
