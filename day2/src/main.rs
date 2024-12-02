use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let cnt = safe_report_count(lines);
    println!("safe reports: {}", cnt);

    let lines = read_lines("input");
    let cnt = safe_report_count_2(lines);
    println!("safe reports 2: {}", cnt);
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

fn line_to_levels(line: &str) -> Vec<u64> {
    line.split_ascii_whitespace()
        .map(|e| e.parse().unwrap())
        .collect::<Vec<_>>()
}

fn safe_report_count(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(|l| line_to_levels(&l))
        .filter(|levels| level_safe(levels.iter().copied()))
        .count()
}

fn safe_report_count_2(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(|l| line_to_levels(&l))
        .filter(|levels| level_safe_2(levels))
        .count()
}

fn level_safe(levels: impl Iterator<Item = u64>) -> bool {
    let mut levels = levels.peekable();
    let mut sign = None;
    while let Some(level) = levels.next() {
        if let Some(next_level) = levels.peek() {
            if level == *next_level {
                return false;
            }
            let diff = level as i64 - *next_level as i64;
            if sign == None {
                sign = Some(diff.signum());
            }
            let Some(sign) = sign else {
                panic!("sign not set");
            };
            let dir_diff = diff * sign;
            if dir_diff < 0 || dir_diff > 3 {
                return false;
            }
        }
    }

    true
}

fn level_safe_2(levels: &[u64]) -> bool {
    for skip_num in 0..levels.len() {
        let passed = || {
            let mut sign = None;
            for (i, level) in levels.iter().copied().enumerate() {
                if i == skip_num {
                    continue;
                }
                let mut skip = 0;
                if i + 1 == skip_num {
                    skip = 1;
                }
                if let Some(next_level) = levels.get(i + 1 + skip) {
                    if level == *next_level {
                        return false;
                    }
                    let diff = level as i64 - *next_level as i64;
                    if sign == None {
                        sign = Some(diff.signum());
                    }
                    let Some(sign) = sign else {
                        panic!("sign not set");
                    };
                    let dir_diff = diff * sign;
                    if dir_diff < 0 || dir_diff > 3 {
                        return false;
                    }
                }
            }
            true
        };
        if passed() {
            return true;
        }
    }

    false
}
