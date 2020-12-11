use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use std::cmp::{max, min};

fn main() {
    let file = File::open("input/day11.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = vec![];

    for line in buf_reader.lines() {
        let mut row: Vec<char> = vec![];
        for c in line.unwrap().chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let grid_copy = grid.clone();

    let h = grid.len();
    let w = grid[0].len();

    let mut changed = true;
    while changed {
        changed = false;

        let mut grid2 = grid.clone();

        for y in 0..h {
            for x in 0..w {
                let adj = count_occupied_adjacent(&grid, y, x);
                let c = grid[y][x];
                if c == 'L' && adj == 0 {
                    grid2[y][x] = '#';
                    changed = true;
                } else if c == '#' && adj >= 4 {
                    grid2[y][x] = 'L';
                    changed = true;
                }
            }
        }

        grid = grid2.clone();
    }

    let count_part1 = count_occupied(&grid);
    println!("Part 1: {}", count_part1);

    grid = grid_copy.clone();

    let mut changed = true;
    while changed {
        changed = false;

        let mut grid2 = grid.clone();

        for y in 0..h {
            for x in 0..w {
                let occ_vis = count_occupied_visible(&grid, y, x);
                let c = grid[y][x];
                if c == 'L' && occ_vis == 0 {
                    grid2[y][x] = '#';
                    changed = true;
                } else if c == '#' && occ_vis >= 5 {
                    grid2[y][x] = 'L';
                    changed = true;
                }
            }
        }

        grid = grid2.clone();
    }

    let count_part2 = count_occupied(&grid);
    println!("Part 2: {}", count_part2);
}

fn count_occupied(grid: &Vec<Vec<char>>) -> usize {
    let h = grid.len();
    let w = grid[0].len();

    let mut count = 0;

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '#' {
                count += 1;
            }
        }
    }

    return count;
}

fn count_occupied_adjacent(grid: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut count = 0;

    let h = grid.len();
    let w = grid[0].len();

    for yy in (if y == 0 { 0 } else { y - 1 })..=min(y + 1, h - 1) {
        for xx in (if x == 0 { 0 } else { x - 1 })..=min(x + 1, w - 1) {
            if yy == y && xx == x {
                continue;
            }

            if grid[yy][xx] == '#' {
                count += 1;
            }
        }
    }

    return count;
}

fn count_occupied_visible(grid: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut count = 0;

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    for dy in (-1)..=(1) {
        for dx in (-1)..=(1) {
            if dy == 0 && dx == 0 {
                continue;
            }

            let mut yy = y as i32;
            let mut xx = x as i32;

            while (0 <= yy + dy && yy + dy < h) && (0 <= xx + dx && xx + dx < w) {
                yy += dy;
                xx += dx;

                let c = grid[yy as usize][xx as usize];
                if c == '#' {
                    count += 1;
                    break;
                } else if c == 'L' {
                    break;
                }
            }
        }
    }

    return count;
}