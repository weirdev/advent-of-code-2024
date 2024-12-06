use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    ops::Div,
    path::Path,
};

fn main() {
    let lines = read_lines_incl_empty("input");
    let cnt = process_pages(lines);
    println!("Middle page sum: {}", cnt);

    let lines = read_lines_incl_empty("input");
    let cnt = process_pages_2(lines);
    println!("Fix middle page sum: {}", cnt);
}

fn read_lines_incl_empty<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap())
}

fn separate_ordering_and_page_lines(
    lines: impl Iterator<Item = String>,
) -> (Vec<String>, Vec<String>) {
    let lines: Vec<String> = lines.collect();
    let mut ord_and_pg = lines.split(|l| l.is_empty()).take(2);

    (
        ord_and_pg.next().unwrap().to_vec(),
        ord_and_pg.next().unwrap().to_vec(),
    )
}

fn ordering_lines_to_after_map(lines: impl Iterator<Item = String>) -> HashMap<u64, HashSet<u64>> {
    let mut ordering: HashMap<u64, HashSet<u64>> = HashMap::new();
    for line in lines {
        let mut l_r = line.split('|');
        let left = l_r.next().unwrap().parse::<u64>().unwrap();
        let right = l_r.next().unwrap().parse::<u64>().unwrap();

        if let Some(existing_afters) = ordering.get_mut(&left) {
            existing_afters.insert(right);
        } else {
            let mut set = HashSet::new();
            set.insert(right);
            ordering.insert(left, set);
        }
    }

    ordering
}

fn pages_line_to_vec(pages_line: String) -> Vec<u64> {
    pages_line.split(',').map(|s| s.parse().unwrap()).collect()
}

fn check_page_ordering(page_vec: &Vec<u64>, ordering: &HashMap<u64, HashSet<u64>>) -> bool {
    let mut previous_pages = HashSet::new();
    for page in page_vec {
        if let Some(afters) = ordering.get(page) {
            if afters.intersection(&previous_pages).next().is_some() {
                return false;
            }
        }
        previous_pages.insert(*page);
    }

    true
}

fn fix_page_ordering(
    mut page_vec: Vec<u64>,
    ordering: &HashMap<u64, HashSet<u64>>,
) -> Option<Vec<u64>> {
    let mut updated = true;
    let mut ever_updated = false;
    'outer: while updated {
        updated = false;
        let mut previous_pages = HashMap::new();
        for i in 0..page_vec.len() {
            let page = page_vec[i];
            if let Some(afters) = ordering.get(&page) {
                for after in afters {
                    if let Some(prev_page_poss) = previous_pages.get(after) {
                        for &prev_page_pos in prev_page_poss {
                            updated = true;
                            ever_updated = true;
                            page_vec.swap(i, prev_page_pos);
                            continue 'outer;
                        }
                    }
                }
            }
            previous_pages.insert(page, vec![i]);
        }
    }

    if ever_updated { Some(page_vec) } else { None }
}

fn process_pages(lines_incl_empty: impl Iterator<Item = String>) -> u64 {
    let (ordering_lines, pages_lines) = separate_ordering_and_page_lines(lines_incl_empty);
    let ordering = ordering_lines_to_after_map(ordering_lines.into_iter());

    let pages_lists = pages_lines.into_iter().map(pages_line_to_vec);

    pages_lists
        .filter(|pages| check_page_ordering(pages, &ordering))
        .map(|pages| pages[pages.len().div(2)])
        .sum()
}

fn process_pages_2(lines_incl_empty: impl Iterator<Item = String>) -> u64 {
    let (ordering_lines, pages_lines) = separate_ordering_and_page_lines(lines_incl_empty);
    let ordering = ordering_lines_to_after_map(ordering_lines.into_iter());

    let pages_lists = pages_lines.into_iter().map(pages_line_to_vec);

    pages_lists
        .filter_map(|pages| fix_page_ordering(pages, &ordering))
        .map(|pages| pages[pages.len().div(2)])
        .sum()
}
