use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::LazyLock,
};

use regex::Regex;

fn main() {
    let lines = read_lines("input");
    let cnt = sum_of_muls(lines);
    println!("Mul sum: {}", cnt);

    let lines = read_lines("input");
    let cnt = sum_of_muls_w_enables(lines);
    println!("Mul sum w/enables: {}", cnt);
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

fn line_to_muls(line: &str) -> Vec<(usize, usize)> {
    const RE: LazyLock<Regex> =
        std::sync::LazyLock::new(|| Regex::new(r"(mul\((\d\d?\d?),(\d\d?\d?)\))").unwrap());
    RE.captures_iter(line)
        .map(|cap| {
            let a = cap.get(2).unwrap().as_str().parse().unwrap();
            let b = cap.get(3).unwrap().as_str().parse().unwrap();
            (a, b)
        })
        .collect()
}

enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

fn line_to_muls_w_enables(line: &str) -> Vec<Instruction> {
    static RE: LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r"(mul\((\d\d?\d?),(\d\d?\d?)\)|do\(\)|don't\(\))").unwrap()
    });
    RE.captures_iter(line)
        .map(|cap| {
            if cap.get(1).unwrap().as_str() == "do()" {
                println!("do");
                Instruction::Do
            } else if cap.get(1).unwrap().as_str() == "don't()" {
                println!("don't");
                Instruction::Dont
            } else {
                let a = cap.get(2).unwrap().as_str().parse().unwrap();
                let b = cap.get(3).unwrap().as_str().parse().unwrap();
                Instruction::Mul(a, b)
            }
        })
        .collect()
}

fn sum_of_muls(lines: impl Iterator<Item = String>) -> usize {
    lines
        .flat_map(|l| line_to_muls(&l).into_iter())
        .map(|(a, b)| a * b)
        .sum()
}

fn sum_of_muls_w_enables(lines: impl Iterator<Item = String>) -> usize {
    lines
        .flat_map(|l| line_to_muls_w_enables(&l))
        .fold((true, 0), |(enabled, sum), instruction| match instruction {
            Instruction::Mul(a, b) => {
                if enabled {
                    (true, sum + (a * b))
                } else {
                    (false, sum)
                }
            }
            Instruction::Do => (true, sum),
            Instruction::Dont => (false, sum),
        })
        .1
}
