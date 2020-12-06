use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use multiset::HashMultiSet;

fn main() {
    let file = File::open("input/day06.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let groups = lines
        .map(|l| l.unwrap())
        .batching(|it| {
            let mut group: Vec<String> = vec![];

            for line in it {
                if line == "" {
                    break;
                }
                group.push(line)
            }

            if group.is_empty() {
                return None;
            }
            return Some(group);
        });

    let mut count_part1 = 0;
    let mut count_part2 = 0;

    for group in groups {
        let mut multiset: HashMultiSet<char> = HashMultiSet::new();
        for line in &group {
            for c in line.chars() {
                multiset.insert(c);
            }
        }

        count_part1 += multiset.distinct_elements().len();

        for c in 'a'..='z' {
            if multiset.count_of(&c) == group.len() {
                count_part2 += 1;
            }
        }
    }

    println!("Part 1: {}", count_part1);
    println!("Part 2: {}", count_part2);
}