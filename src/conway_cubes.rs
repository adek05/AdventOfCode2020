#[macro_use]
extern crate itertools;

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path;

type Coord = (i32, i32, i32);

fn get_neighbors(center: &Coord) -> HashSet<Coord> {
    iproduct!(&[-1, 0, 1], &[-1, 0, 1], &[-1, 0, 1])
        .map(|(x, y, z)| (center.0 + x, center.1 + y, center.2 + z))
        .filter(|cube| cube != center)
        .collect()
}

fn read_input() -> Result<Vec<Coord>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/ConwayCubes.in").map_err(|_| "Input file not found".to_string())?;
    Ok(io::BufReader::new(file)
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            let l = line.unwrap();
            l.chars()
                .enumerate()
                .filter_map(|(y, state)| {
                    if state == '#' {
                        Some((x as i32, y as i32, 0 as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Coord>>()
        })
        .collect())
    // .collect::<Result<_, _>>()
}

fn main() {
    if let Ok(initial_state) = read_input() {
        let mut active_cubes: HashSet<Coord> = HashSet::from_iter(initial_state);
        for _ in 0..6 {
            let mut new_active_cubes: HashSet<Coord> = HashSet::new();

            let inactive_next_to_active: HashSet<Coord> = active_cubes
                .iter()
                .flat_map(|cube| get_neighbors(&cube))
                .filter(|cube| !active_cubes.contains(cube))
                .collect();

            new_active_cubes.extend(active_cubes.iter().filter(|cube| {
                match get_neighbors(&cube).intersection(&active_cubes).count() {
                    2 => true,
                    3 => true,
                    _ => false,
                }
            }));

            new_active_cubes.extend(
                inactive_next_to_active
                    .iter()
                    .filter(|cube| get_neighbors(&cube).intersection(&active_cubes).count() == 3),
            );
            active_cubes = new_active_cubes;
        }
        println!(
            "Part 1. After 6 iterations there are {} active cubes",
            active_cubes.len()
        );
    }
}
