const WIDTH: usize = 12;
const COUNT: usize = 1000;
// const WIDTH: usize = 5;
// const COUNT: usize = 12;

fn main() {
    let binary_str = include_str!("input.txt")
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
        .map(|freq_i| {
            if freq_i >= ((COUNT as u32) / 2) {
                '1'
            } else {
                '0'
            }
        })
        .rev()
        .collect::<String>();

    let gamma_rate = u32::from_str_radix(&binary_str, 2).unwrap();
    let epsilon_rate = (!gamma_rate) & ((1 << WIDTH) - 1);
    println!(
        "part1: {} {} {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    // part 2
    let nums = include_str!("input.txt")
        .split_whitespace()
        .map(|binary_str| u32::from_str_radix(binary_str, 2).unwrap())
        .collect::<Vec<u32>>();

    let oxygen_rating = last_num_standing(nums.clone(), true);
    let co2_rating = last_num_standing(nums, false);
    println!("part 2: {} {} {}", oxygen_rating, co2_rating, oxygen_rating * co2_rating);
}

fn last_num_standing(mut arr: Vec<u32>, use_mcb: bool) -> u32 {
    let mut i = WIDTH - 1;
    while arr.len() > 1 {
        let count: u32 = arr.iter().map(|num| *num >> i & 0x1).sum();
        let mcb = if count * 2 >= (arr.len() as u32) {
            1
        } else {
            0
        };
        arr = arr
            .into_iter()
            .filter(|num| {
                if use_mcb {
                    (num >> i & 0x1) == mcb
                } else {
                    (num >> i & 0x1) != mcb
                }
            })
            .collect();

        if i == 0 {
            break;
        }

        i -= 1;
    }

    assert!(arr.len() == 1);
    return arr[0];
}
