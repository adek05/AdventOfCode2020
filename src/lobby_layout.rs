use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

type Coord = (i64, i64);

fn apply_moves(mut start: Coord, moves: &str) -> Coord {
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
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect())
}

fn neighbors(tile: Coord) -> [Coord; 6] {
    [
        apply_moves(tile, "ne"),
        apply_moves(tile, "nw"),
        apply_moves(tile, "se"),
        apply_moves(tile, "sw"),
        apply_moves(tile, "e"),
        apply_moves(tile, "w"),
    ]
}

fn main() {
    assert_eq!((0, 0), apply_moves((0, 0), "nwwswee"));
    assert_eq!((1, -1), apply_moves((0, 0), "esew"));

    if let Ok(instructions) = read_input() {
        let mut black_tiles: HashSet<Coord> = HashSet::new();
        for instruction in instructions {
            let result = apply_moves((0, 0), &instruction);
            if black_tiles.contains(&result) {
                black_tiles.remove(&result);
            } else {
                black_tiles.insert(result);
            }
        }
        println!(
            "Part 1. Number of flipped tiles is: {}",
            black_tiles.len()
        );

        for _ in 0..100 {
            let white_candidates: HashSet<Coord> = black_tiles
                .iter()
                .flat_map(|tile| neighbors(*tile).to_vec())
                .collect::<HashSet<Coord>>()
                .difference(&black_tiles)
                .cloned()
                .collect();

            let mut new_black_tiles: HashSet<Coord> = black_tiles
                .iter()
                .filter(|black_tile| {
                    let n_black_neighbords = neighbors(**black_tile)
                        .iter()
                        .cloned()
                        .collect::<HashSet<Coord>>()
                        .intersection(&black_tiles)
                        .count();
                    n_black_neighbords == 1 || n_black_neighbords == 2
                })
                .cloned()
                .collect();

            let black_tiles_from_white_tiles: HashSet<Coord> = white_candidates
                .iter()
                .filter(|white_tile| {
                    neighbors(**white_tile)
                        .iter()
                        .cloned()
                        .collect::<HashSet<Coord>>()
                        .intersection(&black_tiles)
                        .count()
                        == 2
                })
                .cloned()
                .collect();

            new_black_tiles.extend(&black_tiles_from_white_tiles);
            black_tiles = new_black_tiles;
        }
        println!(
            "Part 2. Number of black tiles after 100 days is {}",
            black_tiles.len()
        );
    }
}
