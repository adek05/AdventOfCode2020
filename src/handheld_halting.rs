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
    let file =
        File::open("in/HandheldHalting.in").map_err(|_| "Input file not found".to_string())?;
    io::BufReader::new(file)
        .lines()
        .map(|line| {
            let l = line.unwrap();
            scan!(
                &l;
                ( "nop", let x: i32) => Instruction::Nop(x),
                ( "acc", let x: i32) => Instruction::Acc(x),
                ( "jmp", let x: i32) => Instruction::Jmp(x),
            )
            .map_err(|_| "Failed parsing".to_string())
        })
        .collect()
}

fn run_program(
    seen_lines: &mut HashSet<i32>,
    instructions: &[Instruction],
    start_state: ProgramState,
) -> (ProgramState, HashSet<i32>) {
    let mut lines_seen_in_this_run: HashSet<i32> = HashSet::new();
    let mut state = start_state;

    while !seen_lines.contains(&state.line_number)
        && state.line_number < (instructions.len() as i32)
    {
        lines_seen_in_this_run.insert(state.line_number);
        seen_lines.insert(state.line_number);
        state = execute(
            &state,
            instructions.get(state.line_number as usize).unwrap(),
        );
    }
    (state, lines_seen_in_this_run)
}

#[allow(dead_code)]
fn main() {
    if let Ok(mut instructions) = read_input() {
        let mut seen_lines: HashSet<i32> = HashSet::new();
        {
            let (end_state, _) = run_program(
                &mut seen_lines,
                &instructions,
                ProgramState {
                    line_number: 0,
                    accumulator: 0,
                },
            );
            println!(
                "Part 1. Value in accumulator before looping: {}",
                end_state.accumulator
            );
        }

        let lines_from_start: HashSet<i32> = seen_lines.iter().cloned().collect();
        let mut lines_to_end: HashSet<i32> = HashSet::new();
        for idx in 0..instructions.len() {
            let (end_state, seen_lines_in_run) = run_program(&mut seen_lines, &instructions, 
            ProgramState {
                line_number: (idx as i32),
                accumulator: 0,
            }
            );
            if end_state.line_number == (instructions.len() as i32)
                || lines_to_end.contains(&end_state.line_number)
            {
                lines_to_end.extend(seen_lines_in_run);
            }
        }

        for line in lines_from_start {
            let instruction = instructions.get(line as usize).unwrap();
            let next_instruction = match *instruction {
                Instruction::Nop(x) => Instruction::Jmp(x),
                Instruction::Jmp(x) => Instruction::Nop(x),
                Instruction::Acc(x) => Instruction::Acc(x),
            };
            let next_state = execute(
                &ProgramState {
                    line_number: line,
                    accumulator: 0,
                },
                &next_instruction,
            );
            if lines_to_end.contains(&next_state.line_number) {
                instructions[(line as usize)] = next_instruction;
            }
        }

        {
            seen_lines.clear();
            let (end_state, _) = run_program(
                &mut seen_lines,
                &instructions,
                ProgramState {
                    line_number: 0,
                    accumulator: 0,
                },
            );
            println!(
                "Part 2. Value in accumulator before looping: {}",
                end_state.accumulator
            );
        }
    } else {
        println!("Failed to parse input");
    }
}
