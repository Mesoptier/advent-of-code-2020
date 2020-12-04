use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let file = File::open("input/day04.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut record: HashMap<String, String> = HashMap::new();
    let mut valid_part1 = 0;
    let mut valid_part2 = 0;

    for line in lines.map(|l| l.unwrap()) {
        if line == "" {
            if has_required_keys(&record) {
                valid_part1 += 1;
                if is_valid(&record) {
                    valid_part2 += 1;
                }
            }

            record.clear();
            continue;
        }

        for part in line.split(' ') {
            let v: Vec<&str> = part.split(':').collect();
            let key = v[0];
            let value = v[1];

            record.insert(key.to_string(), value.to_string());
        }
    }

    if has_required_keys(&record) {
        valid_part1 += 1;
        if is_valid(&record) {
            valid_part2 += 1;
        }
    }

    println!("Part 1: {}", valid_part1);
    println!("Part 2: {}", valid_part2);
}

fn has_required_keys(record: &HashMap<String, String>) -> bool {
    record.contains_key("byr")
        && record.contains_key("iyr")
        && record.contains_key("eyr")
        && record.contains_key("hgt")
        && record.contains_key("hcl")
        && record.contains_key("ecl")
        && record.contains_key("pid")
}

fn is_valid(record: &HashMap<String, String>) -> bool {
    if !has_required_keys(record) {
        return false
    }

    return validate_year(record.get("byr").unwrap(), 1920, 2002)
        && validate_year(record.get("iyr").unwrap(), 2010, 2020)
        && validate_year(record.get("eyr").unwrap(), 2020, 2030)
        && validate_height(record.get("hgt").unwrap(), 150, 193, 59, 76)
        && validate_color(record.get("hcl").unwrap())
        && validate_enum(record.get("ecl").unwrap(), vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
        && validate_pid(record.get("pid").unwrap());
}

fn validate_year(value: &str, min: i64, max: i64) -> bool {
    let re: Regex = Regex::new(r"^\d{4}$").unwrap();

    if !re.is_match(value) {
        return false
    }

    let v: i64 = value.parse().unwrap();
    return min <= v && v <= max;
}

fn validate_height(value: &str, min_cm: i64, max_cm: i64, min_in: i64, max_in: i64) -> bool {
    let re: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();

    if !re.is_match(value) {
        return false
    }

    let cap = re.captures(value).unwrap();
    let height: i64 = cap[1].parse().unwrap();
    let unit: &str = &cap[2];

    match unit {
        "cm" => min_cm <= height && height <= max_cm,
        "in" => min_in <= height && height <= max_in,
        _ => false
    }
}

fn validate_color(value: &str) -> bool {
    let re: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    return re.is_match(value)
}

fn validate_enum(value: &str, allowed: Vec<&str>) -> bool {
    for a in allowed {
        if value == a {
            return true
        }
    }
    return false
}

fn validate_pid(value: &str) -> bool {
    let re: Regex = Regex::new(r"^\d{9}$").unwrap();
    return re.is_match(value)
}