use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

fn read_input() -> Result<(u64, Vec<(u64, u64)>), String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/ShuttleSearch.in").map_err(|_| "Input file not found".to_string())?;
    let mut line_iter = io::BufReader::new(file).lines();
    let time_at_bus_stop = line_iter.next().unwrap().unwrap().parse::<u64>().unwrap();
    let buses: Vec<(u64, u64)> = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(idx, value)| {
            if value == "x" {
                None
            } else {
                Some((idx as u64, value.parse::<u64>().unwrap()))
            }
        })
        .collect();
    Ok((time_at_bus_stop, buses))
}

fn find_time(buses: &[(u64, u64)]) -> u64 {
    let mut lcm = 1;
    let mut first_matching = 0;
    for (mut modulus, bus_id) in buses {
        modulus %= bus_id;
        let mut cur = first_matching;
        while (cur + modulus) % bus_id != 0 {
            cur += lcm;
        }
        first_matching = cur;
        lcm *= *bus_id; // Should be real LCM, but all input numbers are prime.
    }
    first_matching
}

fn main() {
    if let Ok((earliest_time, bus_ids)) = read_input() {
        let mut departure_times: Vec<(u64, u64, u64)> = bus_ids
            .iter()
            .map(|(_, bus_id)| {
                let m = earliest_time % bus_id;
                if m == 0 {
                    (earliest_time, *bus_id, 0)
                } else {
                    let wait_time = bus_id - m;
                    (earliest_time + wait_time, *bus_id, wait_time)
                }
            })
            .collect();
        departure_times.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some((_, bus_id, wait_time)) = departure_times.first() {
            println!(
                "Part 1. Earliest bus_id * wait_time = {}",
                bus_id * wait_time
            );
        }

        let time = find_time(&bus_ids);
        println!("Part 2. Earliest time for competition: {}", time);
    }
}
