fn main() {
    let mut state: [u64; 9] = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|tok| tok.parse().unwrap())
        .fold([0; 9], |mut acc, time: u64| {
            acc[time as usize] += 1;
            acc
        });

    println!("{:?}", state);
    let num_days = 256;
    for _ in 0..num_days {
        let new_babies = state[0];
        for i in 1..9 {
            state[i - 1] = state[i];
        }
        state[6] += new_babies; // original parents have a 7 day timer
        state[8] = new_babies; // new babies have a 9 day timer
    }
    println!("{:?}", state);
    println!("{}", state.iter().sum::<u64>());
}
