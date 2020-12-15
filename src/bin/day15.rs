use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use std::time::Instant;

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
    let mut last_spoken = vec![u32::MAX; target];

    for (idx, num) in starting_numbers.iter().enumerate() {
        last_spoken[*num as usize] = idx as u32;
        prev_num = *num;
    }

    for idx in starting_numbers.len()..target {
        let num = match last_spoken[prev_num as usize] {
            u32::MAX => 0,
            last_idx => (idx as u32) - last_idx - 1
        };

        last_spoken[prev_num as usize] = (idx as u32) - 1;
        prev_num = num;
    }

    return prev_num;
}
