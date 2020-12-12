#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::scan_a;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq)]
enum Rotation {
    Left,
    Right,
}

struct Move {
    d: Direction,
    offset: i32,
}

struct Turn {
    r: Rotation,
    d: i32,
}

enum Action {
    Move(Move),
    Turn(Turn),
    Forward(i32),
}

#[derive(Clone)]
struct ShipState {
    heading: Direction,
    pos: Position,
}

#[derive(Clone)]
struct Position {
    ns: i32,
    ew: i32,
}

fn turn(start: &ShipState, turn: &Turn) -> ShipState {
    let mut position = start.clone();

    let mut degrees = turn.d % 360;
    if Rotation::Left == turn.r {
        degrees = 360 - degrees;
    }
    while degrees > 0 {
        let new_heading = match position.heading {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        position.heading = new_heading;

        degrees -= 90;
    }
    position
}

fn move_ship(start: &ShipState, m: &Move) -> ShipState {
    let mut new_state = start.clone();
    new_state.pos = move_point(&new_state.pos, m);
    new_state
}

fn move_point(pos: &Position, m: &Move) -> Position {
    let mut new_position = pos.clone();
    match m.d {
        Direction::North => new_position.ns += m.offset,
        Direction::South => new_position.ns -= m.offset,
        Direction::East =>  new_position.ew += m.offset,
        Direction::West =>  new_position.ew -= m.offset,
    };
    new_position
}

fn forward(start: &ShipState, offset: i32) -> ShipState {
    let mut new_position = start.pos.clone();
    match start.heading {
        Direction::North => new_position.ns += offset,
        Direction::South => new_position.ns -= offset,
        Direction::East => new_position.ew += offset,
        Direction::West => new_position.ew -= offset,
    };

    ShipState {
        heading: start.heading,
        pos: new_position,
    }
}

fn read_input() -> Result<Vec<Action>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/RainRisk.in").map_err(|_| "Input file not found".to_string())?;
    io::BufReader::new(file)
        .lines()
        .map(|line| {
            let l = line.unwrap();
            scan!(
                &l;
                ( let c <| scan_a::<char>(), let number: i32) => {
                    match c {
                        'L' => Action::Turn(Turn { r: Rotation::Left, d: number}),
                        'R' => Action::Turn(Turn { r: Rotation::Right, d: number}),
                        'N' => Action::Move(Move { d: Direction::North, offset: number}),
                        'E' => Action::Move(Move { d: Direction::East, offset: number}),
                        'S' => Action::Move(Move { d: Direction::South, offset: number}),
                        'W' => Action::Move(Move { d: Direction::West, offset: number}),
                        'F' => Action::Forward(number),
                        _ => panic!("Invalid character"),
                    }
                }
            )
            .map_err(|e| format!("Failed parsing {}, line: {}", e, l))
        })
        .collect()
}

fn main() {
    match read_input() {
        Ok(actions) => {
            let start = ShipState {
                heading: Direction::East,
                pos: Position { ns: 0, ew: 0 },
            };

            let end: ShipState = actions.iter().fold(start, |acc, action| match action {
                Action::Move(m) => move_ship(&acc, m),
                Action::Turn(t) => turn(&acc, t),
                Action::Forward(offset) => forward(&acc, *offset),
            });

            println!(
                "Part 1: Manhattan distance from start is: {}",
                end.pos.ew.abs() + end.pos.ns.abs()
            );
        }
        Err(e) => println!("{}", e),
    }
}
