use std::collections::HashMap;

// writing my own bc i think it's a good exercise.
fn permutations(v: String) -> Vec<String> {
    if v.len() == 1 {
        return vec![v];
    }

    let mut collector: Vec<String> = Vec::new();
    for sub_perm in permutations(String::from(&v[1..])) {
        for i in 0..(sub_perm.len() + 1) {
            let s = format!("{}{}{}", &sub_perm[0..i], &v[0..1], &sub_perm[i..]);
            collector.push(s);
        }
    }

    collector
}

fn main() {
    // part 1
    let entries: usize = include_str!("input.txt")
        .trim()
        .lines()
        .map(|s| {
            s.split('|')
                .last()
                .unwrap()
                .split_whitespace()
                .filter(|t| vec![2, 4, 3, 7].contains(&t.len()))
                .count()
        })
        .sum();
    println!("Part 1: {}", entries);

    // part 1
    let perms = permutations(String::from("abcdefg"));

    let entries: Vec<(Vec<&str>, Vec<&str>)> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|s| {
            let (patterns, value) = s.split_once('|').unwrap();
            (
                patterns.split_whitespace().collect::<Vec<&str>>(),
                value.split_whitespace().collect::<Vec<&str>>(),
            )
        })
        .collect();

    let mut sum = 0;
    for (patterns, values) in entries.iter() {
        for p in &perms {
            // create mapping
            let mut signal_to_segment = HashMap::new();
            for (signal, segment) in p.chars().into_iter().zip(String::from("abcdefg").chars()) {
                signal_to_segment.insert(signal, segment);
            }

            // check if mapping is valid. all after mapping pattern signals to segments, all
            // transformed patterns must be valid numbers
            if patterns
                .iter()
                .all(|p| map_pattern_to_segment(&signal_to_segment, p) != -1)
            {
                let num: i32 = values
                    .iter()
                    .map(|v| map_pattern_to_segment(&signal_to_segment, v) as i32)
                    .fold(0, |acc, d| acc * 10 + d);
                sum += num;
                break;
            }
        }
    }
    println!("Part 2: {}", sum);
}

fn map_pattern_to_segment(signal_to_segment: &HashMap<char, char>, pattern: &str) -> i8 {
    let mapped_pattern: String = pattern
        .chars()
        .map(|c| *signal_to_segment.get(&c).unwrap())
        .collect::<String>();

    let mut sorted: Vec<char> = mapped_pattern.chars().collect();
    sorted.sort_unstable();

    let segment: String = sorted.into_iter().collect();
    match segment.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => -1,
    }
}
