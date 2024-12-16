use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const BOX: char = 'O';

fn main() {
    let inputs = read_inputs("input");
    let sum = gps_sum(inputs);
    println!("GPS Coord Sum: {}", sum);
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
    // Drop border from grid
    let mut v: Vec<_> = lines
        .into_iter()
        .skip(1)
        .map(|l| {
            let mut v: Vec<_> = l.chars().skip(1).collect();
            v.pop();
            v
        })
        .collect();
    v.pop();
    v
}

fn robot_pos<R>(grid: &[R]) -> (usize, usize)
where
    R: AsRef<[char]>,
{
    grid.iter()
        .enumerate()
        .find_map(|(y, r)| {
            r.as_ref()
                .iter()
                .copied()
                .enumerate()
                .find(|(_, c)| *c == '@')
                .map(|(x, _)| (x, y))
        })
        .unwrap()
}

fn robot_step<R>(grid: &mut [R], robot_pos: (usize, usize), dx: isize, dy: isize) -> (usize, usize)
where
    R: AsMut<[char]> + AsRef<[char]>,
{
    let (x, y) = robot_pos;
    let (nx, ny) = (x as isize + dx, y as isize + dy);

    let mut push_end_x = nx as isize;
    let mut push_end_y = ny as isize;
    while push_end_x >= 0
        && push_end_y >= 0
        && push_end_y < grid.len() as isize
        && push_end_x < grid[push_end_y as usize].as_ref().len() as isize
        && grid[push_end_y as usize].as_ref()[push_end_x as usize] == BOX
    {
        push_end_x += dx;
        push_end_y += dy;
    }

    if push_end_x < 0
        || push_end_y < 0
        || push_end_y >= grid.len() as isize
        || push_end_x >= grid[push_end_y as usize].as_ref().len() as isize
        || grid[push_end_y as usize].as_ref()[push_end_x as usize] != '.'
    {
        return (x, y);
    }

    let nx = nx as usize;
    let ny = ny as usize;

    grid[ny].as_mut()[nx] = '@';
    grid[y].as_mut()[x] = '.';

    if push_end_x != nx as isize || push_end_y != ny as isize {
        grid[push_end_y as usize].as_mut()[push_end_x as usize] = BOX;
    }

    (nx, ny)
}

fn dir(c: char) -> (isize, isize) {
    match c {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn execute_robot_run<R>(grid: &mut [R], direction_seq: impl Iterator<Item = char>)
where
    R: AsMut<[char]> + AsRef<[char]>,
{
    let mut robot_pos = robot_pos(grid);
    for d in direction_seq {
        robot_pos = robot_step(grid, robot_pos, dir(d).0, dir(d).1);
    }
}

fn lines_to_dirs(lines: impl IntoIterator<Item = String>) -> impl Iterator<Item = char> {
    lines
        .into_iter()
        .flat_map(|l| l.chars().collect::<Vec<_>>())
}

fn gps_sum(
    inputs: (
        impl IntoIterator<Item = String>,
        impl IntoIterator<Item = String>,
    ),
) -> usize {
    let (grid_lines, dir_lines) = inputs;
    let mut grid = lines_to_grid(grid_lines);
    let directions = lines_to_dirs(dir_lines);
    execute_robot_run(&mut grid, directions);

    grid.into_iter()
        .enumerate()
        .flat_map(|(y, r)| r.into_iter().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| *c == BOX)
        .map(|(x, y, _)| (x + 1) + ((y + 1) * 100))
        .sum()
}
