#[macro_use]
extern crate scan_rules;

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

enum Instruction {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

struct ProgramState {
    line_number: i32,
    accumulator: i32,
}

fn execute(state: &ProgramState, instruction: &Instruction) -> ProgramState {
    match instruction {
        Instruction::Nop(_) => ProgramState {
            line_number: state.line_number + 1,
            accumulator: state.accumulator,
        },
        Instruction::Acc(value) => ProgramState {
            line_number: state.line_number + 1,
            accumulator: state.accumulator + value,
        },
        Instruction::Jmp(offset) => ProgramState {
            line_number: state.line_number + offset,
            accumulator: state.accumulator,
        },
    }
}

fn read_input() -> Result<Vec<Instruction>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/HandheldHalting.in").map_err(|_| "Input file not found".to_string())?;
    io::BufReader::new(file)
        .lines()
        .map(|line| {
                let l = line.unwrap();
            scan!(
                &l;
                ( "nop", let x: i32) => Instruction::Nop(x),
                ( "acc", let x: i32) => Instruction::Acc(x),
                ( "jmp", let x: i32) => Instruction::Jmp(x),
            ).map_err(|_| "Failed parsing".to_string())
        })
        .collect()
}

#[allow(dead_code)]
fn main() {
    if let Ok(instructions) = read_input() {
        let mut seen_lines: HashSet<i32> = HashSet::new();
        let mut state = ProgramState {
            line_number: 0,
            accumulator: 0,
        };
        while !seen_lines.contains(&state.line_number) {
            seen_lines.insert(state.line_number);
            state = execute(&state, instructions.get(state.line_number as usize).unwrap());
        }
        println!("Part 1. Value in accumulator before looping: {}", state.accumulator);

    } else {
        println!("Failed to parse input");
    }
}
