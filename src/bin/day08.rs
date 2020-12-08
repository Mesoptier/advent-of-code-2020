use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use std::collections::HashSet;
use std::borrow::{Borrow, BorrowMut};

fn main() {
    let file = File::open("input/day08.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut instructions: Vec<(String, i32)> = vec![];
    for line in lines.map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.split(" ").collect();
        let op = parts[0].to_string();
        let arg: i32 = parts[1].parse().unwrap();
        instructions.push((op, arg));
    }

    let (_, acc_part1) = run_program(&instructions);
    println!("Part 1: {}", acc_part1);

    for index in 0..instructions.len() {
        // Change jmp <=> nop
        let (op, _) = instructions[index].clone();
        instructions[index].0 = match op.borrow() {
            "jmp" => "nop".to_string(),
            "nop" => "jmp".to_string(),
            _ => op.to_string()
        };

        let (halts, acc_part2) = run_program(&instructions);

        // Change back jmp <=> nop
        instructions[index].0 = op;

        if halts {
            println!("Part 2: {}", acc_part2);
            break;
        }
    }
}

/// Runs the given program and returns a `(halts, acc)` pair, where:
/// - `halts` indicates whether the program halted
/// - `acc` gives the value of the accumulator after halting or just before looping
fn run_program(instructions: &Vec<(String, i32)>) -> (bool, i32) {
    let mut acc: i32 = 0;
    let mut ip: usize = 0;

    let mut visited: HashSet<usize> = HashSet::new();

    while !visited.contains(&ip) {
        visited.insert(ip);

        let (op, arg) = &instructions[ip];
        match op.borrow() {
            "acc" => {
                acc += arg;
                ip += 1;
            }
            "jmp" => {
                ip = (ip as i32 + arg) as usize;
            }
            "nop" => {
                ip += 1;
            }
            _ => {}
        }

        if ip >= instructions.len() {
            // Halts
            return (true, acc);
        }
    }

    // Loops
    return (false, acc);
}