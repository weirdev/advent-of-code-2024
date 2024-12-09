use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let cnt = antinode_count(lines);
    println!("Antinode count: {}", cnt);

    let lines = read_lines("input");
    let cnt = antinode_count2(lines);
    println!("Antinode count 2: {}", cnt);
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

fn lines_to_antenna_pos(
    lines: impl Iterator<Item = String>,
) -> ((usize, usize), HashMap<char, Vec<(usize, usize)>>) {
    lines
        .enumerate()
        .map(|(y, l)| {
            let chars = l.chars();
            let x = if y == 0 { chars.clone().count() } else { 0 };

            let ant_pos = l
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_alphanumeric())
                .map(move |(x, c)| (c, (x, y)))
                .collect::<Vec<_>>();

            (x, ant_pos)
        })
        .fold(
            ((0, 0), HashMap::new()),
            |((x_bound, y_bound), mut acc), (x, poss)| {
                poss.into_iter().for_each(|(c, pos)| {
                    acc.entry(c).or_insert_with(Vec::new).push(pos);
                });

                ((x_bound.max(x), y_bound + 1), acc)
            },
        )
}

fn antenna_poss_to_antinodes(
    poss: &Vec<(usize, usize)>,
    bounds: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    iter_2combs(poss.iter().copied())
        .flat_map(move |(pos1, pos2)| antenna_pair_to_antinodes(pos1, pos2, bounds))
}

fn antenna_poss_to_antinodes2(
    poss: &Vec<(usize, usize)>,
    bounds: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    iter_2combs(poss.iter().copied())
        .flat_map(move |(pos1, pos2)| antenna_pair_to_antinodes2(pos1, pos2, bounds))
}

fn antenna_pair_to_antinodes(
    pos1: (usize, usize),
    pos2: (usize, usize),
    bounds: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let dx = pos2.0 as isize - pos1.0 as isize;
    let dy = pos2.1 as isize - pos1.1 as isize;

    let first = (pos1.0 as isize + (2 * dx), pos1.1 as isize + (2 * dy));
    let second = (pos1.0 as isize - dx, pos1.1 as isize - dy);

    [first, second]
        .into_iter()
        .filter(move |(x, y)| {
            *x >= 0 && *y >= 0 && *x < bounds.0 as isize && *y < bounds.1 as isize
        })
        .map(|(x, y)| (x as usize, y as usize))
}

fn antenna_pair_to_antinodes2(
    pos1: (usize, usize),
    pos2: (usize, usize),
    bounds: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let dx = pos2.0 as isize - pos1.0 as isize;
    let dy = pos2.1 as isize - pos1.1 as isize;

    let inc = (0..)
        .map(move |i| (pos1.0 as isize + (i * dx), pos1.1 as isize + (i * dy)))
        .take_while(move |(x, y)| {
            *x >= 0 && *y >= 0 && *x < bounds.0 as isize && *y < bounds.1 as isize
        });

    let dec = (1..)
        .map(move |i| (pos1.0 as isize - (i * dx), pos1.1 as isize - (i * dy)))
        .take_while(move |(x, y)| {
            *x >= 0 && *y >= 0 && *x < bounds.0 as isize && *y < bounds.1 as isize
        });

    inc.chain(dec).map(|(x, y)| (x as usize, y as usize))
}

fn iter_2combs<I, E>(iter: I) -> impl Iterator<Item = (E, E)>
where
    I: Iterator<Item = E>,
    I: Clone,
    E: Clone,
{
    iter.clone().enumerate().flat_map(move |(i, e1)| {
        iter.clone()
            .enumerate()
            .filter(move |(j, _)| *j > i)
            .map(move |(_, e2)| (e1.clone(), e2))
    })
}

fn antinode_count(lines: impl Iterator<Item = String>) -> usize {
    let (bounds, ant_pos) = lines_to_antenna_pos(lines);
    let antinodes = ant_pos
        .values()
        .flat_map(|poss| antenna_poss_to_antinodes(poss, bounds))
        .collect::<HashSet<_>>();

    antinodes.len()
}

fn antinode_count2(lines: impl Iterator<Item = String>) -> usize {
    let (bounds, ant_pos) = lines_to_antenna_pos(lines);
    let antinodes = ant_pos
        .values()
        .flat_map(|poss| antenna_poss_to_antinodes2(poss, bounds))
        .collect::<HashSet<_>>();

    antinodes.len()
}
