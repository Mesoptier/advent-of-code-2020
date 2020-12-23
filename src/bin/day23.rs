use std::fs::File;
use std::io::Read;
use std::time::Instant;

use itertools::Itertools;

fn main() {
    let mut input = String::default();
    let mut file = File::open("input/day23.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let now = Instant::now();

    // == Part 1 ==

    let mut cups: Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect_vec();

    for _step in 0..100 {
        let cur_cup = cups.remove(0);

        let pu1 = cups.remove(0);
        let pu2 = cups.remove(0);
        let pu3 = cups.remove(0);

        let mut dst_cup = (cur_cup + 7) % 9 + 1;
        while dst_cup == pu1 || dst_cup == pu2 || dst_cup == pu3 {
            dst_cup = (dst_cup + 7) % 9 + 1;
        }

        let dst_idx = cups.iter().position(|&cup| cup == dst_cup).unwrap();
        cups.insert(dst_idx + 1, pu1);
        cups.insert(dst_idx + 2, pu2);
        cups.insert(dst_idx + 3, pu3);

        cups.push(cur_cup);
    }

    loop {
        let cup = cups.remove(0);
        if cup == 1 {
            break;
        } else {
            cups.push(cup);
        }
    }

    println!("Part 1: {}", cups.iter().join(""));

    // == Part 2 ==

    let length: usize = 1000000;
    let steps: usize = 10000000;

    let cups = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect_vec();
    let mut next_map: Vec<usize> = vec![0; length];

    let mut prev_cup = length - 1;
    for i in 0..length {
        let cur_cup = if i < cups.len() {
            cups[i] - 1
        } else {
            i
        };

        next_map[prev_cup] = cur_cup;
        prev_cup = cur_cup;
    }

    let mut cur_cup = cups[0] - 1;
    for _step in 0..steps {
        let pu1 = next_map[cur_cup];
        let pu2 = next_map[pu1];
        let pu3 = next_map[pu2];

        next_map[cur_cup] = next_map[pu3];

        let mut dst_cup = (cur_cup + length - 1) % length;
        while dst_cup == pu1 || dst_cup == pu2 || dst_cup == pu3 {
            dst_cup = (dst_cup + length - 1) % length;
        }

        let dst_next_cup = next_map[dst_cup];
        next_map[dst_cup] = pu1;
        next_map[pu3] = dst_next_cup;

        cur_cup = next_map[cur_cup];
    }

    let c1 = next_map[0];
    let c2 = next_map[c1];
    println!("Part 2: {}", (c1 + 1) * (c2 + 1));

    println!("Time: {}ms", now.elapsed().as_millis());
}