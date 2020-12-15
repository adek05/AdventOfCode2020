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

#[derive(Clone)]
enum Operation {
    SetBitmask(Bitmask),
    SetValue(u64, u64),      // Memory Location -> Value
    BatchSetValue(u64, u64), // Memory Location -> Value
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
        Operation::BatchSetValue(mem_location, value) => {
            let mut memory = p.memory;
            apply_bitmask_2(format!("{:b}", mem_location), &p.bitmask)
                .iter()
                .for_each(|address| {
                    memory.insert(u64::from_str_radix(address, 2).unwrap(), *value);
                });
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

fn apply_bitmask_2(address: String, bitmask: &Bitmask) -> Vec<String> {
    let mut addresses = vec!["".to_string()];
    let padded_address = format!("{:0>36}", address);
    assert_eq!(padded_address.len(), bitmask.0.len());
    bitmask
        .0
        .chars()
        .zip(padded_address.chars())
        .for_each(|(mask, c)| match mask {
            '0' => addresses = addresses.iter().map(|s| format!("{}{}", s, c)).collect(),
            '1' => addresses = addresses.iter().map(|s| format!("{}1", s)).collect(),
            _ => {
                addresses = addresses
                    .iter()
                    .flat_map(|s| vec![format!("{}1", s), format!("{}0", s)])
                    .collect()
            }
        });
    addresses
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
                program_end
                    .memory
                    .values()
                    .sum::<u64>(),
            );

            let program_end_2 = operations
                .iter()
                .map(|op| match op {
                    Operation::SetValue(location, value) => Operation::BatchSetValue(*location, *value),
                    x => x.clone(),
                })
                .fold(
                    Program {
                        bitmask: Bitmask("".to_string()),
                        memory: HashMap::new(),
                    },
                    |acc, op| execute_operation(acc, &op),
                );
            println!(
                "Part 2. Sum of all memory locations set is: {}",
                program_end_2
                    .memory
                    .values()
                    .sum::<u64>(),
            );
        }
        Err(err) => {
            println!("Parsing error: {}", err);
        }
    }
}
