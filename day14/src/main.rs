use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let factor = safety_factor(lines, 100, 101, 103);
    println!("Safety factor: {}", factor);

    let lines = read_lines("input");
    let t = robot_repl(lines, 101, 103);
    println!("Time for tree: {}", t); // 6771
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

struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

fn line_to_robot(line: &str) -> Robot {
    let (pos, vel) = line.split_once(" ").unwrap();
    let (x, y) = pos.split_once("=").unwrap().1.split_once(",").unwrap();
    let (vx, vy) = vel.split_once("=").unwrap().1.split_once(",").unwrap();

    Robot {
        x: x.trim().parse().unwrap(),
        y: y.trim().parse().unwrap(),
        vx: vx.trim().parse().unwrap(),
        vy: vy.trim().parse().unwrap(),
    }
}

fn future_pos(robot: &Robot, t: usize, x_limit: usize, y_limit: usize) -> (usize, usize) {
    let dx = mod_mul(robot.vx, t as isize, x_limit as isize);
    let dy = mod_mul(robot.vy, t as isize, y_limit as isize);

    let mut x = mod_add(robot.x, dx, x_limit as isize);
    let mut y = mod_add(robot.y, dy, y_limit as isize);

    if x < 0 {
        x += x_limit as isize
    }
    if y < 0 {
        y += y_limit as isize
    }

    (x as usize, y as usize)
}

fn mod_mul(a: isize, b: isize, m: isize) -> isize {
    ((a % m) * (b % m)) % m
}

fn mod_add(a: isize, b: isize, m: isize) -> isize {
    ((a % m) + (b % m)) % m
}

fn pos_to_quadrants(x: usize, y: usize, x_limit: usize, y_limit: usize) -> Option<u8> {
    let x_mid = x_limit / 2;
    let x_quadrant = if x < x_mid {
        0b00
    } else if x > x_mid {
        0b01
    } else {
        return None;
    };
    let y_mid = y_limit / 2;
    let y_quadrant = if y < y_mid {
        0b00
    } else if y > y_mid {
        0b10
    } else {
        return None;
    };

    Some(x_quadrant | y_quadrant)
}

fn safety_factor(
    lines: impl Iterator<Item = String>,
    t: usize,
    x_limit: usize,
    y_limit: usize,
) -> usize {
    lines
        .map(|l| line_to_robot(&l))
        .map(|r| future_pos(&r, t, x_limit, y_limit))
        .filter_map(|(x, y)| pos_to_quadrants(x, y, x_limit, y_limit))
        .fold([0; 4], |mut acc, q| {
            acc[q as usize] += 1;
            acc
        })
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap()
}

fn step_robots(robots: &mut [Robot], t: usize, x_limit: usize, y_limit: usize) {
    for r in robots.iter_mut() {
        let (x, y) = future_pos(r, t, x_limit, y_limit);
        r.x = x as isize;
        r.y = y as isize;
    }
}

fn display_robots(robots: &[Robot], x_limit: usize, y_limit: usize) {
    let mut grid = vec![vec!['.'; x_limit]; y_limit];
    for r in robots {
        grid[r.y as usize][r.x as usize] = '#';
    }

    for row in grid {
        println!("{}", row.into_iter().collect::<String>());
    }
}

fn robot_repl(lines: impl Iterator<Item = String>, x_limit: usize, y_limit: usize) -> usize {
    let mut robots = lines.map(|l| line_to_robot(&l)).collect::<Vec<_>>();
    let mut t = 0;
    loop {
        display_robots(&robots, x_limit, y_limit);
        println!("t = {}\n", t);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "done" {
            break;
        }
        let t_step = input.trim().parse().unwrap();
        t += t_step;
        step_robots(&mut robots, t_step, x_limit, y_limit);
    }

    t
}
