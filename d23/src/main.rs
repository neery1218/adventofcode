use std::cmp;
use std::collections::{BTreeMap, HashMap, HashSet};

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

type Grid = BTreeMap<(isize, isize), char>;
type Pos = HashMap<(isize, isize), String>;

fn main() {
    let (grid, _, pos) = include_str!("test.txt").lines().enumerate().fold(
        (BTreeMap::new(), HashSet::new(), HashMap::new()),
        |(mut grid, mut seen, mut pos), (i, line)| {
            line.char_indices().for_each(|(j, c)| match c {
                'A' | 'B' | 'C' | 'D' if seen.contains(&c) => {
                    let new_id = format!("{}2", c);
                    pos.insert((i as isize, j as isize), new_id);
                    grid.insert((i as isize, j as isize), '.');
                }
                'A' | 'B' | 'C' | 'D' if !seen.contains(&c) => {
                    let new_id = format!("{}1", c);
                    seen.insert(c);
                    pos.insert((i as isize, j as isize), new_id);
                    grid.insert((i as isize, j as isize), '.');
                }
                c => {
                    grid.insert((i as isize, j as isize), c);
                }
            });
            (grid, seen, pos)
        },
    );

    print_grid(&grid, &pos);
    let mut min_sol = None;
    println!(
        "{:?}",
        search(&grid, &pos, &HashMap::new(), 0, &mut min_sol, &mut HashMap::new())
    );
}

fn print_grid(grid: &Grid, pos: &Pos) {
    for j in 0..13 {
        print!("{:2}|", j);
    }
    println!();

    for i in 0..5 {
        for j in 0..13 {
            match pos.get(&(i, j)) {
                None => print!("{:2}|", grid.get(&(i as isize, j as isize)).unwrap()),
                Some(s) => print!("{:2}|", s),
            };
        }
        println!();
    }
}

fn pos_to_str(pos: &Pos) -> String {
    pos
        .iter()
        .map(|(rc, pod)| format!("{:?}-{}", rc, pod))
        .collect()
}

fn search(
    grid: &Grid,
    pos: &Pos,
    moves: &HashMap<String, Vec<(isize, isize)>>,
    energy_expended: usize,
    min_sol: &mut Option<usize>,
    visited: &mut HashMap<String, usize>,
) {
    match *min_sol {
        Some(sol) if energy_expended >= sol => return,
        _ => (),
    };

    let key = pos_to_str(pos);
    match visited.get(&key) {
        Some(&e) if e <= energy_expended => {
            return
        }
        _ => (),
    }
    visited.insert(key, energy_expended);

    // println!("Searching...");
    // println!("Energy expended so far: {}", energy_expended);

    if done(pos) {
        println!("Done!");
        print_grid(grid, pos);
        println!("Energy: {}", energy_expended);
        println!("Moves {:?}", moves);

        match *min_sol {
            None => *min_sol = Some(energy_expended),
            Some(sol) => *min_sol = Some(cmp::min(sol, energy_expended)),
        }

        return;
    }

    let mut valid_moves: Vec<_> = pos
        .iter()
        .flat_map(|((i, j), pod)| {
            if moves.get(pod).map(|v| v.len()).unwrap_or(0) == 2 {
                return vec![];
            }

            let energy_per_step = match pod.as_str() {
                "A1" | "A2" => 1,
                "B1" | "B2" => 10,
                "C1" | "C2" => 100,
                "D1" | "D2" => 1000,
                _ => unreachable!("{}", pod),
            };

            let mut moves_for_pod = BTreeMap::new();
            let pod_rooms = match pod.as_str() {
                "A1" | "A2" => ROOMS_BY_POD[0],
                "B1" | "B2" => ROOMS_BY_POD[1],
                "C1" | "C2" => ROOMS_BY_POD[2],
                "D1" | "D2" => ROOMS_BY_POD[3],
                _ => unreachable!("{}", pod),
            };

            let pod_char = pod.chars().next();
            match (
                pos.get(&pod_rooms[0]).map(|s| s.chars().next()),
                pos.get(&pod_rooms[1]).map(|s| s.chars().next()),
            ) {
                (Some(a), Some(b)) if a == pod_char && b == pod_char => return vec![],
                _ => (),
            }

            if (*i, *j) == pod_rooms[1] {
                return vec![];
            }

            get_moves_for_pod((*i, *j), grid, pos, &mut moves_for_pod, 0, energy_per_step);
            moves_for_pod
                .into_iter()
                .filter(|(rc, _)| !DOORS.contains(rc))
                .map(|(rc, energy)| (energy, pod, (*i, *j), rc))
                .collect()
        })
        .collect();

    valid_moves.sort_unstable();

    valid_moves
        .into_iter()
        .for_each(|(energy, pod, old_rc, new_rc)| {
            let mut new_moves = moves.clone();
            new_moves
                .entry(pod.clone())
                .and_modify(|e| e.push(new_rc))
                .or_insert_with(|| vec![new_rc]);

            let pod_rooms = match pod.as_str() {
                "A1" | "A2" => ROOMS_BY_POD[0],
                "B1" | "B2" => ROOMS_BY_POD[1],
                "C1" | "C2" => ROOMS_BY_POD[2],
                "D1" | "D2" => ROOMS_BY_POD[3],
                _ => unreachable!("{}", pod),
            };

            if new_moves.get(pod).map_or(0, |v| v.len()) == 2 && !pod_rooms.contains(&new_rc) {
                return;
            }

            let mut new_pos = pos.clone();
            new_pos.remove(&old_rc);
            new_pos.insert(new_rc, pod.clone());

            search(
                grid,
                &new_pos,
                &new_moves,
                energy_expended + energy,
                min_sol,
                visited,
            );
        });
}

fn get_moves_for_pod(
    (r, c): (isize, isize),
    grid: &Grid,
    pos: &Pos,
    moves: &mut BTreeMap<(isize, isize), usize>,
    energy: usize,
    energy_per_step: usize,
) {
    NEIGHBORS
        .iter()
        .map(|(dr, dc)| (r + dr, c + dc))
        .for_each(|(new_r, new_c)| match grid.get(&(new_r, new_c)) {
            // out of bounds
            None => (),
            Some('#' | ' ') => (),

            // already visited
            Some('.') if moves.contains_key(&(new_r, new_c)) => (),

            // collision
            Some('.') if pos.contains_key(&(new_r, new_c)) => (),

            // room mechanics
            Some('.') => {
                moves.insert((new_r, new_c), energy + energy_per_step);
                get_moves_for_pod(
                    (new_r, new_c),
                    grid,
                    pos,
                    moves,
                    energy + energy_per_step,
                    energy_per_step,
                )
            }
            i => unreachable!("{:?}", i),
        });
}

fn done(pos: &Pos) -> bool {
    match (
        pos.get(&ROOMS[0]).map(|s| s.as_str()),
        pos.get(&ROOMS[1]).map(|s| s.as_str()),
    ) {
        (Some("A1"), Some("A2")) => (),
        (Some("A2"), Some("A1")) => (),
        _ => return false,
    };

    match (
        pos.get(&ROOMS[2]).map(|s| s.as_str()),
        pos.get(&ROOMS[3]).map(|s| s.as_str()),
    ) {
        (Some("B1"), Some("B2")) => (),
        (Some("B2"), Some("B1")) => (),
        _ => return false,
    };

    match (
        pos.get(&ROOMS[4]).map(|s| s.as_str()),
        pos.get(&ROOMS[5]).map(|s| s.as_str()),
    ) {
        (Some("C1"), Some("C2")) => (),
        (Some("C2"), Some("C1")) => (),
        _ => return false,
    };

    match (
        pos.get(&ROOMS[6]).map(|s| s.as_str()),
        pos.get(&ROOMS[7]).map(|s| s.as_str()),
    ) {
        (Some("D1"), Some("D2")) => (),
        (Some("D2"), Some("D1")) => (),
        _ => return false,
    };

    true
}

const DOORS: [(isize, isize); 4] = [(1, 3), (1, 5), (1, 7), (1, 9)];
const ROOMS: [(isize, isize); 16] = [
    (2, 3),
    (3, 3),
    (4, 3),
    (5, 3),

    (2, 5),
    (3, 5),
    (4, 5),
    (5, 5),

    (2, 7),
    (3, 7),
    (4, 7),
    (5, 7),

    (2, 9),
    (3, 9),
    (4, 9),
    (5, 9),
];

const ROOMS_BY_POD: [[(isize, isize); 2]; 4] = [
    [(2, 3), (3, 3)],
    [(2, 5), (3, 5)],
    [(2, 7), (3, 7)],
    [(2, 9), (3, 9)],
];