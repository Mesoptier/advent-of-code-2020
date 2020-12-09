use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    let file = File::open("input/day09.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let numbers = lines.map(|l| l.unwrap().parse::<u64>().unwrap()).collect_vec();

    let window_size = 25;
    let mut window: VecDeque<u64> = VecDeque::new();

    let mut num_part1 = 0;

    for num in numbers.clone() {
        // Fill preamble
        if window.len() < window_size {
            window.push_back(num);
            continue;
        }

        // Try to find a pair of numbers in the window that sum to the next number
        let found_pair = (&window).iter()
            .tuple_combinations()
            .any(|(n1, n2)| {
                n1 + n2 == num
            });

        if !found_pair {
            num_part1 = num;
            break;
        }

        // Shift the window
        window.pop_front();
        window.push_back(num);
    }

    println!("Part 1: {}", num_part1);

    let mut range: VecDeque<u64> = VecDeque::new();
    let mut sum: u64 = 0;

    for num in numbers.clone() {
        // Add next number to the range
        sum += num;
        range.push_back(num);

        // Remove numbers from the range while it's impossible to sum to the target number
        while sum > num_part1 {
            let num_pop = range.pop_front().unwrap();
            sum -= num_pop;
        }

        if sum == num_part1 {
            break;
        }
    }

    let min = range.iter().min().unwrap();
    let max = range.iter().max().unwrap();
    println!("Part 2: {}", min + max);
}