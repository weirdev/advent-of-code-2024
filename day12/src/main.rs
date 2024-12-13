use std::{
    collections::{HashMap, hash_map::Entry},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let price = region_price_sum(lines);
    println!("Fence price: {}", price);

    let lines = read_lines("input");
    let price = region_price_sum_bulk(lines);
    println!("Fence price (bulk): {}", price);
}

fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
}

fn lines_to_grid(lines: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    lines
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

type RegionId = (usize, usize);

struct Region {
    perimeter: usize,
    area: usize,
}

fn grid_region_from_pos<R>(
    grid: &[R],
    region_mapping: &mut HashMap<(usize, usize), RegionId>,
    x: usize,
    y: usize,
) -> Option<Region>
where
    R: AsRef<[char]>,
{
    let region_lookup = region_mapping.entry((x, y));
    if let Entry::Vacant(v) = region_lookup {
        let region_id = (x, y);
        v.insert(region_id);
        let region_label = grid[y].as_ref()[x];
        let mut region = Region {
            perimeter: 0,
            area: 0,
        };

        let mut to_check = vec![(x, y)];
        while let Some((x, y)) = to_check.pop() {
            region.area += 1;
            if x == 0 || x == grid[0].as_ref().len() - 1 {
                region.perimeter += 1;
            }
            if y == 0 || y == grid.len() - 1 {
                region.perimeter += 1;
            }
            for (x, y) in adj_grid_pos(x, y, grid) {
                if grid[y].as_ref()[x] == region_label {
                    let region_lookup = region_mapping.entry((x, y));
                    if let Entry::Vacant(v) = region_lookup {
                        v.insert(region_id);
                        to_check.push((x, y));
                    }
                } else {
                    region.perimeter += 1;
                }
            }
        }

        Some(region)
    } else {
        None
    }
}

fn grid_region_from_pos_bulk<R>(
    grid: &[R],
    region_mapping: &mut HashMap<(usize, usize), RegionId>,
    x: usize,
    y: usize,
) -> Option<Region>
where
    R: AsRef<[char]>,
{
    let region_lookup = region_mapping.entry((x, y));
    if let Entry::Vacant(v) = region_lookup {
        let region_id = (x, y);
        v.insert(region_id);
        let region_label = grid[y].as_ref()[x];
        let mut region = Region {
            perimeter: 0,
            area: 0,
        };

        let mut to_check = vec![(x, y)];
        while let Some((x, y)) = to_check.pop() {
            region.area += 1;
            if x == 0 || x == grid[0].as_ref().len() - 1 {
                if y == 0 || grid[y - 1].as_ref()[x] != region_label {
                    // Not downward continuation edge
                    region.perimeter += 1;
                }
            }
            if y == 0 || y == grid.len() - 1 {
                if x == 0 || grid[y].as_ref()[x - 1] != region_label {
                    // Not rightward continuation edge
                    region.perimeter += 1;
                }
            }
            for (x2, y2) in adj_grid_pos(x, y, grid) {
                if grid[y2].as_ref()[x2] == region_label {
                    let region_lookup = region_mapping.entry((x2, y2));
                    if let Entry::Vacant(v) = region_lookup {
                        v.insert(region_id);
                        to_check.push((x2, y2));
                    }
                } else {
                    // Start of row?
                    let dx = x2 as isize - x as isize;
                    if dx != 0 {
                        // Vertical edge
                        if y == 0
                            || grid[y - 1].as_ref()[x] != region_label
                            || grid[y - 1].as_ref()[x2] == region_label
                        {
                            // Not downward continuation edge
                            region.perimeter += 1;
                        }
                    } else {
                        // dy != 0
                        // Horizontal edge
                        if x == 0
                            || grid[y].as_ref()[x - 1] != region_label
                            || grid[y2].as_ref()[x - 1] == region_label
                        {
                            // Not rightward continuation edge
                            region.perimeter += 1;
                        }
                    }
                }
            }
        }

        Some(region)
    } else {
        None
    }
}

fn adj_grid_pos<R>(x: usize, y: usize, grid: &[R]) -> impl Iterator<Item = (usize, usize)>
where
    R: AsRef<[char]>,
{
    let x = x as isize;
    let y = y as isize;
    let x_max = grid[0].as_ref().len() as isize;
    let y_max = grid.len() as isize;

    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(move |(dx, dy)| (x + dx, y + dy))
        .filter(move |(x, y)| *x >= 0 && *y >= 0 && *x < x_max && *y < y_max)
        .map(|(x, y)| (x as usize, y as usize))
}

fn region_price_sum(lines: impl Iterator<Item = String>) -> usize {
    let grid = lines_to_grid(lines);
    let mut region_mapping = HashMap::new();

    (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .filter_map(|x| grid_region_from_pos(&grid, &mut region_mapping, x, y))
                .map(|r| r.perimeter * r.area)
                .sum::<usize>()
        })
        .sum()
}

fn region_price_sum_bulk(lines: impl Iterator<Item = String>) -> usize {
    let grid = lines_to_grid(lines);
    let mut region_mapping = HashMap::new();

    (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .filter_map(|x| grid_region_from_pos_bulk(&grid, &mut region_mapping, x, y))
                .map(|r| r.perimeter * r.area)
                .sum::<usize>()
        })
        .sum()
}
