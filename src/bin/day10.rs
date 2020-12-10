use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use multiset::HashMultiSet;
use std::collections::HashMap;

fn main() {
    let file = File::open("input/day10.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut numbers = buf_reader.lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap()).collect_vec();

    numbers.sort_unstable();

    let mut prev_num = 0;
    let mut set = HashMultiSet::new();

    for num in &numbers {
        set.insert(num - prev_num);
        prev_num = *num;
    }
    set.insert(3);

    let part1 = set.count_of(&1) * set.count_of(&3);
    println!("Part 1: {}", part1);

    let mut cache: HashMap<u64, u64> = HashMap::new();
    println!("Part 2: {}", count_arrangements(0, &numbers, &mut cache));
}

fn count_arrangements(from: u64, input: &Vec<u64>, cache: &mut HashMap<u64, u64>) -> u64 {
    if cache.contains_key(&from) {
        return *cache.get(&from).unwrap();
    }

    let mut count = 0;

    for num in input.iter() {
        if *num <= from {
            continue;
        } else if *num <= from + 3 {
            count += count_arrangements(*num, input, cache);
        } else {
            break;
        }
    }

    if count == 0 {
        count = 1;
    }

    cache.insert(from, count);

    return count
}