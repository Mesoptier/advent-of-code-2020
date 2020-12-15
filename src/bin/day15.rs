use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use std::time::Instant;
use rustc_hash::FxHashMap;

fn main() {
    let mut file = File::open("input/day15.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let starting_numbers: Vec<u32> = input.split(",").map(|s| s.parse().unwrap()).collect_vec();

    let now = Instant::now();
    println!("Part 1: {}", compute_answer(&starting_numbers, 2020));
    println!("Part 2: {}", compute_answer(&starting_numbers, 30000000));
    println!("{}", now.elapsed().as_millis());
}

fn compute_answer(starting_numbers: &Vec<u32>, target: usize) -> u32 {
    let mut prev_num: u32 = 0;
    let mut last_spoken: FxHashMap<u32, u32> = FxHashMap::default();

    for idx in 0usize..target {
        let num = if idx < starting_numbers.len() {
            starting_numbers[idx]
        } else if let Some(last_idx) =  last_spoken.get(&prev_num) {
            idx as u32 - *last_idx - 1
        } else {
            0
        };

        if idx > 0 {
            last_spoken.insert(prev_num, (idx - 1) as u32);
        }
        prev_num = num;
    }

    return prev_num;
}
