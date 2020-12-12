use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use nom::character::complete::{anychar, digit1};
use nom::combinator::{map};
use nom::IResult;
use nom::sequence::pair;

fn main() {
    let file = File::open("input/day12.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let instructions: Vec<(char, i32)> = buf_reader.lines().map(|line| {
        let (_, (action, value)) = parse_instruction(&line.unwrap()).unwrap();
        return (action, value);
    }).collect_vec();


    // === PART 1 ===

    let mut e: i32 = 0;
    let mut n: i32 = 0;
    let mut dir = 'E';

    for (action, value) in &instructions {
        match action {
            'N' => n += value,
            'S' => n -= value,
            'E' => e += value,
            'W' => e -= value,
            'L' => {
                for _ in 0..(value / 90) {
                    dir = match dir {
                        'N' => 'W',
                        'W' => 'S',
                        'S' => 'E',
                        'E' => 'N',
                        _ => panic!("Unknown direction"),
                    }
                }
            }
            'R' => {
                for _ in 0..(value / 90) {
                    dir = match dir {
                        'N' => 'E',
                        'E' => 'S',
                        'S' => 'W',
                        'W' => 'N',
                        _ => panic!("Unknown direction"),
                    }
                }
            }
            'F' => match dir {
                'N' => n += value,
                'S' => n -= value,
                'E' => e += value,
                'W' => e -= value,
                _ => panic!("Unknown direction"),
            }
            _ => panic!("Unknown instruction"),
        }
    }

    println!("Part 1: {}", n.abs() + e.abs());


    // === PART 2 ===

    let mut e: i32 = 0;
    let mut n: i32 = 0;
    let mut we: i32 = 10;
    let mut wn: i32 = 1;

    for (action, value) in &instructions {
        match action {
            'N' => wn += value,
            'S' => wn -= value,
            'E' => we += value,
            'W' => we -= value,
            'L' => {
                for _ in 0..(value / 90) {
                    let t = we;
                    we = -wn;
                    wn = t;
                }
            }
            'R' => {
                for _ in 0..(value / 90) {
                    let t = wn;
                    wn = -we;
                    we = t;
                }
            }
            'F' => {
                for _ in 0..(*value) {
                    n += wn;
                    e += we;
                }
            }
            _ => panic!("Unknown instruction"),
        }
    }

    println!("Part 2: {}", n.abs() + e.abs());
}

fn parse_instruction(input: &str) -> IResult<&str, (char, i32)> {
    pair(
        anychar,
        map(digit1, |s: &str| s.parse::<i32>().unwrap()),
    )(input)
}