use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

const N: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    fn from(c: char) -> Self {
        match c {
            'A' => Pod::A,
            'B' => Pod::B,
            'C' => Pod::C,
            'D' => Pod::D,
            _ => unreachable!("{}", c),
        }
    }

    fn energy_per_step(&self) -> usize {
        match self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    }

    fn in_room(&self, (r,c): (isize, isize)) -> bool {
        match &self {
            Pod::A => [(2,3), (3,3)].contains(&(r,c)),
            Pod::B => [(2,5), (3,5)].contains(&(r,c)),
            Pod::C => [(2,7), (3,7)].contains(&(r,c)),
            Pod::D => [(2,9), (3,9)].contains(&(r,c)),
        }
    }

    fn dist(&self, (r,c): (isize, isize)) -> usize {
        match &self {
            Pod::A => (3 - c).abs() as usize,
            Pod::B => (5 - c).abs() as usize,
            Pod::C => (7 - c).abs() as usize,
            Pod::D => (9 - c).abs() as usize,
        }
    }
}

impl fmt::Display for Pod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pod::A => write!(f, "A"),
            Pod::B => write!(f, "B"),
            Pod::C => write!(f, "C"),
            Pod::D => write!(f, "D"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState {
    pos: BTreeMap<(isize, isize), Pod>,
    energy: usize,
}

impl GameState {
    fn new() -> Self {
        GameState {
            pos: BTreeMap::new(),
            energy: 0,
        }
    }

    fn done(&self) -> bool {
        self.pos.iter().all(|(rc, pod)| pod.in_room(*rc))
    }

    fn dist(&self) -> usize {
        self.pos.iter().map(|(rc, pod)| pod.dist(*rc)).sum()
    }
}

fn print_grid(gs: &GameState, grid: &Grid) {
    for j in 0..13 {
        print!("{:2}|", j);
    }
    println!();

    for i in 0..5 {
        for j in 0..13 {
            match gs.pos.get(&(i, j)) {
                None => print!("{:2}|", grid.get(&(i as isize, j as isize)).unwrap()),
                Some(s) => print!(" {}|", s),
            };
        }
        println!();
    }
}

type Grid = BTreeMap<(isize, isize), char>;

fn main() {
    let (grid, gs) = include_str!("test.txt").lines().enumerate().fold(
        (BTreeMap::new(), GameState::new()),
        |(mut grid, mut gs), (i, line)| {
            line.char_indices().for_each(|(j, c)| match c {
                'A' | 'B' | 'C' | 'D' => {
                    gs.pos.insert((i as isize,j as isize), Pod::from(c));
                    grid.insert((i as isize,j as isize), '.');
                }
                c => {
                    grid.insert((i as isize, j as isize), c);
                }
            });
            (grid, gs)
        },
    );

    print_grid(&gs, &grid);
}


// fn pos_to_str(pos: &Pos) -> String {
//     pos
//         .iter()
//         .map(|(rc, pod)| format!("{:?}-{}", rc, pod))
//         .collect()
// }

// fn search(
//     grid: &Grid,
//     pos: &Pos,
//     moves: &HashMap<String, Vec<(isize, isize)>>,
//     energy_expended: usize,
//     min_sol: &mut Option<usize>,
//     visited: &mut HashMap<String, usize>,
// ) {
//     match *min_sol {
//         Some(sol) if energy_expended >= sol => return,
//         _ => (),
//     };

//     let key = pos_to_str(pos);
//     match visited.get(&key) {
//         Some(&e) if e <= energy_expended => {
//             return
//         }
//         _ => (),
//     }
//     visited.insert(key, energy_expended);

//     // println!("Searching...");
//     // println!("Energy expended so far: {}", energy_expended);

//     if done(pos) {
//         println!("Done!");
//         print_grid(grid, pos);
//         println!("Energy: {}", energy_expended);
//         println!("Moves {:?}", moves);

//         match *min_sol {
//             None => *min_sol = Some(energy_expended),
//             Some(sol) => *min_sol = Some(cmp::min(sol, energy_expended)),
//         }

//         return;
//     }

//     let mut valid_moves: Vec<_> = pos
//         .iter()
//         .flat_map(|((i, j), pod)| {
//             if moves.get(pod).map(|v| v.len()).unwrap_or(0) == 2 {
//                 return vec![];
//             }

//             let energy_per_step = match pod.as_str() {
//                 "A1" | "A2" => 1,
//                 "B1" | "B2" => 10,
//                 "C1" | "C2" => 100,
//                 "D1" | "D2" => 1000,
//                 _ => unreachable!("{}", pod),
//             };

//             let mut moves_for_pod = BTreeMap::new();
//             let pod_rooms = match pod.as_str() {
//                 "A1" | "A2" => ROOMS_BY_POD[0],
//                 "B1" | "B2" => ROOMS_BY_POD[1],
//                 "C1" | "C2" => ROOMS_BY_POD[2],
//                 "D1" | "D2" => ROOMS_BY_POD[3],
//                 _ => unreachable!("{}", pod),
//             };

//             let pod_char = pod.chars().next();
//             match (
//                 pos.get(&pod_rooms[0]).map(|s| s.chars().next()),
//                 pos.get(&pod_rooms[1]).map(|s| s.chars().next()),
//             ) {
//                 (Some(a), Some(b)) if a == pod_char && b == pod_char => return vec![],
//                 _ => (),
//             }

//             if (*i, *j) == pod_rooms[1] {
//                 return vec![];
//             }

//             get_moves_for_pod((*i, *j), grid, pos, &mut moves_for_pod, 0, energy_per_step);
//             moves_for_pod
//                 .into_iter()
//                 .filter(|(rc, _)| !DOORS.contains(rc))
//                 .map(|(rc, energy)| (energy, pod, (*i, *j), rc))
//                 .collect()
//         })
//         .collect();

//     valid_moves.sort_unstable();

//     valid_moves
//         .into_iter()
//         .for_each(|(energy, pod, old_rc, new_rc)| {
//             let mut new_moves = moves.clone();
//             new_moves
//                 .entry(pod.clone())
//                 .and_modify(|e| e.push(new_rc))
//                 .or_insert_with(|| vec![new_rc]);

//             let pod_rooms = match pod.as_str() {
//                 "A1" | "A2" => ROOMS_BY_POD[0],
//                 "B1" | "B2" => ROOMS_BY_POD[1],
//                 "C1" | "C2" => ROOMS_BY_POD[2],
//                 "D1" | "D2" => ROOMS_BY_POD[3],
//                 _ => unreachable!("{}", pod),
//             };

//             if new_moves.get(pod).map_or(0, |v| v.len()) == 2 && !pod_rooms.contains(&new_rc) {
//                 return;
//             }

//             let mut new_pos = pos.clone();
//             new_pos.remove(&old_rc);
//             new_pos.insert(new_rc, pod.clone());

//             search(
//                 grid,
//                 &new_pos,
//                 &new_moves,
//                 energy_expended + energy,
//                 min_sol,
//                 visited,
//             );
//         });
// }

// fn get_moves_for_pod(
//     (r, c): (isize, isize),
//     grid: &Grid,
//     pos: &Pos,
//     moves: &mut BTreeMap<(isize, isize), usize>,
//     energy: usize,
//     energy_per_step: usize,
// ) {
//     NEIGHBORS
//         .iter()
//         .map(|(dr, dc)| (r + dr, c + dc))
//         .for_each(|(new_r, new_c)| match grid.get(&(new_r, new_c)) {
//             // out of bounds
//             None => (),
//             Some('#' | ' ') => (),

//             // already visited
//             Some('.') if moves.contains_key(&(new_r, new_c)) => (),

//             // collision
//             Some('.') if pos.contains_key(&(new_r, new_c)) => (),

//             // room mechanics
//             Some('.') => {
//                 moves.insert((new_r, new_c), energy + energy_per_step);
//                 get_moves_for_pod(
//                     (new_r, new_c),
//                     grid,
//                     pos,
//                     moves,
//                     energy + energy_per_step,
//                     energy_per_step,
//                 )
//             }
//             i => unreachable!("{:?}", i),
//         });
// }

// fn done(pos: &Pos) -> bool {
//     match (
//         pos.get(&ROOMS[0]).map(|s| s.as_str()),
//         pos.get(&ROOMS[1]).map(|s| s.as_str()),
//     ) {
//         (Some("A1"), Some("A2")) => (),
//         (Some("A2"), Some("A1")) => (),
//         _ => return false,
//     };

//     match (
//         pos.get(&ROOMS[2]).map(|s| s.as_str()),
//         pos.get(&ROOMS[3]).map(|s| s.as_str()),
//     ) {
//         (Some("B1"), Some("B2")) => (),
//         (Some("B2"), Some("B1")) => (),
//         _ => return false,
//     };

//     match (
//         pos.get(&ROOMS[4]).map(|s| s.as_str()),
//         pos.get(&ROOMS[5]).map(|s| s.as_str()),
//     ) {
//         (Some("C1"), Some("C2")) => (),
//         (Some("C2"), Some("C1")) => (),
//         _ => return false,
//     };

//     match (
//         pos.get(&ROOMS[6]).map(|s| s.as_str()),
//         pos.get(&ROOMS[7]).map(|s| s.as_str()),
//     ) {
//         (Some("D1"), Some("D2")) => (),
//         (Some("D2"), Some("D1")) => (),
//         _ => return false,
//     };

//     true
// }

// const DOORS: [(isize, isize); 4] = [(1, 3), (1, 5), (1, 7), (1, 9)];
// const ROOMS: [(isize, isize); 16] = [
//     (2, 3),
//     (3, 3),
//     (4, 3),
//     (5, 3),

//     (2, 5),
//     (3, 5),
//     (4, 5),
//     (5, 5),

//     (2, 7),
//     (3, 7),
//     (4, 7),
//     (5, 7),

//     (2, 9),
//     (3, 9),
//     (4, 9),
//     (5, 9),
// ];

// const ROOMS_BY_POD: [[(isize, isize); 2]; 4] = [
//     [(2, 3), (3, 3)],
//     [(2, 5), (3, 5)],
//     [(2, 7), (3, 7)],
//     [(2, 9), (3, 9)],
// ];
