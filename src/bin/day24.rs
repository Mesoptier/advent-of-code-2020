use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::slice::Iter;
use std::time::Instant;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::IResult;
use nom::multi::{many1, separated_list0};

fn main() {
    let mut input = String::default();
    let mut file = File::open("input/day24.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let now = Instant::now();

    let (_, paths) = parse_input(&input).unwrap();
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    for path in paths {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for direction in path {
            let (nx, ny) = step_coord((x, y), direction);
            x = nx;
            y = ny;
        }

        if grid.contains(&(x, y)) {
            grid.remove(&(x, y));
        } else {
            grid.insert((x, y));
        }
    }

    println!("Part 1: {}", grid.len());

    for _day in 0..100 {
        // Collect relevant tiles
        let mut tiles = HashSet::new();
        for &tile in &grid {
            tiles.insert(tile);

            for &dir in Direction::iter() {
                tiles.insert(step_coord(tile, dir));
            }
        }

        let mut next_grid = HashSet::new();
        for tile in tiles {
            let mut black_neighbors = 0;
            for &dir in Direction::iter() {
                if grid.contains(&step_coord(tile, dir)) {
                    black_neighbors += 1;
                }
            }

            let is_black = grid.contains(&tile);

            let next_is_black = if is_black {
                !(black_neighbors == 0 || black_neighbors > 2)
            } else {
                black_neighbors == 2
            };

            if next_is_black {
                next_grid.insert(tile);
            } else {
                next_grid.remove(&tile);
            }
        }

        grid = next_grid;
    }

    println!("Part 2: {}", grid.len());

    println!("Time: {}ms", now.elapsed().as_millis());
}

fn step_coord((x, y): (i32, i32), dir: Direction) -> (i32, i32) {
    let is_odd_row = y.rem_euclid(2) == 1;

    let (dx, dy) = match dir {
        Direction::E => (1, 0),
        Direction::SE => if is_odd_row { (1, 1) } else { (0, 1) },
        Direction::SW => if is_odd_row { (0, 1) } else { (-1, 1) },
        Direction::W => (-1, 0),
        Direction::NW => if is_odd_row { (0, -1) } else { (-1, -1) },
        Direction::NE => if is_odd_row { (1, -1) } else { (0, -1) },
    };

    (x + dx, y + dy)
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 6] = [E, SE, SW, W, NW, NE];
        DIRECTIONS.iter()
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    separated_list0(
        line_ending,
        parse_path,
    )(input)
}

fn parse_path(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(input)
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::E, tag("e")),
        value(Direction::SE, tag("se")),
        value(Direction::SW, tag("sw")),
        value(Direction::W, tag("w")),
        value(Direction::NW, tag("nw")),
        value(Direction::NE, tag("ne")),
    ))(input)
}