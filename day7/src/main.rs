use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let sum = result_sum_from_sat_peqs(lines);
    println!("Total calibration result: {}", sum);

    let lines = read_lines("input");
    let sum = result_sum_from_sat_peqs_w_cat(lines);
    println!("Total calibration result with cat: {}", sum);
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

fn lines_to_peqs(lines: impl Iterator<Item = String>) -> Vec<(u64, Vec<u64>)> {
    lines
        .map(|l| {
            let mut ra_iter = l.split(':');
            let result: u64 = ra_iter.next().unwrap().parse().unwrap();
            let args = ra_iter
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (result, args)
        })
        .collect()
}

fn sat<I>(result: u64, mut args: I, partial_result: u64) -> bool
where
    I: Iterator<Item = u64>,
    I: Clone,
{
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        return partial_result == result;
    };

    let add_result = partial_result + arg;
    let mul_result = partial_result * arg;

    // No zeros, so no operation can reduce the partial result
    if add_result <= result {
        if sat(result, args.clone(), add_result) {
            return true;
        }
    }

    if mul_result <= result {
        if sat(result, args, mul_result) {
            return true;
        }
    }

    false
}

fn sat_w_cat<I>(result: u64, mut args: I, partial_result: u64) -> bool
where
    I: Iterator<Item = u64>,
    I: Clone,
{
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        return partial_result == result;
    };

    let add_result = partial_result + arg;
    let mul_result = partial_result * arg;
    // Concatenate without any pesky string allocations
    let cat_result = (partial_result * 10_u64.pow(arg.ilog10() + 1)) + arg;

    // No zeros, so no operation can reduce the partial result
    if add_result <= result {
        if sat_w_cat(result, args.clone(), add_result) {
            return true;
        }
    }

    if mul_result <= result {
        if sat_w_cat(result, args.clone(), mul_result) {
            return true;
        }
    }

    if cat_result <= result {
        if sat_w_cat(result, args, cat_result) {
            return true;
        }
    }

    false
}

fn result_sum_from_sat_peqs(lines: impl Iterator<Item = String>) -> u64 {
    let peqs = lines_to_peqs(lines);
    peqs.iter()
        .filter(|(result, args)| sat(*result, args.iter().cloned(), 0))
        .map(|(result, _)| *result)
        .sum()
}

fn result_sum_from_sat_peqs_w_cat(lines: impl Iterator<Item = String>) -> u64 {
    let peqs = lines_to_peqs(lines);
    peqs.iter()
        .filter(|(result, args)| sat_w_cat(*result, args.iter().cloned(), 0))
        .map(|(result, _)| *result)
        .sum()
}
