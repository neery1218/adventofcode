use itertools::Itertools;

fn main() {
    let final_pos: (i32, i32) = include_str!("input.txt")
        .split_whitespace()
        .tuples()
        .map(|(dir, mag)| {
            let v = mag.parse().unwrap();
            match dir {
                "forward" => (0, v),
                "backward" => (0, -1 * v),
                "up" => (-1 * v, 0),
                "down" => (v, 0),
                _ => panic!("Missed case: {}", dir),
            }
        })
        .fold((0, 0), |acc, ele| (acc.0 + ele.0, acc.1 + ele.1));

    println!("{}", final_pos.0 * final_pos.1);

    let with_aim: (i32, i32, i32) = include_str!("input.txt")
        .split_whitespace()
        .tuples()
        .fold((0, 0, 0), |(acc_v, acc_h, acc_a), (dir, _mag)| {
            let mag: i32 = _mag.parse().unwrap();
            match dir {
                "down" => (acc_v, acc_h, acc_a + mag),
                "up" => (acc_v, acc_h, acc_a - mag),
                "forward" => (acc_v + acc_a * mag, acc_h + mag, acc_a),
                _ => panic!("Missed case: {}", dir),
            }
        });

    println!("{}", with_aim.0 * with_aim.1);
}
