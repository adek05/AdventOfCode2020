use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;
use std::collections::HashSet;

type Coord = (i64, i64);

fn apply_moves(mut start: Coord, moves: String) -> Coord {
    let mut it = moves.chars();
    while let Some(c) = it.next() {
        match c {
            'e' => {
                start = (start.0 + 2, start.1);
            }
            'w' => {
                start = (start.0 - 2, start.1);
            }
            'n' => match it.next() {
                Some('e') => {
                    start = (start.0 + 1, start.1 + 1);
                }
                Some('w') => {
                    start = (start.0 - 1, start.1 + 1);
                }
                _ => panic!("Invalid input - expected 'e' or 'w' after 'n'"),
            },
            's' => match it.next() {
                Some('e') => start = (start.0 + 1, start.1 - 1),
                Some('w') => start = (start.0 - 1, start.1 - 1),
                _ => panic!("Invalid input - expected 'e' or 'w' after 's'"),
            },
            _ => panic!("Invalid input - unexpected character {}", c),
        }
    }

    start
}

fn read_input() -> Result<Vec<String>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file =
        File::open("in/LobbyLayout.in").map_err(|_| "Input file not found".to_string())?;
    Ok(io::BufReader::new(file).lines().map(|line| line.unwrap()).collect())
}

fn main() {
    assert_eq!((0, 0), apply_moves((0, 0), "nwwswee".to_string()));
    assert_eq!((1, -1), apply_moves((0, 0), "esew".to_string()));
    if let Ok(instructions) = read_input() {
        let mut touched_tiles: HashSet<Coord> = HashSet::new();
        for instruction in instructions {
            let result = apply_moves((0, 0), instruction);
            if touched_tiles.contains(&result) {
                touched_tiles.remove(&result);
            } else {
                touched_tiles.insert(result);
            }
        }
        println!("Part 1. Number of flipped tiles is: {}", touched_tiles.len());
    }
}
