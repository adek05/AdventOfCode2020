use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
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

fn simulate_game(
    deck_a: &[u64],
    deck_b: &[u64],
    mut memory: &mut HashMap<(Vec<u64>, Vec<u64>), (Vec<u64>, usize)>,
) -> (Vec<u64>, usize) {
    let state_key = (deck_a.to_vec(), deck_b.to_vec());
    if let Some(ret) = memory.get(&state_key) {
        // println!("Hit cache");
        return ret.clone();
    }

    let mut player1: VecDeque<u64> = deck_a.iter().cloned().collect();
    let mut player2: VecDeque<u64> = deck_b.iter().cloned().collect();

    let mut seen_states: HashSet<(VecDeque<u64>, VecDeque<u64>)> = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        // Deal with seen states
        if seen_states.contains(&(player1.clone(), player2.clone())) {
            return (player1.iter().cloned().collect(), 1);
        }
        seen_states.insert((player1.clone(), player2.clone()));

        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        // Deal if recursion cannot be played
        if (card1 as usize) > player1.len() || (card2 as usize) > player2.len() {
            if card1 > card2 {
                player1.extend(&[card1, card2]);
            } else {
                player2.extend(&[card2, card1]);
            }
        } else {
            match simulate_game(
                &player1.iter().cloned().take(card1 as usize).collect::<Vec<u64>>(),
                &player2.iter().cloned().take(card2 as usize).collect::<Vec<u64>>(),
                &mut memory,
            ) {
                (_, 1) => player1.extend(&[card1, card2]),
                (_, 2) => player2.extend(&[card2, card1]),
                _ => panic!("Unreachable"),
            }
        }
    }

    if player1.is_empty() {
        let ret = (player2.into_iter().collect(), 2);
        memory.insert(state_key, ret.clone());
        ret
    } else {
        let ret = (player1.into_iter().collect(), 1);
        memory.insert(state_key, ret.clone());
        ret
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
    let mut memory: HashMap<(Vec<u64>, Vec<u64>), (Vec<u64>, usize)> = HashMap::new();

    if let Ok((player1, player2)) = read_input() {
        let (winning_deck, _) = simulate_game(&player1, &player2, &mut memory);
        println!("Part 1. Winning deck score: {}", deck_score(&winning_deck));
    }
}
