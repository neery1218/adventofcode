use std::{char, collections::HashMap};

use itertools::*;

fn main() {
    let (template, insertion_rules): (&str, HashMap<(char, char), char>) =
        include_str!("test.txt").lines().enumerate().fold(
            ("", HashMap::new()),
            |(mut template, mut rules), (i, line)| {
                match i {
                    0 => {
                        template = line;
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

    println!(
        "{} {} {:?}",
        template,
        insertion_rules.len(),
        insertion_rules
    );
    // abcd => ab, bc, cd
    let process = (0..10).fold(String::from(template), |template, i| {
        let iteration: String = template
            .chars()
            .tuple_windows()
            .chain([(template.chars().last().unwrap(), ' ')].into_iter())
            .flat_map(|(c1, c2)| match insertion_rules.get(&(c1, c2)) {
                None => vec![c1],
                Some(insert) => vec![c1, *insert],
            })
            .collect();

        println!("{}", i);
        iteration
    });

    let mut freq: Vec<(char, i32)> = process
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
            acc
        })
        .into_iter()
        .collect_vec();

    freq.sort_unstable_by_key(|k| k.1);
    println!("{:?}", freq);
    println!("{}", freq.last().unwrap().1 - freq.first().unwrap().1);
}
