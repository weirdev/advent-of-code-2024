use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let line = read_line("input");
    let cs = get_checksum(line);
    println!("Checksum: {}", cs);

    let line = read_line("input");
    let cs = get_contig_checksum(line);
    println!("Contiguous checksum: {}", cs);
}

fn read_line<P>(filename: P) -> Vec<u8>
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
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>()
}

fn get_checksum<C>(mut chars: C) -> usize
where
    C: AsMut<[u8]>,
{
    let mut checksum = 0;
    let chars = chars.as_mut();
    let mut j = chars.len() - 1;
    j -= j % 2; // Start on a file
    let mut idx: usize = 0;
    for i in 0..chars.len() {
        if i % 2 == 0 {
            let ci = chars[i];
            for x in 0..ci {
                checksum += (i / 2) * (idx + x as usize);
            }
            idx += ci as usize;
        } else {
            while j > i + 1 && chars[j] > 0 && chars[i] > 0 {
                let filled = u8::min(chars[j], chars[i]);
                chars[i] -= filled;
                chars[j] -= filled;

                for x in 0..filled {
                    checksum += (j / 2) * (idx + x as usize);
                }
                idx += filled as usize;

                if chars[j] == 0 {
                    // Move to the next file, skip blanks when coming this direction
                    j -= 2;
                }
            }
        }

        if i == j {
            break;
        }
    }

    checksum
}

fn get_contig_checksum<C>(mut chars: C) -> usize
where
    C: AsMut<[u8]>,
{
    let mut checksum = 0;
    let chars = chars.as_mut();
    let mut j = (chars.len() - 1) as isize;
    j -= j % 2; // Start on a file

    let mut idxs = {
        let mut running_sum = 0;
        chars
            .iter()
            .map(|c| {
                let idx = running_sum;
                running_sum += *c as usize;
                idx
            })
            .collect::<Vec<_>>()
    };

    while j >= 0 {
        {
            let j = j as usize;
            let mut i = 1;
            while i < j {
                if chars[i] >= chars[j] {
                    chars[i] -= chars[j];

                    for x in 0..chars[j] {
                        checksum += (j / 2) * (idxs[i] + x as usize);
                    }

                    // "Merge" the file into the previous one (just for idx calculation)
                    idxs[i] += chars[j] as usize;

                    chars[j] = 0;

                    break;
                }

                i += 2;
            }

            if chars[j] > 0 {
                for x in 0..chars[j] {
                    checksum += (j / 2) * (idxs[j] + x as usize);
                }
            }
        }

        j -= 2;
    }

    checksum
}
