use itertools::*;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    On,
    Off,
}

#[derive(Debug)]
struct Rebook {
    state: State,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

fn main() {
    let mut grid = [[[State::Off; 101]; 101]; 101];

    let cuboids: Vec<Rebook> = include_str!("input.txt")
        .lines()
        .map(|l| {
            let (state, coords) = l.split_once(' ').unwrap();
            let c = coords
                .split(',')
                .flat_map(|s| {
                    let toks = s.split_once('=').unwrap().1.split('.').collect_vec();
                    vec![toks[0].parse().unwrap(), toks[2].parse().unwrap()]
                })
                .collect_vec();

            Rebook {
                state: match state {
                    "on" => State::On,
                    "off" => State::Off,
                    _ => unreachable!("{}", state),
                },
                x: (c[0], c[1]),
                y: (c[2], c[3]),
                z: (c[4], c[5]),
            }
        })
        .collect();

    for c in &cuboids {
        for x in cmp::max(c.x.0, -50)..=cmp::min(c.x.1, 50) {
            for y in cmp::max(c.y.0, -50)..=cmp::min(c.y.1, 50) {
                for z in cmp::max(c.z.0, -50)..=cmp::min(c.z.1, 50) {
                    grid[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = c.state;
                }
            }
        }
    }

    let num_lit: usize = grid
        .iter()
        .map(|&plane| {
            let a: usize = plane
                .iter()
                .map(|&line| line.iter().filter(|&cell| *cell == State::On).count())
                .sum();
            a
        })
        .sum();
    println!("{:?} {}", cuboids, num_lit);
}
