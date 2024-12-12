use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let cnt = all_trails_count(lines);
    println!("Trails: {}", cnt);

    let lines = read_lines("input");
    let cnt = trailhead_rating_sum(lines);
    println!("Trailhead rating sum: {}", cnt);
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

fn lines_to_grid(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn adj_grid_pos<R>(x: usize, y: usize, grid: &[R]) -> impl Iterator<Item = (usize, usize)>
where
    R: AsRef<[u8]>,
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

fn nines_from_zero<R>(x: usize, y: usize, grid: &[R]) -> usize
where
    R: AsRef<[u8]>,
{
    let mut visited = HashSet::new();
    let mut nines = HashSet::new();
    let mut to_visit = vec![((x, y), 0)];

    while to_visit.len() > 0 {
        let ((x, y), e) = to_visit.pop().unwrap();

        if grid[y].as_ref()[x] != e {
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        if grid[y].as_ref()[x] == 9 {
            nines.insert((x, y));
            continue;
        }

        let ne = e + 1;
        for (nx, ny) in adj_grid_pos(x, y, grid) {
            to_visit.push(((nx, ny), ne));
        }
    }

    nines.len()
}

fn path_count_from_zero<R>(x: usize, y: usize, grid: &[R]) -> usize
where
    R: AsRef<[u8]>,
{
    let mut nines = 0;
    let mut to_visit = vec![((x, y), 0)];

    while to_visit.len() > 0 {
        let ((x, y), e) = to_visit.pop().unwrap();

        if grid[y].as_ref()[x] != e {
            continue;
        }

        if grid[y].as_ref()[x] == 9 {
            nines += 1;
            continue;
        }

        let ne = e + 1;
        for (nx, ny) in adj_grid_pos(x, y, grid) {
            to_visit.push(((nx, ny), ne));
        }
    }

    nines
}

fn all_trails_count(lines: impl Iterator<Item = String>) -> usize {
    let grid = lines_to_grid(lines);

    (0..grid.len())
        .flat_map(|y| (0..grid[y].len()).map(move |x| (x, y)))
        .map(|(x, y)| nines_from_zero(x, y, &grid))
        .sum()
}

fn trailhead_rating_sum(lines: impl Iterator<Item = String>) -> usize {
    let grid = lines_to_grid(lines);

    (0..grid.len())
        .flat_map(|y| (0..grid[y].len()).map(move |x| (x, y)))
        .map(|(x, y)| path_count_from_zero(x, y, &grid))
        .sum()
}
