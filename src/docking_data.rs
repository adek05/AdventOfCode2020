#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

#[derive(Clone)]
struct Bitmask(String);

struct Program {
    bitmask: Bitmask,
    memory: HashMap<u64, u64>,
}

enum Operation {
    SetBitmask(Bitmask),
    SetValue(u64, u64), // Memory Location -> Value
}

fn execute_operation(p: Program, op: &Operation) -> Program {
    match op {
        Operation::SetBitmask(bitmask) => Program {
            bitmask: bitmask.clone(),
            memory: p.memory,
        },
        Operation::SetValue(mem_location, value) => {
            let mut memory = p.memory;
            memory.insert(*mem_location, apply_bitmask(*value, &p.bitmask));
            Program {
                bitmask: p.bitmask,
                memory,
            }
        }
    }
}

fn apply_bitmask(mut value: u64, bitmask: &Bitmask) -> u64 {
    for (idx, bit) in bitmask.0.chars().rev().enumerate() {
        match bit {
            '0' => value &= !(1 << idx),
            '1' => value |= 1 << idx,
            _ => (),
        }
    }
    value
}

fn read_input() -> Result<Vec<Operation>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/DockingData.in").map_err(|_| "Input file not found".to_string())?;
    io::BufReader::new(file).lines().map(
        |line| {
            let l = line.unwrap();
            scan!(
                &l;
                ("mask = ", let mask: Word<String>) => Operation::SetBitmask(Bitmask(mask)),
                ("mem[", let location: u64, "] = ", let value: u64) => Operation::SetValue(location, value),
            ).map_err(|_| format!("Could not parse line {}", l))
        }
    ).collect()
}

fn main() {
    match read_input() {
        Ok(operations) => {
            let program_end = operations.iter().fold(
                Program {
                    bitmask: Bitmask("".to_string()),
                    memory: HashMap::new(),
                },
                |acc, op| execute_operation(acc, op),
            );
            println!(
                "Part 1. Sum of all memory locations set is: {}",
                program_end.memory.values().sum::<u64>()
            );
        },
        Err(err) => {
            println!("Parsing error: {}", err);
        }
    }
}
