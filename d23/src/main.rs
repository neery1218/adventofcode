#![feature(let_chains)]

use std::collections::{BTreeMap, HashMap};
use std::fmt;

type Grid = BTreeMap<(isize, isize), char>;

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
const MAX_ROOM_DEPTH: isize = 5;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Pod {
    A(usize),
    B(usize),
    C(usize),
    D(usize),
}

impl Pod {
    fn from(c: char, id: usize) -> Self {
        match c {
            'A' => Pod::A(id),
            'B' => Pod::B(id),
            'C' => Pod::C(id),
            'D' => Pod::D(id),
            _ => unreachable!("{}", c),
        }
    }

    fn energy_per_step(&self) -> usize {
        match self {
            Pod::A(_) => 1,
            Pod::B(_) => 10,
            Pod::C(_) => 100,
            Pod::D(_) => 1000,
        }
    }

    fn in_room(&self, (r, c): (isize, isize)) -> bool {
        match &self {
            Pod::A(_) => [(2, 3), (3, 3), (4, 3), (5, 3)].contains(&(r, c)),
            Pod::B(_) => [(2, 5), (3, 5), (4, 5), (5, 5)].contains(&(r, c)),
            Pod::C(_) => [(2, 7), (3, 7), (4, 7), (5, 7)].contains(&(r, c)),
            Pod::D(_) => [(2, 9), (3, 9), (4, 9), (5, 9)].contains(&(r, c)),
        }
    }

    fn in_right_place(&self, (r, c): (isize, isize), gs: &GameState) -> bool {
        if !self.in_room((r, c)) {
            return false;
        }

        (r..(MAX_ROOM_DEPTH + 1)).all(|d| {
            gs.pos
                .get(&(d, c))
                .map(|p| p.in_room((d, c)))
                .unwrap_or(false)
        })
    }

    fn in_wrong_room(&self, rc: (isize, isize)) -> bool {
        ROOMS.contains(&rc) && !self.in_room(rc)
    }

    fn in_door(&self, rc: (isize, isize)) -> bool {
        DOORS.contains(&rc)
    }

    fn dist(&self, (_, c): (isize, isize)) -> usize {
        match &self {
            Pod::A(_) => (3 - c).abs() as usize,
            Pod::B(_) => (5 - c).abs() as usize,
            Pod::C(_) => (7 - c).abs() as usize,
            Pod::D(_) => (9 - c).abs() as usize,
        }
    }
}

impl fmt::Display for Pod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pod::A(id) => write!(f, "A{}", id),
            Pod::B(id) => write!(f, "B{}", id),
            Pod::C(id) => write!(f, "C{}", id),
            Pod::D(id) => write!(f, "D{}", id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GameState {
    pos: BTreeMap<(isize, isize), Pod>,
}

// impl PartialOrd for GameState {
//     fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
//         Some(self.dist().cmp(&other.dist()))
//     }
// }

// impl Ord for GameState {
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         self.dist().cmp(&other.dist())
//     }
// }

impl GameState {
    fn new() -> Self {
        GameState {
            pos: BTreeMap::new(),
        }
    }

    fn done(&self) -> bool {
        self.pos.iter().all(|(rc, pod)| pod.in_room(*rc))
    }

    fn dist(&self) -> usize {
        self.pos.iter().map(|(rc, pod)| pod.dist(*rc)).sum()
    }

    /// sorted list of neighboring game states by manhattan distance
    fn get_neighbor_states(&self, grid: &Grid) -> Vec<(GameState, Pod, (isize, isize), usize)> {
        let mut states = BTreeMap::new();

        for (rc, pod) in &self.pos {
            if pod.in_right_place(*rc, self) {
                continue;
            }

            GameState::get_moves_for_pod(self, *rc, grid, &mut states, 0);
        }

        let mut vec_states: Vec<(GameState, Pod, (isize, isize), usize)> = states
            .into_iter()
            .filter(|(s, _)| s.pos.iter().all(|(rc, p)| !p.in_door(*rc)))
            .map(|(s, (p, rc, e))| (s, p, rc, e))
            .collect();
        vec_states.sort_unstable_by(|(gs_a, ..), (gs_b, ..)| gs_a.dist().cmp(&gs_b.dist()));
        vec_states
    }

    fn get_moves_for_pod(
        gs: &GameState,
        (r, c): (isize, isize),
        grid: &Grid,
        states: &mut BTreeMap<GameState, (Pod, (isize, isize), usize)>,
        energy: usize,
    ) {
        NEIGHBORS
            .iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .for_each(|rc| match (grid.get(&rc), gs.pos.get(&rc)) {
                // out of bounds
                (None, _) => (),
                (Some('#' | ' '), _) => (),

                // collision
                (Some('.'), Some(_)) => (),

                (Some('.'), None) => {
                    let mut new_gs = gs.clone();
                    let p = new_gs.pos.remove(&(r, c)).unwrap();
                    new_gs.pos.insert(rc, p);

                    match states.get(&new_gs) {
                        Some(&(.., e)) if e <= energy + p.energy_per_step() => return,
                        _ => (),
                    }

                    states.insert(new_gs.clone(), (p, rc, energy + p.energy_per_step()));
                    GameState::get_moves_for_pod(
                        &new_gs,
                        rc,
                        grid,
                        states,
                        energy + p.energy_per_step(),
                    )
                }
                i => unreachable!("{:?}", i),
            });
    }
}

fn print_grid(gs: &GameState, grid: &Grid) {
    for j in 0..13 {
        print!("{:2}|", j);
    }
    println!();

    for i in 0..7 {
        for j in 0..13 {
            match gs.pos.get(&(i, j)) {
                None => print!("{:2}|", grid.get(&(i as isize, j as isize)).unwrap()),
                Some(s) => print!("{}|", s),
            };
        }
        println!();
    }
}

fn main() {
    let (grid, gs, _) = include_str!("test.txt").lines().enumerate().fold(
        (BTreeMap::new(), GameState::new(), HashMap::new()),
        |(mut grid, mut gs, mut count), (i, line)| {
            line.char_indices().for_each(|(j, c)| match c {
                'A' | 'B' | 'C' | 'D' => {
                    let a = count.entry(c).and_modify(|e| *e += 1).or_insert(1);
                    gs.pos.insert((i as isize, j as isize), Pod::from(c, *a));
                    grid.insert((i as isize, j as isize), '.');
                }
                c => {
                    grid.insert((i as isize, j as isize), c);
                }
            });
            (grid, gs, count)
        },
    );

    print_grid(&gs, &grid);
    let mut states = BTreeMap::new();
    let mut min_state = None;
    let mut moves = HashMap::new();
    search(&gs, 0, &grid, &mut min_state, &mut states, &mut moves);
    println!("{:?}", min_state);
}

fn search(
    gs: &GameState,
    energy: usize,
    grid: &Grid,
    min_state: &mut Option<usize>,
    states: &mut BTreeMap<GameState, usize>,
    moves: &mut HashMap<Pod, usize>,
) {
    // print_grid(gs, grid);
    if gs.done() {
        match *min_state {
            Some(e) if e <= energy => return,
            _ => (),
        }

        println!("Done! with energy {}", energy);
        println!("{:?}", moves);
        *min_state = Some(energy);

        return;
    }
    // println!();

    // terminate early if solution has less energy than our current state
    match *min_state {
        Some(e) if e <= energy => return,
        _ => (),
    }

    // terminate early if we've encountered this state already, but with a lesser energy
    match states.get(gs) {
        Some(&e) if e <= energy => return,
        _ => (),
    }

    states.insert(gs.clone(), energy);
    for (neighbor, pod, rc, energy_used) in gs.get_neighbor_states(grid) {
        if let Some(2) = moves.get(&pod) {
            continue;
        }
        if let Some(1) = moves.get(&pod) && !pod.in_right_place(rc, &neighbor) {
            continue
        }
        if pod.in_wrong_room(rc) {
            continue;
        }
        if pod.in_room(rc) && !pod.in_right_place(rc, &neighbor) {
            continue;
        }

        moves.entry(pod).and_modify(|e| *e += 1).or_insert(1);
        search(
            &neighbor,
            energy + energy_used,
            grid,
            min_state,
            states,
            moves,
        );
        moves.entry(pod).and_modify(|e| *e -= 1);
    }
}
