fn main() {
    let depths: Vec<u32> = include_str!("input.txt")
        .split_whitespace()
        .map(|tok| tok.parse().unwrap())
        .collect();

    let mut prev_sliding_window_sum = depths[0] + depths[1] + depths[2];
    let mut sliding_window_sum = prev_sliding_window_sum;
    let mut num_increases = 0;

    for (i, val) in depths.iter().skip(3).enumerate() {
        sliding_window_sum = sliding_window_sum - depths[i] + val;
        if sliding_window_sum > prev_sliding_window_sum {
            num_increases += 1;
        }

        prev_sliding_window_sum = sliding_window_sum;
    }
    println!("{}", num_increases);
}
