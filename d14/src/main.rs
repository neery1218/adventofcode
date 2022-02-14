use std::{char, collections::HashMap};

use itertools::*;

fn main() {
    let (template, insertion_rules): (HashMap<(char, char), i64>, HashMap<(char, char), char>) =
        include_str!("input.txt").lines().enumerate().fold(
            (HashMap::new(), HashMap::new()),
            |(mut template, mut rules), (i, line)| {
                match i {
                    0 => {
                        template = line
                            .chars()
                            .into_iter()
                            .tuple_windows()
                            .chain([(line.chars().last().unwrap(), ' ')].into_iter())
                            .fold(HashMap::new(), |mut acc, (c1, c2)| {
                                acc.entry((c1, c2)).and_modify(|e| *e += 1).or_insert(1);
                                acc
                            })
                    }
                    1 => {} // blank line
                    _ => {
                        let toks: (&str, &str, &str) =
                            line.split_whitespace().collect_tuple().unwrap();
                        let left = toks.0.chars().collect_vec();
                        let right = toks.2.chars().collect_vec();
                        rules.insert((left[0], left[1]), right[0]);
                    }
                };

                (template, rules)
            },
        );

    // abcd => ab, bc, cd
    let process = (0..40).fold(template, |template, _| {
        template
            .into_iter()
            .fold(HashMap::new(), |mut acc, ((c1, c2), val)| {
                match insertion_rules.get(&(c1, c2)) {
                    None => {
                        acc.entry((c1, c2)).and_modify(|e| *e += val).or_insert(val);
                    }
                    Some(inter_val) => {
                        *acc.entry((c1, *inter_val)).or_insert(0) += val;
                        *acc.entry((*inter_val, c2)).or_insert(0) += val;
                    }
                };
                acc
            })
    });

    let mut freq: Vec<(char, i64)> = process
        .into_iter()
        .fold(HashMap::new(), |mut acc, ((c1, _), val)| {
            *acc.entry(c1).or_insert(0) += val;
            acc
        })
        .into_iter()
        .collect_vec();

    freq.sort_unstable_by_key(|k| k.1);
    println!("{}", freq.last().unwrap().1 - freq.first().unwrap().1);
}
