use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, opt, value};
use nom::IResult;
use nom::multi::{count, separated_list0};
use nom::sequence::{delimited, separated_pair, terminated};

fn main() {
    let mut input = String::default();
    let mut file = File::open("input/day20.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let (_, tiles) = parse_input(&input).unwrap();

    let mut top_borders: HashMap<[bool; 10], HashSet<usize>> = HashMap::new();
    for (idx, tile) in &tiles {
        for tile_perm in tile.permutations().iter() {
            top_borders.entry(tile_perm.top_border()).or_default().insert(*idx);
        }
    }

    let mut idx_edge_count: HashMap<usize, usize> = HashMap::new();
    let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (_, idxes) in &top_borders {
        if idxes.len() == 1 {
            let idx = *idxes.iter().nth(0).unwrap();
            *idx_edge_count.entry(idx).or_default() += 1;
        }

        for idx in idxes {
            for adj_idx in idxes {
                if idx == adj_idx {
                    continue;
                }
                adjacency.entry(*idx).or_default().insert(*adj_idx);
            }
        }
    }

    let mut prod = 1;
    let mut first_corner_idx = 0;
    for (idx, count) in idx_edge_count {
        if count == 4 {
            prod *= idx;
            first_corner_idx = idx;
        }
    }
    println!("Part 1: {}", prod);

    let mut todo_tiles: HashSet<usize> = tiles.keys().cloned().collect();
    todo_tiles.remove(&first_corner_idx);

    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    grid.insert((0, 0), first_corner_idx);

    let size = 12;
    for y in 0..size {
        for x in 0..size {
            if y == 0 && x == 0 {
                continue;
            }
            let idx = if y == 0 || x == 0 {
                let adj_idx = if y == 0 { grid[&(x - 1, y)] } else { grid[&(x, y - 1)] };

                todo_tiles.iter().cloned().find(|idx| {
                    // println!("left_idx={} idx={} adj={}", adj_idx, idx, adjacency[idx].iter().join(", "));
                    adjacency[idx].len() <= 3 && adjacency[idx].contains(&adj_idx)
                }).unwrap()
            } else {
                let top_idx = grid[&(x, y - 1)];
                let left_idx = grid[&(x - 1, y)];

                todo_tiles.iter().cloned().find(|idx| {
                    // println!("left_idx={} idx={} adj={}", left_idx, idx, adjacency[idx].iter().join(", "));
                    adjacency[idx].contains(&top_idx) && adjacency[idx].contains(&left_idx)
                }).unwrap()
            };

            grid.insert((x, y), idx);
            todo_tiles.remove(&idx);
        }
    }

    // for y in 0..size {
    //     println!("{}", (0..size).map(|x| grid[&(x, y)]).join(" "));
    // }

    let first_corner = tiles[&first_corner_idx].permutations().iter().cloned()
        .find(|tile| {
            top_borders[&tile.top_border()].len() == 1 &&
                top_borders[&tile.left_border()].len() == 1 &&
                tiles[&grid[&(1, 0)]].permutations().iter().find(|right_tile| {
                    right_tile.left_border() == tile.right_border()
                }).is_some()
        })
        .unwrap();

    let mut tile_grid: HashMap<(usize, usize), Tile> = HashMap::new();
    tile_grid.insert((0,0), first_corner);

    for y in 0..size {
        for x in 0..size {
            if y == 0 && x == 0 {
                continue;
            }

            let tile_top = if y == 0 { None } else { tile_grid.get(&(x, y - 1)) };
            let tile_left = if x == 0 { None } else { tile_grid.get(&(x - 1, y)) };

            let tile = tiles[&grid[&(x, y)]].permutations().iter().cloned().find(|tile| {
                // println!("{} {}", x, y);
                tile_top.map_or(true, |tile_top| tile_top.bottom_border() == tile.top_border()) &&
                    tile_left.map_or(true, |tile_left| tile_left.right_border() == tile.left_border())
            }).unwrap().clone();
            tile_grid.insert((x, y), tile);
        }
    }

    let mut image: HashMap<(usize, usize), bool> = HashMap::new();
    for ((tx, ty), tile) in tile_grid {
        let tile_image = tile.without_borders();
        for x in 0..8 {
            for y in 0..8 {
                image.insert((tx * 8 + x, ty * 8 + y), tile_image[y][x]);
            }
        }
    }

    // for y in 0..(size * 8) {
    //     println!("{}", (0..(size * 8)).map(|x| if image[&(x, y)] { '#' } else { '.' } ).join(""))
    // }

    let mut monster_count = 0;

    for _ in 0..2 {
        for _ in 0..4 {
            // println!();
            // println!("------------------------");
            // println!();
            //
            // for y in 0..(size * 8) {
            //     println!("{}", (0..(size * 8)).map(|x| if image[&(x, y)] { '#' } else { '.' } ).join(""))
            // }

            // Check for monsters
            for x in 0..(size * 8) {
                for y in 0..(size * 8) {
                    if is_monster_at(&image, x, y) {
                        monster_count += 1;
                    }
                }
            }

            // Rotate
            let mut next_image: HashMap<(usize, usize), bool> = HashMap::new();
            for x in 0..(size * 8) {
                for y in 0..(size * 8) {
                    next_image.insert(((size * 8 - 1) - y, x), image[&(x, y)]);
                }
            }
            image = next_image;
        }

        // Flip
        let mut next_image: HashMap<(usize, usize), bool> = HashMap::new();
        for x in 0..(size * 8) {
            for y in 0..(size * 8) {
                next_image.insert(((size * 8 - 1) - x, y), image[&(x, y)]);
            }
        }
        image = next_image;
    }

    let part2 = image.values().filter(|v| **v).count() - monster_count * 15;
    println!("Part 2: {}", part2);
}

fn is_monster_at(image: &HashMap<(usize, usize), bool>, x: usize, y: usize) -> bool {
    let offsets: &[(usize, usize)] = &[
        (18, 0),
        (0, 1),
        (5, 1),
        (6, 1),
        (11, 1),
        (12, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (1, 2),
        (4, 2),
        (7, 2),
        (10, 2),
        (13, 2),
        (16, 2),
    ];

    offsets.iter().cloned().all(|(dx, dy)| {
        image.get(&(x + dx, y + dy)).map_or(false, |a| *a)
    })
}

#[derive(Clone)]
struct Tile {
    data: [[bool; 10]; 10],
}

impl Tile {
    fn flip_h(&self) -> Tile {
        let mut data = [[false; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                data[y][9 - x] = self.data[y][x];
            }
        }
        Tile { data }
    }
    fn rotate_cw(&self) -> Tile {
        let mut data = [[false; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                data[x][9 - y] = self.data[y][x];
            }
        }
        Tile { data }
    }
    fn permutations(&self) -> [Tile; 8] {
        [
            self.clone(),
            self.rotate_cw(),
            self.rotate_cw().rotate_cw(),
            self.rotate_cw().rotate_cw().rotate_cw(),
            self.flip_h(),
            self.flip_h().rotate_cw(),
            self.flip_h().rotate_cw().rotate_cw(),
            self.flip_h().rotate_cw().rotate_cw().rotate_cw(),
        ]
    }
    fn top_border(&self) -> [bool; 10] {
        self.data[0]
    }
    fn left_border(&self) -> [bool; 10] {
        self.rotate_cw().flip_h().top_border()
    }
    fn bottom_border(&self) -> [bool; 10] {
        self.rotate_cw().rotate_cw().flip_h().top_border()
    }
    fn right_border(&self) -> [bool; 10] {
        self.rotate_cw().rotate_cw().rotate_cw().top_border()
    }
    fn without_borders(&self) -> [[bool; 8]; 8] {
        let mut data = [[false; 8]; 8];
        for y in 0..8 {
            for x in 0..8 {
                data[y][x] = self.data[y + 1][x + 1];
            }
        }
        data
    }
}

fn parse_input(input: &str) -> IResult<&str, HashMap<usize, Tile>> {
    map(
        separated_list0(line_ending, parse_tile),
        |v| v.iter().cloned().collect(),
    )(input)
}

fn parse_tile(input: &str) -> IResult<&str, (usize, Tile)> {
    separated_pair(
        delimited(
            tag("Tile "),
            map(digit1, |s| usize::from_str(s).unwrap()),
            tag(":"),
        ),
        line_ending,
        map(parse_tile_grid, |data| Tile { data }),
    )(input)
}

fn parse_tile_grid(input: &str) -> IResult<&str, [[bool; 10]; 10]> {
    map(
        count(terminated(parse_tile_row, opt(line_ending)), 10),
        |v| vec_to_10_array(v),
    )(input)
}

fn parse_tile_row(input: &str) -> IResult<&str, [bool; 10]> {
    map(
        count(
            alt((
                value(false, tag(".")),
                value(true, tag("#")),
            )),
            10,
        ),
        |v| vec_to_10_array(v),
    )(input)
}

fn vec_to_10_array<V: Default + Copy>(vec: Vec<V>) -> [V; 10] {
    let mut arr = [V::default(); 10];
    for (place, element) in arr.iter_mut().zip(vec.iter()) {
        *place = *element;
    }
    arr
}