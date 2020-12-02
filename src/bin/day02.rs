#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use itertools::Itertools;

fn main() {
    let file = File::open("input/day02.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    lazy_static! {
        static ref re: Regex = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    }

    let mut valid_part1: u64 = 0;
    let mut valid_part2: u64 = 0;

    for line in lines.map(|line| line.unwrap()) {
        let cap = re.captures(&line).unwrap();

        let min: usize = cap[1].parse().unwrap();
        let max: usize = cap[2].parse().unwrap();
        let chr: char = cap[3].parse().unwrap();
        let pwd: String = cap[4].parse().unwrap();

        let occ: usize = pwd.matches(chr).count();

        if min <= occ && occ <= max {
            valid_part1 += 1;
        }

        let min_chr: char = pwd.chars().nth(min - 1).unwrap();
        let max_chr: char = pwd.chars().nth(max - 1).unwrap();

        if (min_chr == chr) != (max_chr == chr) {
            valid_part2 += 1;
        }
    }

    println!("Part 1: {}", valid_part1);
    println!("Part 1: {}", valid_part2);
}