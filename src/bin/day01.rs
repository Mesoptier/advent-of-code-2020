use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn main() {
    let file = File::open("input/day01.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let numbers: Vec<i64> = buf_reader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

    for (n1, n2) in numbers.iter().tuple_combinations() {
        if n1 + n2 == 2020 {
            println!("Part 1: {}", n1 * n2);
            break;
        }
    }

    for (n1, n2, n3) in numbers.into_iter().tuple_combinations() {
        if n1 + n2 + n3 == 2020 {
            println!("Part 2: {}", n1 * n2 * n3);
            break;
        }
    }
}