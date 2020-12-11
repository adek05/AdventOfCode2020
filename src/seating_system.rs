#[macro_use]
extern crate itertools;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::mem;
use std::path;

const EMPTY: char = 'L';
const OCCUPIED: char = '#';

type Grid<T> = Vec<Vec<T>>;

fn read_input() -> Result<Grid<char>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file =
        File::open("in/SeatingSystem.in").map_err(|_| "Input file not found".to_string())?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect())
}

fn is_seat(seats: &Grid<char>, row: i32, column: i32) -> Option<bool> {
    match seats.get(row as usize) {
        None => None,
        Some(r) => match r.get(column as usize) {
            None => None,
            Some(c) => Some(c == &OCCUPIED || c == &EMPTY),
        },
    }
}

fn is_occupied(seats: &Grid<char>, row: i32, column: i32) -> Option<bool> {
    match seats.get(row as usize) {
        None => None,
        Some(r) => match r.get(column as usize) {
            None => None,
            Some(c) => Some(c == &OCCUPIED),
        },
    }
}

fn count_occupied(seats: &Grid<char>, row: i32, column: i32) -> usize {
    iproduct!(&[-1, 0, 1], &[-1, 0, 1])
        .filter(|x| x != &(&0, &0))
        .map(|(row_offset, column_offset)| {
            if Some(true) == is_occupied(seats, row + row_offset, column + column_offset) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn count_occupied_2(seats: &Grid<char>, row: i32, column: i32) -> usize {
    iproduct!(&[-1, 0, 1], &[-1, 0, 1])
        .filter(|x| x != &(&0, &0))
        .map(|(row_offset, column_offset)| {
            let mut mul = 1;
            while Some(false) == is_seat(seats, row + mul * row_offset, column + mul * column_offset) {
                mul += 1;
            }
            if Some(true) == is_occupied(seats, row + mul * row_offset, column + mul * column_offset) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    if let Ok(mut cur_grid) = read_input() {
        let mut next_grid = cur_grid.clone();

        loop {
            for (row_num, row) in cur_grid.iter().enumerate() {
                for (col_num, seat) in row.iter().enumerate() {
                    let adjacent_occupied =
                        count_occupied_2(&cur_grid, row_num as i32, col_num as i32);
                    if seat == &EMPTY && adjacent_occupied == 0 {
                        next_grid[row_num][col_num] = OCCUPIED;
                    } else if seat == &OCCUPIED && adjacent_occupied >= 5 {
                        next_grid[row_num][col_num] = EMPTY;
                    } else {
                        next_grid[row_num][col_num] = *seat;
                    }
                }
            }

            if cur_grid == next_grid {
                println!(
                    "Part 1. When stabilized, number of occupied seats is: {}",
                    cur_grid
                        .iter()
                        .flatten()
                        .filter(|seat| seat == &&OCCUPIED)
                        .count()
                );
                return;
            }
            mem::swap(&mut cur_grid, &mut next_grid);
        }
    }
}
