fn main() {
    let depths: Vec<u32> = include_str!("input.txt")
        .split_whitespace()
        .map(|tok| tok.parse().unwrap())
        .collect();

    let triple_sums: Vec<u32> = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();

    let num_increases = triple_sums
        .iter()
        .zip(triple_sums.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();

    println!("{}", num_increases);
}
