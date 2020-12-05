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

struct Move {
    right: usize,
    down: usize,
}

impl Position {
    pub fn next_pos(&self, movement: &Move, grid: &Grid) -> Position {
        let row = self.row + movement.down;
        let column = (self.column + movement.right) % grid.width();
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

fn count_hit_trees_for_move(slope: &Grid, movement: &Move) -> u64 {
    let mut position = Position { row: 0, column: 0 };
    let mut number_of_hit_trees = 0;
    while position.row < slope.height() {
        if slope.has_tree_at(&position) {
            number_of_hit_trees += 1;
        }
        position = position.next_pos(movement, &slope);
    }
    number_of_hit_trees
}

#[allow(dead_code)]
fn main() {
    let slope: Grid = read_input().unwrap();

    println!(
        "[Part 1] Number of trees on the way: {}",
        count_hit_trees_for_move(&slope, &Move { right: 3, down: 1 })
    );

    let moves = [
        Move { right: 1, down: 1 },
        Move { right: 3, down: 1 },
        Move { right: 5, down: 1 },
        Move { right: 7, down: 1 },
        Move { right: 1, down: 2 },
    ];

    let multiplication: u64 = moves
        .iter()
        .map(|movement| count_hit_trees_for_move(&slope, movement))
        .product();
    println!(
        "[Part 1] Product of number of trees hit on all slopes: {}",
        multiplication
    );
}
