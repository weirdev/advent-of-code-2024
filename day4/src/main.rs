use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let cnt = count_word_occurances(lines, "XMAS");
    println!("XMAS count: {}", cnt);

    let lines = read_lines("input");
    let cnt = count_x_occurances(lines, "MAS");
    println!("X-MAS count: {}", cnt);
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

fn lines_to_grid(lines: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    lines.map(|l| l.chars().collect()).collect()
}

fn word_start_at_position(grid: &Vec<Vec<char>>, word: &Vec<char>, x: usize, y: usize) -> usize {
    let mut count = 0;

    // Horizontal
    {
        let mut found = true;
        for i in 0..word.len() {
            if grid[y].len() <= x + i || grid[y][x + i] != word[i] {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
        }
    }

    // Horizontal backwards
    {
        let mut found = true;
        for i in 0..word.len() {
            if x < i || grid[y][x - i] != word[i] {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
        }
    }

    // Vertical
    {
        let mut found = true;
        for i in 0..word.len() {
            if grid.len() <= y + i || grid[y + i][x] != word[i] {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
        }
    }

    // Vertical backwards
    {
        let mut found = true;
        for i in 0..word.len() {
            if y < i || grid[y - i][x] != word[i] {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
        }
    }

    // Diagonal 315
    if diag_315(grid, word, x, y) {
        count += 1;
    }

    // Diagonal 45
    if diag_45(grid, word, x, y) {
        count += 1;
    }

    // Diagonal 135
    if diag_135(grid, word, x, y) {
        count += 1;
    }

    // Diagonal 225
    if diag_225(grid, word, x, y) {
        count += 1;
    }

    count
}

fn diag_315(grid: &Vec<Vec<char>>, word: &Vec<char>, x: usize, y: usize) -> bool {
    if y >= grid.len() || x >= grid[0].len() {
        return false;
    }

    for i in 0..word.len() {
        if grid.len() <= y + i || grid[y + i].len() <= x + i || grid[y + i][x + i] != word[i] {
            return false;
        }
    }
    true
}

fn diag_45(grid: &Vec<Vec<char>>, word: &Vec<char>, x: usize, y: usize) -> bool {
    if y >= grid.len() || x >= grid[0].len() {
        return false;
    }

    for i in 0..word.len() {
        if y < i || grid[y - i].len() <= x + i || grid[y - i][x + i] != word[i] {
            return false;
        }
    }
    true
}

fn diag_135(grid: &Vec<Vec<char>>, word: &Vec<char>, x: usize, y: usize) -> bool {
    if y >= grid.len() || x >= grid[0].len() {
        return false;
    }

    for i in 0..word.len() {
        if y < i || x < i || grid[y - i][x - i] != word[i] {
            return false;
        }
    }
    true
}

fn diag_225(grid: &Vec<Vec<char>>, word: &Vec<char>, x: usize, y: usize) -> bool {
    if y >= grid.len() || x >= grid[0].len() {
        return false;
    }

    for i in 0..word.len() {
        if grid.len() <= y + i || x < i || grid[y + i][x - i] != word[i] {
            return false;
        }
    }
    true
}

fn x_at_position(grid: &Vec<Vec<char>>, word: &Vec<char>, x: usize, y: usize) -> bool {
    // 00 ** ** 30
    // ** 11 21 **
    // ** 12 22 **
    // 03 ** ** 33

    // 315 * * 225
    //  *  * *  *
    //  *  * *  *
    //  45 * * 135

    (diag_315(grid, word, x, y) || diag_135(grid, word, x + word.len() - 1, y + word.len() - 1))
        && (diag_225(grid, word, x + word.len() - 1, y)
            || diag_45(grid, word, x, y + word.len() - 1))
}

fn count_word_occurances(lines: impl Iterator<Item = String>, word: &str) -> usize {
    let grid = lines_to_grid(lines);
    let word = word.chars().collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map({
                    let grid = &grid;
                    let word = &word;
                    move |(x, _)| word_start_at_position(grid, word, x, y)
                })
                .sum::<usize>()
        })
        .sum()
}

fn count_x_occurances(lines: impl Iterator<Item = String>, word: &str) -> usize {
    let grid = lines_to_grid(lines);
    let word = word.chars().collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter({
                    let grid = &grid;
                    let word = &word;
                    move |(x, _)| x_at_position(grid, word, *x, y)
                })
                .count()
        })
        .sum()
}
