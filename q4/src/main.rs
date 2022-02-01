const WIDTH: usize = 12;
const COUNT: usize = 1000;

fn main() {
    let freq = include_str!("input.txt")
        .split_whitespace()
        .map(|binary_str| u32::from_str_radix(binary_str, 2).unwrap())
        .fold(vec![0; WIDTH], |count, num| {
            count
                .iter()
                .enumerate()
                .map(|(i, count_i)| count_i + ((num >> i) & 0x1))
                .collect()
        })
        .into_iter()
        .map(|freq_i| if freq_i >= ((COUNT as u32) / 2) { '1' } else { '0' })
        .rev()
        .collect::<String>();

    let gamma_rate = u32::from_str_radix(&freq, 2).unwrap();
    let epsilon_rate = (!gamma_rate) & ((1 << WIDTH) - 1);
    println!("{} {} {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}
