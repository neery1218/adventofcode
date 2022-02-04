const MAX: i32 = 1763;

fn main() {
    let mut pos: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|tok| tok.parse().unwrap())
        .collect();

    pos.sort_unstable();

    let median = pos[pos.len() / 2];
    let fuel: i32 = pos.iter().map(|p| (median - p).abs()).sum();
    println!("best pos: {} fuel: {}", median, fuel);

    let mut best_p = 0;
    let mut best_fuel = i32::MAX;

    // part 2
    for p in 0..MAX {
        let fuel: i32 = pos
            .iter()
            .map(|x| {
                let d = (p - x).abs();
                d * (d + 1) / 2
            })
            .sum();
        if fuel <= best_fuel {
            best_fuel = fuel;
            best_p = p;
        }
    }

    println!("best pos: {} with fuel: {}", best_p, best_fuel);
}
