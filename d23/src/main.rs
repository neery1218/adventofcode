use std::cmp;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

type Grid = BTreeMap<(isize, isize), char>;
const N: usize = 2;

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

    fn in_room(&self, (r, c): (isize, isize)) -> bool {
        match &self {
            Pod::A => [(2, 3), (3, 3)].contains(&(r, c)),
            Pod::B => [(2, 5), (3, 5)].contains(&(r, c)),
            Pod::C => [(2, 7), (3, 7)].contains(&(r, c)),
            Pod::D => [(2, 9), (3, 9)].contains(&(r, c)),
        }
    }

    // commented out bc it's easy to use incorrectly
    // fn in_wrong_room(&self, rc: (isize, isize)) -> bool {
    //     ROOMS.contains(&rc) && !self.in_room(rc)
    // }

    fn in_door(&self, rc: (isize, isize)) -> bool {
        DOORS.contains(&rc)
    }

    fn dist(&self, (r, c): (isize, isize)) -> usize {
        match &self {
            Pod::A => ((3 - c).abs() + cmp::min((2 - r).abs(), (3 - r).abs())) as usize,
            Pod::B => ((5 - c).abs() + cmp::min((2 - r).abs(), (3 - r).abs())) as usize,
            Pod::C => ((7 - c).abs() + cmp::min((2 - r).abs(), (3 - r).abs())) as usize,
            Pod::D => ((9 - c).abs() + cmp::min((2 - r).abs(), (3 - r).abs())) as usize,
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
    fn get_neighbor_states(&self, grid: &Grid) -> Vec<(GameState, usize)> {
        let mut states = BTreeMap::new();
        states.insert(self.clone(), 0);

        println!("{:?}", self.pos);
        for rc in self.pos.keys() {
            GameState::get_moves_for_pod(self, *rc, grid, &mut states, 0);
        }

        let mut vec_states: Vec<(GameState, usize)> = states
            .into_iter()
            .filter(|(s, _)| s.pos.iter().all(|(rc, p)| !p.in_door(*rc)))
            .collect();
        vec_states.sort_unstable_by(|(gs_a, _), (gs_b, _)| gs_a.dist().cmp(&gs_b.dist()));
        vec_states
    }

    fn get_moves_for_pod(
        gs: &GameState,
        (r, c): (isize, isize),
        grid: &Grid,
        states: &mut BTreeMap<GameState, usize>,
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
                        Some(&e) if e <= energy + p.energy_per_step() => return,
                        _ => (),
                    }

                    states.insert(new_gs.clone(), energy + p.energy_per_step());
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

fn main() {
    let (grid, gs) = include_str!("test.txt").lines().enumerate().fold(
        (BTreeMap::new(), GameState::new()),
        |(mut grid, mut gs), (i, line)| {
            line.char_indices().for_each(|(j, c)| match c {
                'A' | 'B' | 'C' | 'D' => {
                    gs.pos.insert((i as isize, j as isize), Pod::from(c));
                    grid.insert((i as isize, j as isize), '.');
                }
                c => {
                    grid.insert((i as isize, j as isize), c);
                }
            });
            (grid, gs)
        },
    );

    print_grid(&gs, &grid);

    let mut states = BTreeMap::new();
    let mut min_state = None;
    search(&gs, 0, &grid, &mut min_state, &mut states);
    println!("{:?}", min_state);
}

fn search(
    gs: &GameState,
    energy: usize,
    grid: &Grid,
    min_state: &mut Option<usize>,
    states: &mut BTreeMap<GameState, usize>,
) {
    if gs.done() {
        match *min_state {
            None => *min_state = Some(energy),
            Some(e) if e > energy => *min_state = Some(energy),
            _ => (),
        }

        return;
    }

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
    for (neighbor, energy_used) in gs.get_neighbor_states(grid) {
        search(&neighbor, energy + energy_used, grid, min_state, states);
    }
}
