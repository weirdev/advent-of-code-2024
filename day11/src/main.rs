use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let line = read_line("input");
    let cnt = stone_count(line, 25);
    println!("Stone count 25x: {}", cnt);

    let line = read_line("input");
    let cnt = stone_count_w_collapse(line, 75);
    println!("Stone count 75x: {}", cnt);
}

fn read_line<P>(filename: P) -> Vec<u64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn child_stones(stone: u64) -> impl Iterator<Item = u64> {
    let mut output_arr = [None, None];
    if stone == 0 {
        output_arr[0] = Some(1);
    } else {
        let log = f64::log10(stone as f64) as u32;
        if log % 2 == 1 {
            let splitter = 10u64.pow((log + 1) / 2);
            let left = stone / splitter;
            let right = stone % splitter;

            output_arr[0] = Some(left);
            output_arr[1] = Some(right);
        } else {
            output_arr[0] = Some(stone * 2024);
        }
    }

    output_arr.into_iter().filter_map(|x| x)
}

fn levels<I>(stones: I, n: usize) -> impl Iterator<Item = u64>
where
    I: Iterator<Item = u64> + 'static,
{
    let mut stones: Box<dyn Iterator<Item = u64>> = Box::new(stones);
    for _ in 0..n {
        stones = Box::new(stones.flat_map(child_stones));
    }

    stones
}

fn stone_count<I>(stones: I, n: usize) -> usize
where
    I: IntoIterator<Item = u64> + 'static,
{
    levels(stones.into_iter(), n).count()
}

fn levels_w_collapse<I>(stones: I, n: usize) -> impl Iterator<Item = u64>
where
    I: Iterator<Item = u64> + 'static,
{
    let mut stones_w_count = stones.map(|s| (s, 1)).collect::<HashMap<_, _>>();
    for _ in 0..n {
        let mut new_stones = HashMap::new();
        for (s, c) in stones_w_count
            .iter()
            .flat_map(|(s, c)| child_stones(*s).into_iter().map(|cs| (cs, *c)))
        {
            let count = new_stones.entry(s).or_insert(0);
            *count += c;
        }
        stones_w_count = new_stones;
    }

    stones_w_count.into_iter().map(|(_, v)| v)
}

fn stone_count_w_collapse<I>(stones: I, n: usize) -> u64
where
    I: IntoIterator<Item = u64> + 'static,
{
    levels_w_collapse(stones.into_iter(), n).sum()
}
