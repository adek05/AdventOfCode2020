extern crate itertools;
#[macro_use]
extern crate scan_rules;

use itertools::izip;
use scan_rules::scanner::Word;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Bag {
    adjective: String,
    color: String,
}

#[derive(Debug)]
struct Graph {
    can_be_contained: HashMap<Bag, Vec<(Bag, i32)>>,
    contains: HashMap<Bag, Vec<(Bag, i32)>>,
}

fn parse_line(line: String) -> Option<(Bag, Vec<(Bag, i32)>)> {
    if let Ok((bag, (count, adjectives, colors))) = scan!(
        &line;
        ( let source_adj: Word<String>, let source_color: Word<String>, "bags contain", [ let count: i32, let adj: Word<String>, let color: Word<String>, let _: Word ],+, "." ) => (Bag { adjective: source_adj, color: source_color }, (count, adj, color)),
        ( let source_adj: Word<String>, let source_color: Word<String>, "bags contain no other bags.") => ( Bag { adjective: source_adj, color: source_color }, (vec![], vec![], vec![]))
    ) {
        let targets: Vec<(Bag, i32)> = izip!(count, adjectives, colors)
            .map(|(count, adjective, color)| (Bag { adjective, color }, count))
            .collect();
        Some((bag, targets))
    } else {
        None
    }
}

fn read_input() -> Result<Graph, io::Error> {
    if !path::Path::new("in").exists() {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }
    let file = File::open("in/HandyHaversacks.in")?;
    let mut graph: HashMap<Bag, Vec<(Bag, i32)>> = HashMap::new();
    let mut inverted_graph: HashMap<Bag, Vec<(Bag, i32)>> = HashMap::new();

    io::BufReader::new(file).lines().for_each(|line| {
        if let Some((source, targets)) = parse_line(line.unwrap()) {
            targets.clone().into_iter().for_each(|(target, count)| {
                graph
                    .entry(target)
                    .or_default()
                    .push((source.clone(), count))
            });
            inverted_graph.entry(source).or_default().extend(targets);
        }
    });

    Ok(Graph {
        can_be_contained: graph,
        contains: inverted_graph,
    })
}

#[allow(dead_code)]
fn main() {
    let my_bag = Bag {
        adjective: "shiny".to_string(),
        color: "gold".to_string(),
    };

    if let Ok(graph) = read_input() {
        {
            let mut queue: VecDeque<&Bag> = VecDeque::new();
            queue.push_back(&my_bag);

            let mut result: HashSet<&Bag> = HashSet::new();
            while let Some(bag) = queue.pop_front() {
                if result.contains(bag) {
                    continue;
                }
                result.insert(bag);
                if let Some(bags) = graph.can_be_contained.get(bag) {
                    queue.extend(bags.iter().map(|(bag, _)| bag));
                }
            }
            result.remove(&my_bag);

            println!(
                "Part 1. Number of different outer bags is: {}",
                result.len()
            );
        }

        {
            let mut queue: VecDeque<(&Bag, i32)> = VecDeque::new();
            queue.push_back((&my_bag, 1));

            let mut total_contained_bags = 0;

            while let Some((bag, count)) = queue.pop_front() {
                total_contained_bags += count;
                if let Some(bags) = graph.contains.get(bag) {
                    queue.extend(bags.iter().map(|(bag, how_many)| (bag, count * how_many)));
                } 
            }

            println!(
                "Part 2. Total of bags to buy: {}",
                total_contained_bags - 1,
            );
        }
    }
}
