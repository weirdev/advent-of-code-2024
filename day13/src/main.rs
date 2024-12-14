#![feature(strict_overflow_ops)]

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    usize,
};

fn main() {
    let inputs = read_inputs("input");
    let cnt = token_count(inputs);
    println!("Token count: {}", cnt);

    let inputs = read_inputs("input");
    let cnt = token_count_shifted(inputs);
    println!("Token count shifted: {}", cnt);
}

fn read_inputs<P>(filename: P) -> impl Iterator<Item = Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    lines
        .split(|l| l.is_empty())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
        .into_iter()
}

#[derive(Debug)]
struct Spec {
    a: (usize, usize),
    b: (usize, usize),
    target: (usize, usize),
}

fn solve_spec(spec: &Spec) -> Option<usize> {
    if spec.a.1 * spec.b.0 == spec.a.0 * spec.b.1 {
        unreachable!("Linearly dependent shouldn't occur");
        // Linearly dependent

        // Of possible solutions, the one either the least or most number of A presses
        // is optimal
        let mut sol: Option<usize> = None;

        // Min B solution
        {
            let mut a_rem = 1;
            let mut b_apps = 0;
            while a_rem != 0 {
                let target = spec.target.0 - (b_apps * spec.b.0);

                let a_apps = target / spec.a.0;
                a_rem = spec.target.0 % spec.a.0;
                if a_rem == 0 {
                    let ns = 3 * a_apps + b_apps;
                    sol = Some(sol.map_or(ns, |s| s.min(ns)));
                    break;
                } else {
                    b_apps += 1;
                    if b_apps * spec.b.0 > spec.target.0 {
                        // No solution
                        return None;
                    }
                }
            }
        }

        // Min A solution
        let mut b_rem = 1;
        let mut a_apps = 0;
        while b_rem != 0 {
            let target = spec.target.0 - (a_apps * spec.a.0);

            let b_apps = target / spec.b.0;
            b_rem = spec.target.0 % spec.b.0;
            if b_rem == 0 {
                let ns = (3 * a_apps) + b_apps;
                return Some(sol.map_or(ns, |s| s.min(ns)));
            } else {
                a_apps += 1;
                if a_apps * spec.a.0 > spec.target.0 {
                    // Should have already detected no solution
                    assert!(b_apps * spec.b.0 < spec.target.0);
                }
            }
        }

        return None;
    }

    // Linearly independent
    let x_b_scaled = spec.b.0 * spec.a.1;
    let y_b_scaled = spec.b.1 * spec.a.0;
    let x_res_scaled = spec.target.0.strict_mul(spec.a.1);
    let y_res_scaled = spec.target.1.strict_mul(spec.a.0);

    let b_coeff = x_b_scaled as isize - y_b_scaled as isize;
    let res = isize::try_from(x_res_scaled).unwrap() - isize::try_from(y_res_scaled).unwrap();

    let b = (res / b_coeff) as usize;
    if res % b_coeff != 0 {
        // No solution
        return None;
    }
    let a_numerator = spec.target.0 - (b * spec.b.0);
    let a = a_numerator / spec.a.0;
    if a_numerator % spec.a.0 == 0 {
        Some((3 * a) + b)
    } else {
        None
    }
}

fn input_to_spec(mut input: impl Iterator<Item = String>) -> Spec {
    let a_button = input.next().unwrap();
    let b_button = input.next().unwrap();
    let result = input.next().unwrap();

    Spec {
        a: button_to_bspec(&a_button),
        b: button_to_bspec(&b_button),
        target: result_to_rspec(&result),
    }
}

fn button_to_bspec(button: &str) -> (usize, usize) {
    let button = button.split(": ").skip(1).next().unwrap();

    let mut button = button.split(", ");

    let x = button.next().unwrap();
    let y = button.next().unwrap();

    let x = x.split("+").skip(1).next().unwrap();
    let y = y.split("+").skip(1).next().unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

fn result_to_rspec(result: &str) -> (usize, usize) {
    let result = result.split(": ").skip(1).next().unwrap();

    let mut result = result.split(", ");

    let x = result.next().unwrap();
    let y = result.next().unwrap();

    let x = x.split("=").skip(1).next().unwrap();
    let y = y.split("=").skip(1).next().unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

fn token_count<T>(inputs: impl Iterator<Item = T>) -> usize
where
    T: IntoIterator<Item = String>,
{
    inputs
        .filter_map(|input| {
            let spec = input_to_spec(input.into_iter());
            solve_spec(&spec)
        })
        .sum()
}

fn token_count_shifted<T>(inputs: impl Iterator<Item = T>) -> usize
where
    T: IntoIterator<Item = String>,
{
    inputs
        .filter_map(|input| {
            let mut spec = input_to_spec(input.into_iter());
            spec.target.0 += 10000000000000;
            spec.target.1 += 10000000000000;
            solve_spec(&spec)
        })
        .sum()
}
