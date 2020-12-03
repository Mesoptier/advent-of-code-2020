use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn main() {
    let file = File::open("input/day03.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut grid: Vec<String> = vec![];
    for line in lines.map(|l| l.unwrap()) {
        grid.push(line);
    }

    let count_part1 = test_slope(&grid, 3, 1);
    let count_part2 = test_slope(&grid, 1, 1)
        * test_slope(&grid, 3, 1)
        * test_slope(&grid, 5, 1)
        * test_slope(&grid, 7, 1)
        * test_slope(&grid, 1, 2);

    println!("Part 1: {}", count_part1);
    println!("Part 2: {}", count_part2);
}

fn test_slope(grid: &Vec<String>, x_step: usize, y_step: usize) -> usize {
    let w = grid[0].len();
    let h = grid.len();

    let mut x = 0;
    let mut y = 0;

    let mut count = 0;

    while y < h {
        let c = grid[y].as_bytes()[x] as char;
        if c == '#' {
            count += 1;
        }

        x += x_step;
        x = x % w;
        y += y_step;
    }

    return count;
}