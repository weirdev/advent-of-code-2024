use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let cnt = visted_after_walk(lines);
    println!("Visited after walk: {}", cnt);

    let lines = read_lines("input");
    let cnt = loop_causing_obstacle_positions_count(lines);
    println!("Loop causing positions count: {}", cnt);
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

#[derive(Clone)]
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

enum SimUpdate {
    Pos((usize, usize)),
    Dir((isize, isize)),
    Loop,
    End,
}

fn next_step(grid: &Vec<Vec<MapPos>>, x: usize, y: usize, dir: (isize, isize)) -> SimUpdate {
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
        return SimUpdate::End;
    }

    match grid[y2 as usize][x2 as usize] {
        MapPos::Empty(up, down, left, right) => {
            let in_loop = match dir {
                (0, -1) => up,
                (0, 1) => down,
                (-1, 0) => left,
                (1, 0) => right,
                _ => panic!("Invalid direction"),
            };

            if in_loop {
                SimUpdate::Loop
            } else {
                SimUpdate::Pos((x2 as usize, y2 as usize))
            }
        }
        MapPos::Obstacle => {
            let dir = match dir {
                (0, -1) => (1, 0),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (1, 0) => (0, 1),
                _ => panic!("Invalid direction"),
            };

            SimUpdate::Dir(dir)
        }
    }
}

fn run_sim(grid: &mut Vec<Vec<MapPos>>) -> bool {
    let (mut start_pos, mut start_dir) = start_pos_and_dir(grid);

    loop {
        match next_step(grid, start_pos.0, start_pos.1, start_dir) {
            SimUpdate::Pos(pos) => {
                start_pos = pos;
            }
            SimUpdate::Dir(dir) => {
                start_dir = dir;
            }
            SimUpdate::End => {
                return false;
            }
            SimUpdate::Loop => {
                return true;
            }
        }

        match grid[start_pos.1][start_pos.0] {
            MapPos::Empty(ref mut up, ref mut down, ref mut left, ref mut right) => match start_dir
            {
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

fn add_obstacle_and_check_for_loop(mut grid: Vec<Vec<MapPos>>, x: usize, y: usize) -> bool {
    match grid[y][x] {
        MapPos::Empty(up, down, left, right) => {
            // Guard location, can't add obstacle here
            if up || down || left || right {
                return false;
            }

            grid[y][x] = MapPos::Obstacle;

            run_sim(&mut grid)
        }
        // Already an obstacle
        _ => false,
    }
}

fn loop_causing_obstacle_positions_count(lines: impl Iterator<Item = String>) -> usize {
    let grid = lines_to_grid(lines);

    (0..grid.len())
        .map(|y| {
            (0..grid[0].len())
                .filter(|x| add_obstacle_and_check_for_loop(grid.clone(), *x, y))
                .count()
        })
        .sum()
}
