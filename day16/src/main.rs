use std::{
    cmp::{Ordering, Reverse}, fmt::Binary, fs::File, io::{self, BufRead}, iter, path::Path
};

const BOX: char = 'O';

fn main() {
    let inputs = read_inputs("input");
    let sum = gps_sum(inputs);
    println!("GPS Coord Sum: {}", sum);

    let inputs = read_inputs("input");
    let sum = gps_sum_wide(inputs);
    println!("GPS Coord Sum (Wide): {}", sum);
}

fn read_inputs<P>(
    filename: P,
) -> (
    impl IntoIterator<Item = String>,
    impl IntoIterator<Item = String>,
)
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let mut r_m = lines
        .split(|l| l.is_empty())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned());

    (r_m.next().unwrap(), r_m.next().unwrap())
}

fn lines_to_grid(lines: impl IntoIterator<Item = String>) -> Vec<Vec<char>> {
    lines.into_iter().map(|l| l.chars().collect()).collect()
}

#[derive(Clone, Copy)]
enum DIR {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn dir_to_offset(dir: DIR) -> (isize, isize) {
    match dir {
        DIR::UP => (0, -1),
        DIR::DOWN => (0, 1),
        DIR::LEFT => (-1, 0),
        DIR::RIGHT => (1, 0),
    }
}

fn dir_to_index(dir: DIR) -> usize {
    match dir {
        DIR::UP => 0,
        DIR::DOWN => 1,
        DIR::LEFT => 2,
        DIR::RIGHT => 3,
    }
}

fn rotation_options(dir: DIR) -> [DIR; 2] {
    match dir {
        DIR::UP => [DIR::LEFT, DIR::RIGHT],
        DIR::DOWN => [DIR::RIGHT, DIR::LEFT],
        DIR::LEFT => [DIR::DOWN, DIR::UP],
        DIR::RIGHT => [DIR::UP, DIR::DOWN],
    }
}

struct HeapNode((usize, usize, DIR), usize);

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1).map(Ordering::reverse)
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

fn dijkstra<R>(grid: &[R], start: (usize, usize, DIR), end: (usize, usize)) -> Result<usize, ()>
where
    R: AsRef<[char]>,
{
    let mut dist = vec![vec![[usize::MAX; 4]; grid[0].as_ref().len()]; grid.len()];
    let mut visited = vec![vec![false; grid[0].as_ref().len()]; grid.len()];

    // Heap
    let mut queue = std::collections::BinaryHeap::new();
    queue.push(HeapNode(start, 0));
    while let Some(HeapNode((x, y, dir), d)) = queue.pop() {
        if visited[y][x] {
            continue;
        }

        if (x, y) == end {
            return Ok(d);
        }

        visited[y][x] = true;
        dist[y][x][dir_to_index(dir)] = d;

        for (dir, cost) in
            iter::once((dir, 1)).chain(rotation_options(dir).into_iter().map(|d| (d, 1000)))
        {
            let (dx, dy) = dir_to_offset(dir);
            let (nx, ny) = (x as isize + dx, y as isize + dy);

            if nx < 0
                || ny < 0
                || ny >= grid.len() as isize
                || nx >= grid[ny as usize].as_ref().len() as isize
            {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
            if grid[ny].as_ref()[nx] == '#' {
                continue;
            }

            if dist[ny][nx][dir_to_index(dir)] > d + cost {
                queue.push(HeapNode((nx, ny, dir), d + cost));
            }
        }
    }

    // No path found
    Err(())
}
