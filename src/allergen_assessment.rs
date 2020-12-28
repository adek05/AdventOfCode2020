#![feature(iterator_fold_self)]

#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

#[derive(Debug)]
struct Product {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn parse_product(line: &str) -> Product {
    scan!(
        line;
        ([let ingredients: Word<String>]+, "(contains", [let allergens: Word<String>],+, ")") => Product {
            ingredients: ingredients.iter().cloned().collect(),
            allergens: allergens.iter().cloned().collect(),
        }
    )
    .unwrap()
}

fn read_input() -> Result<Vec<Product>, String> {
    if !path::Path::new("in").exists() {
        return Err("File not found".to_string());
    }
    let file =
        File::open("in/AllergenAssessment.in").map_err(|_| "Input file not found".to_string())?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| parse_product(&line.unwrap()))
        .collect())
}

fn main() {
    if let Ok(products) = read_input() {
        let all_allergens: HashSet<String> = products
            .iter()
            .flat_map(|product| product.allergens.clone())
            .collect();
        let mut ingredients_with_allergen: HashSet<String> = HashSet::new();
        let mut allergen_to_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
        for allergen in all_allergens {
            let ingredients: HashSet<String> = products
                .iter()
                .filter(|product| product.allergens.contains(&allergen))
                .map(|product| product.ingredients.clone())
                .fold_first(|acc, p| acc.intersection(&p).cloned().collect())
                .unwrap();
            ingredients_with_allergen.extend(ingredients.clone());
            allergen_to_ingredients.insert(allergen, ingredients);
        }

        println!(
            "Part 1. Number of times ingredients without allergens show up is: {}",
            products
                .iter()
                .map(|product| product
                    .ingredients
                    .difference(&ingredients_with_allergen)
                    .count())
                .sum::<usize>()
        );

        let mut res: Vec<(String, String)> = vec![];
        while !allergen_to_ingredients.is_empty() {
            let (allergen_ref, ingredients_ref) = allergen_to_ingredients
                .iter()
                .find(|(_, ingredients)| ingredients.len() == 1)
                .unwrap();
            let allergen = allergen_ref.to_string();
            let ingredient = ingredients_ref.iter().next().unwrap().clone();
            res.push((allergen.clone(), ingredient.clone()));
            allergen_to_ingredients.remove(&allergen);
            let keys: Vec<String> = allergen_to_ingredients.keys().cloned().collect();
            keys.iter().for_each(|a| {
                allergen_to_ingredients.entry(a.clone()).and_modify(|is| {
                    is.remove(&ingredient);
                });
            });
        }
        res.sort();
        println!(
            "Part 2. Canonical dangerous ingredient list is: {}",
            res.iter()
                .map(|(_, i)| i)
                .cloned()
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}
