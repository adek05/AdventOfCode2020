/// To run:
/// 
///   rustc ReportRepair.rs && ./ReportRepair
/// 

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn readInput() -> Result<Vec<i64>, io::Error> {
    let file = File::open("../in/ReportRepairs.in")?;
    let x = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    Ok(x)
}

fn main() {
    let input = readInput().unwrap();
    let mut frequency: HashMap<i64, i32> = HashMap::new();
    for number in input.iter() {
        frequency
            .entry(*number)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }

    for number in input {
        if let Some(freq) = frequency.get(&(2020 - number)) {
            if (number == (2020 - number) && freq > &1) || (freq == &1) {
                println!("{}", number * (2020 - number));
            }
        }
    }
}
