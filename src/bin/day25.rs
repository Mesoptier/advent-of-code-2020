use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    let file = File::open("input/day25.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let card_pub_key = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    let door_pub_key = lines.next().unwrap().unwrap().parse::<u64>().unwrap();

    let now = Instant::now();

    let card_loop_size = find_loop_size(7, card_pub_key);

    let mut encryption_key = 1;
    for i in 0..card_loop_size {
        encryption_key *= door_pub_key;
        encryption_key %= 20201227;
    }

    println!("Part 1: {}", encryption_key);

    println!("Time: {}ms", now.elapsed().as_millis());
}

fn find_loop_size(subject: u64, pub_key: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;

    loop {
        loop_size += 1;

        value *= subject;
        value %= 20201227;

        if value == pub_key {
            return loop_size;
        }
    }
}