use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path;

fn next_number(
    mut state: HashMap<u64, Vec<usize>>,
    current_pos: usize,
    last_number: u64,
) -> (HashMap<u64, Vec<usize>>, u64) {
    let next_to_be_spoken = if let Some(idxs) = state.get(&last_number) {
        if idxs.len() == 1 {
            0
        } else {
            idxs.last().unwrap() - idxs[idxs.len() - 2]
        }
    } else {
        0
    };
    state
        .entry(next_to_be_spoken as u64)
        .and_modify(|v| v.push(current_pos))
        .or_insert_with(|| vec![current_pos]);
    (state, next_to_be_spoken as u64)
}

fn read_input() -> Result<Vec<u64>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/RambunctiousRecitation.in")
        .map_err(|_| "Input file not found".to_string())?;
    let mut lines = io::BufReader::new(file).lines();
    let line = lines.next().unwrap();
    line.unwrap()
        .split(',')
        .map(|number| number.parse::<u64>().map_err(|_| format!("")))
        .collect()
}

fn main() {
    if let Ok(initial_numbers) = read_input() {
        let mut state: HashMap<u64, Vec<usize>> =
            HashMap::from_iter(initial_numbers.iter().enumerate().map(|(a, b)| (*b, vec![a])));

        let mut last_number = *initial_numbers.last().unwrap();
        let current_pos = initial_numbers.len();
        for pos in current_pos..2020 {
            let (new_state, new_last_number) = next_number(state, pos, last_number);
            state = new_state;
            last_number = new_last_number;
        }
        println!("Part 1. 2020th number is: {}", last_number);
    }
}
