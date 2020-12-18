use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, BTreeSet};
use std::{fmt, cmp};
use std::fmt::Write;


fn main() {
    let file = File::open("input/day17.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut grid = InfiniteCubeGrid::new();

    for (y, line) in buf_reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            grid.set(&(x as i64, y as i64, 0, 0), c == '#');
        }
    }

    for step in 0..6 {
        let mut next_grid = InfiniteCubeGrid::new();

        let ((x1, y1, z1, w1), (x2, y2, z2, w2)) = grid.bbox();
        for x in (x1 - 1)..=(x2 + 1) {
            for y in (y1 - 1)..=(y2 + 1) {
                for z in (z1 - 1)..=(z2 + 1) {
                    for w in (w1 - 1)..=(w2 + 1) {

                        // Count active neighbors
                        let mut active_neighbors = 0;
                        for dx in -1..=1 {
                            for dy in -1..=1 {
                                for dz in -1..=1 {
                                    for dw in -1..=1 {
                                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                            continue;
                                        }
                                        if grid.is_active(&(x + dx, y + dy, z + dz, w + dw)) {
                                            active_neighbors += 1;
                                        }
                                    }
                                }
                            }
                        }

                        // Update states in next grid
                        if grid.is_active(&(x, y, z, w)) {
                            next_grid.set(&(x, y, z, w), active_neighbors == 2 || active_neighbors == 3)
                        } else {
                            next_grid.set(&(x, y, z, w), active_neighbors == 3)
                        }
                    }
                }
            }
        }

        grid = next_grid;
    }

    println!("Part 1: {}", grid.count_active());
}

#[derive(Clone)]
struct InfiniteCubeGrid {
    active_cells: BTreeSet<(i64, i64, i64, i64)>,
    bbox_min: (i64, i64, i64, i64),
    bbox_max: (i64, i64, i64, i64),
}

impl InfiniteCubeGrid {
    fn new() -> Self {
        Self {
            active_cells: BTreeSet::new(),
            bbox_min: (0, 0, 0, 0),
            bbox_max: (0, 0, 0, 0),
        }
    }

    fn is_active(&self, pos: &(i64, i64, i64, i64)) -> bool {
        return self.active_cells.contains(pos);
    }

    fn set(&mut self, pos: &(i64, i64, i64, i64), active: bool) {
        if active {
            self.active_cells.insert(*pos);

            self.bbox_min = (
                cmp::min(self.bbox_min.0, pos.0),
                cmp::min(self.bbox_min.1, pos.1),
                cmp::min(self.bbox_min.2, pos.2),
                cmp::min(self.bbox_min.3, pos.3),
            );

            self.bbox_max = (
                cmp::max(self.bbox_max.0, pos.0),
                cmp::max(self.bbox_max.1, pos.1),
                cmp::max(self.bbox_max.2, pos.2),
                cmp::max(self.bbox_max.3, pos.3),
            );
        } else {
            self.active_cells.remove(pos);
        }
    }

    fn bbox(&self) -> ((i64, i64, i64, i64), (i64, i64, i64, i64)) {
        (self.bbox_min, self.bbox_max)
    }

    fn count_active(&self) -> usize {
        self.active_cells.len()
    }
}

// impl fmt::Debug for InfiniteCubeGrid {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let ((x1, y1, z1), (x2, y2, z2)) = self.bbox();
//         for z in z1..=z2 {
//             writeln!(f, "z={}", z);
//             for y in y1..=y2 {
//                 for x in x1..=x2 {
//                     match self.is_active(&(x, y, z)) {
//                         true => f.write_char('#'),
//                         false => f.write_char('.'),
//                     };
//                 }
//                 writeln!(f, "");
//             }
//         }
//
//         Ok(())
//     }
// }