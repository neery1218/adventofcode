use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read failed!");
    let depths: Vec<u32> = contents
        .split_whitespace()
        .map(|token| token.parse().expect("Couldn't parse token!"))
        .collect();

    let mut num_increases = 0;
    let mut prev_depth = depths[0];
    for depth in depths.iter().skip(1) {
        if *depth > prev_depth {
            num_increases += 1;
        }
        prev_depth = *depth;
    }
    println!("Num increases: {}", num_increases);
}
