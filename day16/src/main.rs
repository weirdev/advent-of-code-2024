use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead},
    iter::{self, repeat_with},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let score = lowest_path_score(lines);
    println!("Lowest path score: {}", score);

    let lines = read_lines("input");
    let count = tile_count_in_shortest_paths(lines);
    println!("Tile count in shortest paths: {}", count);
}

fn read_lines<P>(filename: P) -> impl IntoIterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap())
}

fn lines_to_grid(lines: impl IntoIterator<Item = String>) -> Vec<Vec<char>> {
    lines.into_iter().map(|l| l.chars().collect()).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

struct HeapNode((usize, usize, DIR), usize, (usize, usize, DIR));

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

fn shortest_path_len<R>(
    grid: &[R],
    start: (usize, usize, DIR),
    end: (usize, usize),
) -> Result<usize, ()>
where
    R: AsRef<[char]>,
{
    let (dist, _) = dijkstra_dists_and_paths(grid, start, end);

    if let Some(&d) = dist[end.1][end.0].iter().min() {
        if d != usize::MAX {
            return Ok(d);
        }
    }

    Err(())
}

fn dijkstra_dists_and_paths<R>(
    grid: &[R],
    start: (usize, usize, DIR),
    end: (usize, usize),
) -> (
    Vec<Vec<[usize; 4]>>,
    Vec<Vec<[Vec<(usize, usize, DIR)>; 4]>>,
)
where
    R: AsRef<[char]>,
{
    // Three dimensions (x, y, direction)
    let mut dist = vec![vec![[usize::MAX; 4]; grid[0].as_ref().len()]; grid.len()];
    let inner_array: [Vec<(usize, usize, DIR)>; 4] = repeat_with(|| Vec::new())
        .take(4)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut prev = vec![vec![inner_array; grid[0].as_ref().len()]; grid.len()];

    // Heap
    let mut queue = std::collections::BinaryHeap::new();
    queue.push(HeapNode(start, 0, start));
    while let Some(HeapNode((x, y, odir), d, prev_node)) = queue.pop() {
        let iodir = dir_to_index(odir);

        if d > dist[y][x][iodir] {
            continue;
        }
        // If d was less than the current distance, we would have already visited this node
        // So d == dist[y][x][iodir] or node is unvisited

        dist[y][x][iodir] = d;

        let visited = prev[y][x][iodir].len() > 0;

        prev[y][x][iodir].push(prev_node);

        // Match (end_x, end_y, any direction)
        if (x, y) == end || visited {
            continue;
        }

        for (dir, cost) in
            iter::once((odir, 1)).chain(rotation_options(odir).into_iter().map(|d| (d, 1000)))
        {
            let (nx, ny) = if odir == dir {
                // Continue in the same direction
                let (dx, dy) = dir_to_offset(dir);
                (x as isize + dx, y as isize + dy)
            } else {
                // Rotate, no translation
                (x as isize, y as isize)
            };

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
                queue.push(HeapNode((nx, ny, dir), d + cost, (x, y, odir)));
            }
        }
    }

    (dist, prev)
}

fn get_start_and_end<R>(grid: &[R]) -> ((usize, usize, DIR), (usize, usize))
where
    R: AsRef<[char]>,
{
    let mut start = None;
    let mut end = None;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.as_ref().iter().enumerate() {
            if c == 'S' {
                start = Some((x, y, DIR::RIGHT));
            } else if c == 'E' {
                end = Some((x, y));
            }
        }
    }

    (start.unwrap(), end.unwrap())
}

fn tiles_in_shortest_paths<R>(grid: &[R], start: (usize, usize, DIR), end: (usize, usize)) -> usize
where
    R: AsRef<[char]>,
{
    let (_, prev) = dijkstra_dists_and_paths(grid, start, end);

    let mut queue = [DIR::UP, DIR::DOWN, DIR::LEFT, DIR::RIGHT]
        .into_iter()
        .map(|d| (end.0, end.1, d))
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; grid[0].as_ref().len()]; grid.len()];

    while let Some((x, y, dir)) = queue.pop() {
        visited[y][x] = true;

        if (x, y, dir) == start {
            continue;
        }

        for &(px, py, pdir) in &prev[y][x][dir_to_index(dir)] {
            queue.push((px, py, pdir));
        }
    }

    visited.iter().flatten().filter(|&&v| v).count()
}

fn lowest_path_score(lines: impl IntoIterator<Item = String>) -> usize {
    let grid = lines_to_grid(lines);
    let (start, end) = get_start_and_end(&grid);

    shortest_path_len(&grid, start, end).unwrap()
}

fn tile_count_in_shortest_paths(lines: impl IntoIterator<Item = String>) -> usize {
    let grid = lines_to_grid(lines);
    let (start, end) = get_start_and_end(&grid);

    tiles_in_shortest_paths(&grid, start, end)
}
