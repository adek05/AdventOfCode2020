use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

fn read_input() -> Result<(u64, Vec<u64>), String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/ShuttleSearch.in").map_err(|_| "Input file not found".to_string())?;
    let mut line_iter = io::BufReader::new(file).lines();
    let time_at_bus_stop = line_iter.next().unwrap().unwrap().parse::<u64>().unwrap();
    let buses: Vec<u64> = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter_map(|value| {
            if value == "x" {
                None
            } else {
                Some(value.parse::<u64>().unwrap())
            }
        })
        .collect();
    Ok((time_at_bus_stop, buses))
}

fn main() {
    if let Ok((earliest_time, bus_ids)) = read_input() {
        let mut ts: Vec<(u64, u64, u64)> = bus_ids
            .iter()
            .map(|bus_id| {
                let m = earliest_time % bus_id;
                if m == 0 {
                    (earliest_time, *bus_id, 0)
                } else {
                    let wait_time = bus_id - m;
                    (earliest_time + wait_time, *bus_id, wait_time)
                }
            })
            .collect();
        ts.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some((_, bus_id, wait_time)) = ts.first() {
            println!(
                "Part 1. Earliest bus_id * wait_time = {}",
                bus_id * wait_time
            );
        }
    }
}
