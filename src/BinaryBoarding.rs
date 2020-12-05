use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

struct BoardingPass {
    code: String,
}

impl BoardingPass {
    pub fn getRow(&self) -> u32 {
        let row_slice = self.code.get(0..7).unwrap();
        Self::code_slice_to_binary(row_slice)
    }

    pub fn getSeat(&self) -> u32 {
        let seat_slice = self.code.get(7..10).unwrap();
        Self::code_slice_to_binary(seat_slice)
    }

    pub fn getSeatID(&self) -> u32 {
        8 * self.getRow() + self.getSeat()
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

    let highest_seat_id = passes.iter().map(|pass| pass.getSeatID()).max();
    println!("Part 1: Max seat id is: {}", highest_seat_id.unwrap());
}
