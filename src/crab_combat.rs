use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

fn deck_score(card_deck: &[u64]) -> u64 {
    card_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| ((i + 1) as u64) * card)
        .sum()
}

fn simulate_game(deck_a: &[u64], deck_b: &[u64]) -> Vec<u64> {
    let mut player1: VecDeque<u64> = deck_a.iter().cloned().collect();
    let mut player2: VecDeque<u64> = deck_b.iter().cloned().collect();

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        if card1 > card2 {
            player1.extend(&[card1, card2]);
        } else {
            player2.extend(&[card2, card1]);
        }
    }

    if player1.is_empty() {
        player2.into_iter().collect()
    } else {
        player1.into_iter().collect()
    }
}

fn read_input() -> Result<(Vec<u64>, Vec<u64>), String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file = File::open("in/CrabCombat.in").map_err(|_| "Input file not found".to_string())?;
    let input: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut it = input.split(|line| line.is_empty());
    let first_player = it.next().unwrap();
    let second_player = it.next().unwrap();
    Ok((
        first_player
            .iter()
            .skip(1)
            .map(|value| value.parse::<u64>().unwrap())
            .collect(),
        second_player
            .iter()
            .skip(1)
            .map(|value| value.parse::<u64>().unwrap())
            .collect(),
    ))
}

fn main() {
    if let Ok((player1, player2)) = read_input() {
        println!(
            "Part 1. Winning deck score: {}",
            deck_score(&simulate_game(&player1, &player2))
        );
    }
}
