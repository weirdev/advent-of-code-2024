use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let cnt = visted_after_walk(lines);
    println!("Visited after walk: {}", cnt);

    // let lines = read_lines_incl_empty("input");
    // let cnt = process_pages_2(lines);
    // println!("Fix middle page sum: {}", cnt);
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

enum MapPos {
    Empty(
        bool, /* up */
        bool, /* down */
        bool, /* left */
        bool, /* right */
    ),
    Obstacle,
}

fn lines_to_grid(lines: impl Iterator<Item = String>) -> Vec<Vec<MapPos>> {
    lines
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => MapPos::Empty(false, false, false, false),
                    '#' => MapPos::Obstacle,
                    '^' => MapPos::Empty(true, false, false, false),
                    'v' => MapPos::Empty(false, true, false, false),
                    '<' => MapPos::Empty(false, false, true, false),
                    '>' => MapPos::Empty(false, false, false, true),
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect()
}

fn next_step(
    grid: &Vec<Vec<MapPos>>,
    x: usize,
    y: usize,
    dir: (isize, isize),
) -> Option<((usize, usize), (isize, isize))> {
    // Sanity check current position
    match grid[y][x] {
        MapPos::Empty(up, down, left, right) => match dir {
            (0, -1) => {
                if !up {
                    panic!("Current dir not set on grid");
                }
            }
            (0, 1) => {
                if !down {
                    panic!("Current dir not set on grid");
                }
            }
            (-1, 0) => {
                if !left {
                    panic!("Current dir not set on grid");
                }
            }
            (1, 0) => {
                if !right {
                    panic!("Current dir not set on grid");
                }
            }
            _ => panic!("Invalid direction"),
        },
        _ => panic!("Invalid start position"),
    };

    let x2 = x as isize + dir.0;
    let y2 = y as isize + dir.1;

    // Check if we are out of bounds
    if x2 < 0 || y2 < 0 || y2 >= grid.len() as isize || x2 >= grid[0].len() as isize {
        return None;
    }

    match grid[y2 as usize][x2 as usize] {
        MapPos::Empty(up, down, left, right) => {
            match dir {
                (0, -1) => {
                    if up {
                        panic!("Loop detected");
                    }
                }
                (0, 1) => {
                    if down {
                        panic!("Loop detected");
                    }
                }
                (-1, 0) => {
                    if left {
                        panic!("Loop detected");
                    }
                }
                (1, 0) => {
                    if right {
                        panic!("Loop detected");
                    }
                }
                _ => panic!("Invalid direction"),
            }

            Some(((x2 as usize, y2 as usize), dir))
        }
        MapPos::Obstacle => {
            let dir = match dir {
                (0, -1) => (1, 0),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (1, 0) => (0, 1),
                _ => panic!("Invalid direction"),
            };

            Some(((x, y), dir))
        }
    }
}

fn run_sim(grid: &mut Vec<Vec<MapPos>>) {
    let (mut start_pos, mut start_dir) = start_pos_and_dir(grid);

    while let Some((pos, dir)) = next_step(grid, start_pos.0, start_pos.1, start_dir) {
        match grid[pos.1][pos.0] {
            MapPos::Empty(ref mut up, ref mut down, ref mut left, ref mut right) => match dir {
                (0, -1) => {
                    *up = true;
                }
                (0, 1) => {
                    *down = true;
                }
                (-1, 0) => {
                    *left = true;
                }
                (1, 0) => {
                    *right = true;
                }
                _ => panic!("Invalid direction"),
            },
            _ => panic!("Invalid position"),
        }

        start_pos = pos;
        start_dir = dir;
    }
}

fn start_pos_and_dir(grid: &Vec<Vec<MapPos>>) -> ((usize, usize), (isize, isize)) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match grid[y][x] {
                MapPos::Empty(up, down, left, right) => {
                    if up {
                        return ((x, y), (0, -1));
                    }
                    if down {
                        return ((x, y), (0, 1));
                    }
                    if left {
                        return ((x, y), (-1, 0));
                    }
                    if right {
                        return ((x, y), (1, 0));
                    }
                }
                _ => {}
            }
        }
    }

    panic!("No start position found");
}

fn pos_visited(pos: &MapPos) -> bool {
    match *pos {
        MapPos::Empty(up, down, left, right) => up || down || left || right,
        _ => false,
    }
}

fn visited_pos_count(grid: &Vec<Vec<MapPos>>) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|p| pos_visited(p)).count())
        .sum()
}

fn visted_after_walk(lines: impl Iterator<Item = String>) -> usize {
    let mut grid = lines_to_grid(lines);
    run_sim(&mut grid);

    visited_pos_count(&grid)
}
