use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::HashMap;
use nom::sequence::{delimited, preceded, separated_pair};

fn main() {
    let file = File::open("input/day14.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut mask0 = 0;
    let mut mask1 = 0;
    let mut x_indices: Vec<usize> = vec![];

    let mut mem1: HashMap<u64, u64> = HashMap::new();
    let mut mem2: HashMap<u64, u64> = HashMap::new();

    for line in buf_reader.lines().map(|l| l.unwrap()) {
        if let Ok((_, (m0, m1))) = parse_mask(&line) {
            mask0 = m0;
            mask1 = m1;

            // Find indices of the 'X' bits
            x_indices.clear();
            let mask_x = format!("{:036b}", m0 ^ m1);
            for (i, c) in mask_x.chars().rev().enumerate() {
                if c == '1' {
                    x_indices.push(i);
                }
            }
        } else if let Ok((_, (address, value))) = parse_mem(&line) {
            // Part 1
            let result_value = (value & mask0) | mask1;
            mem1.insert(address, result_value);

            // Part 2
            for k in 0..=x_indices.len() {
                for flip_indices in x_indices.iter().combinations(k) {
                    let mut result_address = address | mask1;
                    for i in flip_indices {
                        result_address ^= 1 << i;
                    }
                    mem2.insert(result_address, value);
                }
            }
        }
    }

    let answer_part1: u64 = mem1.iter().map(|(_, value)| value).sum();
    println!("Part 1: {}", answer_part1);

    let answer_part2: u64 = mem2.iter().map(|(_, value)| value).sum();
    println!("Part 2: {}", answer_part2);
}

fn parse_mask(input: &str) -> IResult<&str, (u64, u64)> {
    preceded(tag("mask = "), map(take(36usize), |s: &str| {
        let mask0 = s.replace('X', "1");
        let mask1 = s.replace('X', "0");
        (u64::from_str_radix(&mask0, 2).unwrap(), u64::from_str_radix(&mask1, 2).unwrap())
    }))(input)
}

fn parse_mem(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(
        delimited(
            tag("mem["),
            map(digit1, |s: &str| s.parse().unwrap()),
            tag("]"),
        ),
        tag(" = "),
        map(digit1, |s: &str| s.parse().unwrap()),
    )(input)
}