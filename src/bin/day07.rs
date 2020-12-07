use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

fn main() {
    let file = File::open("input/day07.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut rules: HashMap<String, HashMap<String, u64>> = HashMap::new();

    let re: Regex = Regex::new(r"(.+) bags contain (.+)\.").unwrap();
    let re2: Regex = Regex::new(r"(\d+) (.+) bags?").unwrap();

    for line in lines.map(|l| l.unwrap()) {
        let cap = re.captures(&line).unwrap();
        let from = cap[1].to_string();
        let to_unparsed = cap[2].to_string();

        if to_unparsed == "no other bags" {
            rules.insert(from, HashMap::new());
            continue;
        }

        let mut to = HashMap::new();

        let to_parts = to_unparsed.split(", ");
        for to_part in to_parts {
            let cap = re2.captures(to_part).unwrap();
            to.insert(cap[2].to_string(), cap[1].parse().unwrap());
        }
        rules.insert(from, to);
    }

    let mut valid_set: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back("shiny gold".to_string());

    while !queue.is_empty() {
        let target = queue.pop_front().unwrap();

        for (from, to) in &rules {
            if to.contains_key(&target) {
                valid_set.insert(from.to_string());
                queue.push_back(from.to_string());
            }
        }
    }

    println!("Part 1: {}", valid_set.len());
    println!("Part 2: {}", count_bags(&rules, "shiny gold".to_string()));
}

fn count_bags(rules: &HashMap<String, HashMap<String, u64>>, target: String) -> u64 {
    let mut count = 0;

    for (to, n) in rules.get(&target).unwrap() {
        count += (1 + count_bags(rules, to.to_string())) * n;
    }

    return count
}