fn main() {
    let bitfields: Vec<Vec<u32>> = include_str!("input.txt")
        .split_whitespace()
        .map(|binary| binary.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let ones_per_column: Vec<u32> = bitfields
        .iter()
        .fold(vec![0; bitfields[0].len()], |acc, bitfield| {
            acc.iter()
                .zip(bitfield.iter())
                .map(|(a, b)| a + b)
                .collect()
        });

    let (_gamma_rate, _epsilon_rate): (Vec<char>, Vec<char>) = ones_per_column
        .iter()
        .map(|_num_ones| {
            let num_ones = *_num_ones as i32;
            if num_ones > (bitfields.len() as i32) - num_ones {
                ('1', '0')
            }
            else {
                ('0', '1')
            }
        }).unzip();

    let gamma_rate: i32 = i32::from_str_radix(&_gamma_rate.into_iter().collect::<String>(), 2).unwrap();
    let epsilon_rate: i32 = i32::from_str_radix(&_epsilon_rate.into_iter().collect::<String>(), 2).unwrap();

    println!("{} {}", gamma_rate, epsilon_rate);
    println!("{}", gamma_rate * epsilon_rate);
}
