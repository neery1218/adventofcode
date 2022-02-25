use itertools::*;
use std::{cmp, collections::BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cuboid {
    state: State,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Split {
    X(i64),
    Y(i64),
    Z(i64),
}

// break cube A into disjoint cubes that don't intersect cube B
fn break_first_cube(a: Cuboid, b: Cuboid) -> Vec<Cuboid> {
    // check if it intersects
    if a.x.0 > b.x.1
        || a.x.1 < b.x.0
        || a.y.0 > b.y.1
        || a.y.1 < b.y.0
        || a.z.0 > b.z.1
        || a.z.1 < b.z.0
    {
        return vec![a];
    }

    let splits = BTreeSet::from([
        Split::X(b.x.0),
        Split::X(b.x.1),
        Split::Y(b.y.0),
        Split::Y(b.y.1),
        Split::Z(b.z.0),
        Split::Z(b.z.1),
    ]);

    let mut cuboids = vec![a];
    // break cubes so they're entirely inside b or not inside b
    for s in splits {
        cuboids = cuboids
            .into_iter()
            .flat_map(|c| match s {
                Split::X(x) if c.x.0 <= x && x <= c.x.1 => vec![
                    Cuboid {
                        state: c.state,
                        x: (c.x.0, x - 1),
                        y: c.y,
                        z: c.z,
                    },
                    Cuboid {
                        state: c.state,
                        x: (x, x),
                        y: c.y,
                        z: c.z,
                    },
                    Cuboid {
                        state: c.state,
                        x: (x + 1, c.x.1),
                        y: c.y,
                        z: c.z,
                    },
                ],
                Split::Y(y) if c.y.0 <= y && y <= c.y.1 => vec![
                    Cuboid {
                        state: c.state,
                        x: c.x,
                        y: (c.y.0, y - 1),
                        z: c.z,
                    },
                    Cuboid {
                        state: c.state,
                        x: c.x,
                        y: (y, y),
                        z: c.z,
                    },
                    Cuboid {
                        state: c.state,
                        x: c.x,
                        y: (y + 1, c.y.1),
                        z: c.z,
                    },
                ],
                Split::Z(z) if c.z.0 <= z && z <= c.z.1 => vec![
                    Cuboid {
                        state: c.state,
                        x: c.x,
                        y: c.y,
                        z: (c.z.0, z - 1),
                    },
                    Cuboid {
                        state: c.state,
                        x: c.x,
                        y: c.y,
                        z: (z, z),
                    },
                    Cuboid {
                        state: c.state,
                        x: c.x,
                        y: c.y,
                        z: (z + 1, c.z.1),
                    },
                ],
                _ => vec![c],
            })
            .collect();

        // filter out cubes that are within b
        // and filter out invalid cubes
        // println!("Split: {:?} on unb cube {:?}", s, b);
        // println!("cuboids pre-filter {:?}", cuboids);
        cuboids = cuboids
            .into_iter()
            .filter(|c| {
                !(b.x.0 <= c.x.0
                    && c.x.1 <= b.x.1
                    && b.y.0 <= c.y.0
                    && c.y.1 <= b.y.1
                    && b.z.0 <= c.z.0
                    && c.z.1 <= b.z.1)
                    && c.x.0 <= c.x.1
                    && c.y.0 <= c.y.1
                    && c.z.0 <= c.z.1
            })
            .collect();
        // println!("cuboids post-filter {:?}", cuboids);
    }

    cuboids
}

fn main() {
    let cuboids: Vec<Cuboid> = include_str!("test.txt")
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

            Cuboid {
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
        .rev()
        .collect();

    // println!("{:?}", break_first_cube(cuboids[1], cuboids[0]));
    let disjoint_cubes: Vec<Cuboid> =
        cuboids
            .iter()
            .enumerate()
            .fold(vec![], |mut unbreakable_cubes, (i, &cube)| {
                let mut breakable_cubes = vec![cube];
                for unbreakable_cube in &unbreakable_cubes {
                    // println!("breakable cube: {:?}", breakable_cubes);
                    breakable_cubes = breakable_cubes
                        .into_iter()
                        .flat_map(|c| break_first_cube(c, *unbreakable_cube))
                        .collect_vec();
                    // println!("breakable cube: {:?}", breakable_cubes);
                }
                unbreakable_cubes.append(&mut breakable_cubes);
                println!("{}: {}", i, unbreakable_cubes.len());
                unbreakable_cubes
            });

    let volume: i64 = disjoint_cubes
        .iter()
        .map(|c| match c.state {
            State::On => (c.x.1 - c.x.0 + 1) * (c.y.1 - c.y.0 + 1) * (c.z.1 - c.z.0 + 1),
            State::Off => 0,
        })
        .sum();
    println!("{:?}", volume);
}
