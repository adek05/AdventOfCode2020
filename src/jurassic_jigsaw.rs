#[macro_use]
extern crate scan_rules;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path;

#[derive(Clone, Debug)]
struct Tile {
    number: usize,
    tile: Vec<String>,
}

impl Tile {
    fn get_borders(&self) -> [String; 4] {
        [
            self.tile[0].clone(),
            self.tile
                .iter()
                .map(|row| row.chars().next().unwrap())
                .collect(),
            self.tile.last().unwrap().clone(),
            self.tile
                .iter()
                .map(|row| row.chars().rev().next().unwrap())
                .collect(),
        ]
    }
}

fn are_borders_matching(border_a: &str, border_b: &str) -> bool {
    border_a == border_b || border_a == border_b.chars().rev().collect::<String>().as_str()
}

fn find_corners(tiles: &[Tile]) -> Vec<Tile> {
    // TileID -> Tile
    let tiles_map: HashMap<usize, Tile> =
        HashMap::from_iter(tiles.iter().map(|tile| (tile.number, tile.clone())));
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (i, tile_a) in tiles.iter().enumerate() {
        for (j, tile_b) in tiles.iter().enumerate() {
            // Only skip when tiles are the same.
            if i == j {
                continue;
            };

            for border_a in &tile_a.get_borders() {
                for border_b in &tile_b.get_borders() {
                    if are_borders_matching(border_a, border_b) {
                        graph
                            .entry(tile_a.number)
                            .and_modify(|v| {
                                v.insert(tile_b.number);
                            })
                            .or_insert_with(|| HashSet::from_iter(vec![tile_b.number]));
                        graph
                            .entry(tile_b.number)
                            .and_modify(|v| {
                                v.insert(tile_a.number);
                            })
                            .or_insert_with(|| HashSet::from_iter(vec![tile_a.number]));
                    }
                }
            }
        }
    }

    // Corners will have exactly 2 matching edges
    graph
        .iter()
        .filter_map(|(k, v)| {
            if v.len() == 2 {
                Some(tiles_map[k].clone())
            } else {
                None
            }
        })
        .collect()
}

fn parse_tile(lines: &[String]) -> Tile {
    let tile_no = scan!(
        &lines[0];
        ("Tile", let tile_no: usize, ..other) => tile_no
    )
    .unwrap();
    Tile {
        number: tile_no,
        tile: lines[1..].to_vec(),
    }
}

fn read_input() -> Result<Vec<Tile>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file =
        File::open("in/JurassicJigsaw.in").map_err(|_| "Input file not found".to_string())?;
    let input: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    Ok(input
        .split(|s| s.is_empty())
        .map(|tile| parse_tile(tile))
        .collect())
}

fn main() {
    if let Ok(tiles) = read_input() {
        let c = find_corners(&tiles);
        assert_eq!(c.len(), 4);
        println!(
            "Part 1. Product of corner tile IDs: {}",
            c.iter().map(|tile| tile.number).product::<usize>()
        );
    }
}
