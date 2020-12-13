use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

struct Bus {
    id: i64,
    offset: i64,
}

fn main() {
    let file = File::open("input/day13.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().map(|l| l.unwrap()).collect_vec();

    let start_time: i64 = lines[0].parse().unwrap();
    let buses = lines[1].split(",").enumerate().fold(vec![], |mut acc, (index, x)| {
        if x == "x" {
            return acc;
        }

        acc.push(Bus {
            id: x.parse::<i64>().unwrap(),
            offset: index as i64,
        });
        acc
    });


    // == Part 1 ==

    let mut time = start_time;
    let earliest_bus_id = 'outer: loop {
        for bus in &buses {
            if time % bus.id == 0 {
                break 'outer bus.id;
            }
        }
        time += 1;
    };

    let answer_part1 = (time - start_time) * earliest_bus_id;
    println!("Part 1: {}", answer_part1);


    // == Part 2 ==

    let residues = buses.iter()
        .map(|Bus { id, offset }| (id - offset % id) % id)
        .collect_vec();
    let moduli = buses.iter()
        .map(|Bus { id, .. }| *id)
        .collect_vec();

    let answer_part2 = chinese_remainder(residues.as_slice(), moduli.as_slice()).unwrap();
    println!("Part 2: {}", answer_part2);
}

///
/// Chinese remainder theorem implementation from:
/// https://rosettacode.org/wiki/Chinese_remainder_theorem
///

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], moduli: &[i64]) -> Option<i64> {
    let prod = moduli.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(moduli) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
