use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let sum = dist_sum(lines);
    println!("dist sum: {}", sum);

    let lines = read_lines("input");
    let sum = similarity_sum(lines);
    println!("similarity sum: {}", sum);
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

fn line_to_locs(line: &str) -> (u64, u64) {
    let mut elems = line.split_ascii_whitespace();
    let left = elems.next().unwrap();
    let right = elems.next().unwrap();

    let left: u64 = left.parse().unwrap();
    let right: u64 = right.parse().unwrap();

    (left, right)
}

fn dist_sum(lines: impl Iterator<Item = String>) -> u64 {
    let locs = lines.map(|l| line_to_locs(&l)).collect::<Vec<_>>();

    let mut left_locs = locs.iter().map(|(l, _)| *l).collect::<Vec<_>>();
    let mut right_locs = locs.iter().map(|(_, r)| *r).collect::<Vec<_>>();

    left_locs.sort();
    right_locs.sort();

    left_locs
        .into_iter()
        .zip(right_locs.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u64>()
}

fn similarity_sum(lines: impl Iterator<Item = String>) -> u64 {
    let locs = lines.map(|l| line_to_locs(&l)).collect::<Vec<_>>();

    let left_locs = locs.iter().map(|(l, _)| *l).collect::<Vec<_>>();
    let right_locs_freq = locs
        .iter()
        .map(|(_, r)| *r)
        .fold(HashMap::new(), |mut acc, r| {
            acc.insert(r, acc.get(&r).unwrap_or(&0) + 1 as u64);
            acc
        });

    left_locs
        .into_iter()
        .map(|l| l * right_locs_freq.get(&l).unwrap_or(&0))
        .sum::<u64>()
}
