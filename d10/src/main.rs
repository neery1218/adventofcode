use std::collections::HashMap;

fn main() {
    let matchings = HashMap::from([('}', '{'), (')', '('), ('>', '<'), (']', '[')]);

    // part 1
    let mut points: Vec<i64> = include_str!("input.txt")
        .lines()
        .map(|l| {
            let mut stack: Vec<char> = Vec::new();

            for bkt in l.chars() {
                match bkt {
                    '{' | '[' | '(' | '<' => stack.push(bkt),
                    right_bkt => match stack.last() {
                        None => return 0,
                        Some(c) if matchings.get(&right_bkt).unwrap() == c => {
                            stack.pop();
                        }
                        Some(c) if matchings.get(&right_bkt).unwrap() != c => {
                            return 0;
                        }
                        _ => unreachable!(),
                    },
                }
            }

            // create completion string
            let mut completion_str: String = String::new();
            let mut total_score = 0;
            while !stack.is_empty() {
                match stack.pop().unwrap() {
                    '{' => {
                        completion_str.push('}');
                        total_score = total_score * 5 + 3;
                    }
                    '(' => {
                        completion_str.push(')');
                        total_score = total_score * 5 + 1;
                    }
                    '<' => {
                        completion_str.push('>');
                        total_score = total_score * 5 + 4;
                    }
                    '[' => {
                        completion_str.push(']');
                        total_score = total_score * 5 + 2;
                    }
                    _ => unreachable!(),
                }
            }
            total_score
        })
        .filter(|s| *s != 0)
        .collect();

    points.sort_unstable();
    println!("Part 2 points: {:?}", points[points.len() / 2]);
}
