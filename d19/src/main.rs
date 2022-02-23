use itertools::Itertools;
use std::cmp;
use std::collections::BTreeSet;
use std::collections::HashMap;

const THRESHOLD: usize = 12;

fn get_all_orientations(v: &[i32]) -> Vec<(i32, i32, i32)> {
    vec![
        (v[0], v[1], v[2]),
        (v[0], v[2], v[1]),
        (v[1], v[0], v[2]),
        (v[1], v[2], v[0]),
        (v[2], v[0], v[1]),
        (v[2], v[1], v[0]),
        (v[0], v[1], -v[2]),
        (v[0], v[2], -v[1]),
        (v[1], v[0], -v[2]),
        (v[1], v[2], -v[0]),
        (v[2], v[0], -v[1]),
        (v[2], v[1], -v[0]),
        (v[0], -v[1], v[2]),
        (v[0], -v[2], v[1]),
        (v[1], -v[0], v[2]),
        (v[1], -v[2], v[0]),
        (v[2], -v[0], v[1]),
        (v[2], -v[1], v[0]),
        (v[0], -v[1], -v[2]),
        (v[0], -v[2], -v[1]),
        (v[1], -v[0], -v[2]),
        (v[1], -v[2], -v[0]),
        (v[2], -v[0], -v[1]),
        (v[2], -v[1], -v[0]),
        (-v[0], v[1], v[2]),
        (-v[0], v[2], v[1]),
        (-v[1], v[0], v[2]),
        (-v[1], v[2], v[0]),
        (-v[2], v[0], v[1]),
        (-v[2], v[1], v[0]),
        (-v[0], v[1], -v[2]),
        (-v[0], v[2], -v[1]),
        (-v[1], v[0], -v[2]),
        (-v[1], v[2], -v[0]),
        (-v[2], v[0], -v[1]),
        (-v[2], v[1], -v[0]),
        (-v[0], -v[1], v[2]),
        (-v[0], -v[2], v[1]),
        (-v[1], -v[0], v[2]),
        (-v[1], -v[2], v[0]),
        (-v[2], -v[0], v[1]),
        (-v[2], -v[1], v[0]),
        (-v[0], -v[1], -v[2]),
        (-v[0], -v[2], -v[1]),
        (-v[1], -v[0], -v[2]),
        (-v[1], -v[2], -v[0]),
        (-v[2], -v[0], -v[1]),
        (-v[2], -v[1], -v[0]),
    ]
}

fn main() {
    let scanners: Vec<Vec<Vec<i32>>> = include_str!("input.txt")
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .skip(1)
                .map(|l| {
                    let toks = l.split(',').collect_vec();
                    vec![
                        toks[0].parse::<i32>().unwrap(),
                        toks[1].parse::<i32>().unwrap(),
                        toks[2].parse::<i32>().unwrap(),
                    ]
                })
                .collect()
        })
        .collect();

    let mut oriented_scanners = Vec::new();
    let mut oriented_scanners_dists = Vec::new();
    for scanner in &scanners {
        let mut oriented_scanner: Vec<Vec<(i32, i32, i32)>> = vec![Vec::new(); 48];
        for beacon in scanner {
            for (i, oriented_beacon) in get_all_orientations(beacon).iter().enumerate() {
                oriented_scanner[i].push(*oriented_beacon);
            }
        }

        // compute dists
        // distance matrix for all orientations of this scanner
        let mut dists = Vec::new();
        for sc in &oriented_scanner {
            // distance matrix for oriented scanner
            let mut dist = Vec::new();
            for i in 0..sc.len() {
                dist.push(BTreeSet::new());
                for j in 0..sc.len() {
                    dist[i].insert((sc[j].0 - sc[i].0, sc[j].1 - sc[i].1, sc[j].2 - sc[i].2));
                }
            }
            dists.push(dist);
        }
        oriented_scanners_dists.push(dists);
        oriented_scanners.push(oriented_scanner);
    }

    // correct orientation index
    let mut solved_indexes = vec![0];
    let mut final_orientation = HashMap::from([(0, 0)]);
    let mut scanner_pos = HashMap::from([(0, (0, 0, 0))]);

    let mut final_grid = BTreeSet::new();
    for beacon in &scanners[0] {
        final_grid.insert((beacon[0], beacon[1], beacon[2]));
    }

    while solved_indexes.len() < scanners.len() {
        for i in 1..scanners.len() {
            if solved_indexes.contains(&i) {
                continue;
            }

            'outer: for &j in &solved_indexes {
                // one of oriented_scanners[i] must match scanner_j
                let final_orientation_j = *final_orientation.get(&j).unwrap();
                let scanner_j = &oriented_scanners[j][final_orientation_j];
                let scanner_pos_j = scanner_pos.get(&j).unwrap();

                // which oriented_scanner matches scanner j?
                for (orientation_index, oriented_scanner_i) in
                    oriented_scanners[i].iter().enumerate()
                {
                    // find two beacons where dists(beacon_1) ^ dists(beacon_2) >= THRESHOLD
                    for (k, beacon_j) in scanner_j.iter().enumerate() {
                        for (l, beacon_i) in oriented_scanner_i.iter().enumerate() {
                            let dists_j = &oriented_scanners_dists[j][final_orientation_j][k];
                            let dists_i = &oriented_scanners_dists[i][orientation_index][l];
                            if dists_j.intersection(dists_i).count() >= THRESHOLD {
                                final_orientation.insert(i, orientation_index);

                                let true_scanner_pos = (
                                    scanner_pos_j.0 + beacon_j.0 - beacon_i.0,
                                    scanner_pos_j.1 + beacon_j.1 - beacon_i.1,
                                    scanner_pos_j.2 + beacon_j.2 - beacon_i.2,
                                );

                                scanner_pos.insert(i, true_scanner_pos);
                                for b in oriented_scanner_i {
                                    final_grid.insert((
                                        true_scanner_pos.0 + b.0,
                                        true_scanner_pos.1 + b.1,
                                        true_scanner_pos.2 + b.2,
                                    ));
                                }
                                println!("scanner_{} true position is: {:?}", i, true_scanner_pos);
                                solved_indexes.push(i);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("num unique: {}", final_grid.len());
    let mut max_dist = i32::min_value();
    for p1 in scanner_pos.values() {
        for p2 in scanner_pos.values() {
            max_dist = cmp::max(
                max_dist,
                (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs(),
            );
        }
    }

    println!("max dist is: {}", max_dist);

    println!("True positions: {:?}", scanner_pos);
}
