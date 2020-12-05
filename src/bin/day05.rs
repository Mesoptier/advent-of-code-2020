use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::Chars;
use std::ops::Range;

fn main() {
    let file = File::open("input/day05.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut max_seat_id: u64 = 0;
    let mut seat_ids: HashSet<u64> = HashSet::new();

    for line in lines.map(|l| l.unwrap()) {
        let seat_id = compute_seat_id(line);
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }

        seat_ids.insert(seat_id);
    }

    println!("Part 1: {}", max_seat_id);

    for id in 1u64..1022 {
        if seat_ids.contains(&(id - 1)) && seat_ids.contains(&(id + 1)) && !seat_ids.contains(&id) {
            println!("Part 2: {}", id);
        }
    }
}

fn compute_seat_id(pass: String) -> u64 {
    let row =  binary_search(0..127, pass.chars().take(7), 'F', 'B');
    let col =  binary_search(0..7, pass.chars().skip(7), 'L', 'R');
    return row * 8 + col;
}

fn binary_search<I: Iterator<Item=char>>(range: Range<u64>, seq: I, c_lo: char, c_hi: char) -> u64 {
    let mut lo = range.start;
    let mut hi = range.end;

    for c in seq {
        let mid = (lo + hi) / 2;
        if c == c_lo {
            hi = mid;
        } else if c == c_hi  {
            lo = mid + 1;
        } else {
            panic!("Unexpected character '{}' in sequence", c)
        }
    }

    return lo;
}