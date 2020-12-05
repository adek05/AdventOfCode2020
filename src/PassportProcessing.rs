extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum PassportField {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}

impl From<String> for PassportField {
    fn from(value: String) -> PassportField {
        let valid_fields: HashMap<String, PassportField> = [
            ("byr".to_string(), PassportField::BYR),
            ("iyr".to_string(), PassportField::IYR),
            ("eyr".to_string(), PassportField::EYR),
            ("hgt".to_string(), PassportField::HGT),
            ("hcl".to_string(), PassportField::HCL),
            ("ecl".to_string(), PassportField::ECL),
            ("pid".to_string(), PassportField::PID),
            ("cid".to_string(), PassportField::CID),
        ]
        .iter()
        .cloned()
        .collect();

        valid_fields.get(&value).unwrap().clone()
    }
}

#[derive(Debug)]
struct Passport {
    fields: HashMap<PassportField, String>,
}

fn read_input() -> Result<Vec<String>, io::Error> {
    if !path::Path::new("in").exists() {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }
    let file = File::open("in/PassportProcessing.in")?;
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    Ok(lines)
}

fn lines_to_passport(lines: &[String]) -> Passport {
    let fields: HashMap<PassportField, String> = lines
        .iter()
        .map(|line| {
            line.split(" ").map(|field_value| {
                let mut iter = field_value.split(":");
                let field_name = iter.next().unwrap();
                let field_value = iter.next().unwrap();

                (
                    PassportField::from(field_name.to_string()),
                    field_value.to_string(),
                )
            })
        })
        .flatten()
        .collect();

    Passport { fields }
}

fn is_passport_valid(passport: &Passport) -> bool {
    passport.fields.len() == 8
        || (passport.fields.len() == 7 && !passport.fields.contains_key(&PassportField::CID))
}

fn is_byr_valid(passport: &Passport) -> bool {
    let default = "0".to_string();
    let byr = passport.fields.get(&PassportField::BYR).unwrap_or(&default);
    if let Ok(year) = byr.parse::<u32>() {
        return 1920 <= year && year <= 2002;
    }
    false
}

fn is_iyr_valid(passport: &Passport) -> bool {
    let default = "0".to_string();
    let iyr = passport.fields.get(&PassportField::IYR).unwrap_or(&default);
    if let Ok(year) = iyr.parse::<u32>() {
        return 2010 <= year && year <= 2020;
    }
    false
}

fn is_yyr_valid(passport: &Passport) -> bool {
    let default = "0".to_string();
    let eyr = passport.fields.get(&PassportField::EYR).unwrap_or(&default);
    if let Ok(year) = eyr.parse::<u32>() {
        return 2020 <= year && year <= 2030;
    }
    false
}

fn is_hgt_valid(passport: &Passport) -> bool {
    let default = "0".to_string();
    let hgt = passport.fields.get(&PassportField::HGT).unwrap_or(&default);

    let re = Regex::new(r"^(?P<number>[0-9]+)(?P<unit>[a-z]+)$").unwrap();
    if let Some(captures) = re.captures(hgt) {
        let height = captures
            .name("number")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let unit = captures.name("unit").unwrap().as_str();
        if unit == "cm" {
            return 150 <= height && height <= 193;
        }
        if unit == "in" {
            return 59 <= height && height <= 76;
        }
    }
    false
}

fn is_hcl_valid(passport: &Passport) -> bool {
    let default = "0".to_string();
    let hcl = passport.fields.get(&PassportField::HCL).unwrap_or(&default);

    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(hcl)
}

fn is_ecl_valid(passport: &Passport) -> bool {
    let valid_eye_colors: HashSet<String> = HashSet::from_iter(
        [
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string(),
        ]
        .iter()
        .cloned(),
    );
    let default = "0".to_string();
    let hcl = passport.fields.get(&PassportField::ECL).unwrap_or(&default);

    valid_eye_colors.contains(hcl)
}

fn is_pid_valid(passport: &Passport) -> bool {
    let default = "0".to_string();
    let pid = passport.fields.get(&PassportField::PID).unwrap_or(&default);

    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(pid)
}

fn are_passport_fields_valid(passport: &Passport) -> bool {
    [
        is_byr_valid,
        is_iyr_valid,
        is_yyr_valid,
        is_hgt_valid,
        is_hcl_valid,
        is_ecl_valid,
        is_pid_valid,
    ]
    .iter()
    .all(|pred| pred(passport))
        && is_passport_valid(passport)
}

#[allow(dead_code)]
fn main() {
    let input = read_input().unwrap();
    let passports: Vec<Passport> = input
        .split(|line| line.is_empty())
        .map(|lines| lines_to_passport(lines))
        .collect();

    println!(
        "Part 1: Number of valid passports: {}",
        passports
            .iter()
            .filter(|passport| is_passport_valid(passport))
            .count()
    );

    println!(
        "Part 1: Number of valid passports: {}",
        passports
            .iter()
            .filter(|passport| are_passport_fields_valid(passport))
            .count()
    );
}
