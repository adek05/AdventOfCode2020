use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

struct Grid {
    grid: Vec<String>,
}

impl Grid {
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn has_tree_at(&self, pos: &Position) -> bool {
        self.grid[pos.row].chars().nth(pos.column).unwrap() == '#'
    }
}

struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn next_pos(&self, grid: &Grid) -> Position {
        let row = self.row + 1;
        let column = (self.column + 3) % grid.width();
        Position { row, column }
    }
}

fn read_input() -> Result<Grid, io::Error> {
    if !path::Path::new("in").exists() {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }
    let file = File::open("in/TobogganTrajectory.in")?;
    let grid = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    Ok(Grid { grid })
}

#[allow(dead_code)]
fn main() {
    let slope: Grid = read_input().unwrap();

    let mut position = Position { row: 0, column: 0 };
    let mut number_of_hit_trees = 0;
    while position.row < slope.height() {
        if slope.has_tree_at(&position) {
            number_of_hit_trees += 1;
        }
        position = position.next_pos(&slope);
    }

    println!("Number of trees on the way: {}", number_of_hit_trees);
}
