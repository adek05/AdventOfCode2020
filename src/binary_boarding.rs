use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

struct BoardingPass {
    code: String,
}

impl BoardingPass {
    pub fn get_row(&self) -> u32 {
        let row_slice = self.code.get(0..7).unwrap();
        Self::code_slice_to_binary(row_slice)
    }

    pub fn get_seat(&self) -> u32 {
        let seat_slice = self.code.get(7..10).unwrap();
        Self::code_slice_to_binary(seat_slice)
    }

    pub fn get_seat_id(&self) -> u32 {
        8 * self.get_row() + self.get_seat()
    }

    fn code_slice_to_binary(code: &str) -> u32 {
        code.chars().fold(0, |acc, character| {
            2 * acc
                + if character == 'F' || character == 'L' {
                    0
                } else {
                    1
                }
        })
    }
}

fn read_input() -> Result<Vec<BoardingPass>, io::Error> {
    if !path::Path::new("in").exists() {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }
    let file = File::open("in/BinaryBoarding.in")?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| BoardingPass {
            code: line.unwrap(),
        })
        .collect())
}

#[allow(dead_code)]
fn main() {
    let passes = read_input().unwrap();

    let seat_ids = passes.iter().map(|pass| pass.get_seat_id());
    let highest_seat_id = seat_ids.clone().max();
    println!("Part 1: Max seat id is: {}", highest_seat_id.unwrap());

    let mut sorted_seat_ids = seat_ids.collect::<Vec<u32>>();
    sorted_seat_ids.sort();
    for i in 1..(sorted_seat_ids.len() - 1) {
        if sorted_seat_ids.get(i).unwrap() - sorted_seat_ids.get(i - 1).unwrap() == 1
            && sorted_seat_ids.get(i + 1).unwrap() - sorted_seat_ids.get(i).unwrap() == 2
        {
            println!(
                "Part 2: My seat number is: {}",
                sorted_seat_ids.get(i).unwrap() + 1
            );
            return;
        }
    }
}

// x-1, x, x+2
