use std::collections::BTreeMap;
use std::collections::BTreeSet;

const NEIGHBORS: [(isize, isize); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_bounds(img: &BTreeMap<(isize, isize), char>) -> (isize, isize, isize, isize) {
    let min_r = *img.keys().map(|(r, _)| r).min().unwrap();
    let max_r = *img.keys().map(|(r, _)| r).max().unwrap();
    let min_c = *img.keys().map(|(_, c)| c).min().unwrap();
    let max_c = *img.keys().map(|(_, c)| c).max().unwrap();
    (min_r, max_r, min_c, max_c)
}

fn print_img(img: &BTreeMap<(isize, isize), char>) {
    let (min_r, max_r, min_c, max_c) = get_bounds(img);
    println!();
    for r in (min_r - 5)..=(max_r + 5) {
        for c in (min_c - 5)..=(max_c + 5) {
            print!("{}", img.get(&(r, c)).or(Some(&'.')).unwrap());
        }
        println!();
    }
    println!();
}

fn main() {
    let (alg_str, img_str) = include_str!("input.txt").split_once("\n\n").unwrap();

    // keep indices that have '#'
    let alg: BTreeSet<usize> = alg_str
        .chars()
        .filter(|&c| c == '.' || c == '#')
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i)
        .collect();

    let img: BTreeMap<(isize, isize), char> = img_str
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.char_indices()
                .map(move |(c, val)| ((r as isize, c as isize), val))
        })
        .collect();

    let b = get_bounds(&img);
    let (final_img, bounds) = (0..50).fold((img, b), |(img, (min_r, max_r, min_c, max_c)), i| {
        println!("{}", i);
        let mut new_img = BTreeMap::new();
        for r in (min_r - 1)..=(max_r + 1) {
            for c in (min_c - 1)..=(max_c + 1) {
                let binary_str: String = NEIGHBORS
                    .map(|(dr, dc)| match img.get(&(r + dr, c + dc)) {
                        Some('#') => '1',
                        Some('.') => '0',
                        None if i % 2 == 1
                            && alg.contains(&0)
                            && (r + dr <= min_r - 1
                                || r + dr >= max_r + 1
                                || c + dc <= min_c - 1
                                || c + dc >= max_c + 1) =>
                        {
                            // println!("On Iteration {}, override ({}, {}) to '1'", i, r+dr, c+dc);
                            '1'
                        }
                        None => '0',
                        _ => unreachable!(),
                    })
                    .iter()
                    .collect();
                let v = alg.contains(&usize::from_str_radix(&binary_str, 2).unwrap());
                new_img.insert((r, c), if v { '#' } else { '.' });
            }
        }

        (new_img, (min_r - 1, max_r + 1, min_c - 1, max_c + 1))
    });

    let mut num_lit = 0;
    for r in bounds.0..=bounds.1 {
        for c in bounds.2..=bounds.3 {
            if final_img.get(&(r, c)) == Some(&'#') {
                num_lit += 1;
            }
        }
    }
    println!("{}", num_lit);
    println!("{:?}", alg);
}
