use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct GameState {
    pos: [usize; 2],
    scores: [usize; 2],
    turn: usize,
}

fn find_winners(
    mut gs: GameState,
    roll: usize,
    h: &mut HashMap<GameState, (usize, usize)>,
) -> (usize, usize) {
    gs.pos[gs.turn] = (gs.pos[gs.turn] + roll) % 10;
    gs.scores[gs.turn] += gs.pos[gs.turn] + 1;
    if gs.scores[gs.turn] >= 21 {
        return match gs.turn {
            0 => (1, 0),
            1 => (0, 1),
            _ => unreachable!(),
        };
    }
    gs.turn = (gs.turn + 1) % 2;

    if let Some(s) = h.get(&gs) {
        return *s;
    }

    let mut s = (0, 0);
    for x in [1, 2, 3] {
        for y in [1, 2, 3] {
            for z in [1, 2, 3] {
                let res = find_winners(gs.clone(), x + y + z, h);
                s = (s.0 + res.0, s.1 + res.1);
            }
        }
    }

    if let Some(_s) = h.get(&gs) {
        println!("Already exists! {:?} {:?} {:?}", gs, s, _s);
    }

    h.insert(gs, s);
    s
}

fn main() {
    let gs = GameState {
        pos: [1, 0],
        scores: [0, 0],
        turn: 0,
    };

    let mut s = (0, 0);
    let mut h = HashMap::new();
    for x in [1, 2, 3] {
        for y in [1, 2, 3] {
            for z in [1, 2, 3] {
                let res = find_winners(gs.clone(), x + y + z, &mut h);
                s = (s.0 + res.0, s.1 + res.1);
            }
        }
    }

    println!("{:?}", s);
}
