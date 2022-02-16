#![feature(map_first_last)]
use std::cmp;
use std::collections::{BTreeSet, HashMap};

const GRID_SIZE: usize = 100;

fn main() {
    let deltas: [(isize, isize); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    let small_grid: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // expand grid
    let mut grid: [[u32; GRID_SIZE * 5]; GRID_SIZE*5] = [[0; GRID_SIZE*5]; GRID_SIZE*5];
    for i in 0..small_grid.len() {
        for j in 0..small_grid[0].len() {
            let val = small_grid[i][j];
            for r in 0..5 {
                for c in 0..5 {
                    let mut new_val = val + r as u32 + c as u32;
                    if new_val > 9 {
                        new_val = new_val % 10 + 1;
                    }
                    grid[GRID_SIZE*r + i][GRID_SIZE*c + j] = new_val;
                }
            }
        }
    }

    // dijsktra's
    let mut dist = BTreeSet::new();
    let mut vertices = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let d = if (i, j) == (0, 0) {
                0
            } else {
                i32::max_value()
            };
            dist.insert((d, (i, j)));
            vertices.insert((i, j), d);
        }
    }

    while let Some((d, (r, c))) = dist.pop_first() {
        vertices.remove(&(r, c));

        if r as usize == grid.len() - 1 && c as usize == grid[0].len() - 1 {
            println!("dist: ({}, {}), {}", r, c, d);
            return;
        }

        let neighbors: Vec<(usize, usize)> = deltas
            .iter()
            .map(|(dr, dc)| (r as isize + dr, c as isize + dc))
            .filter(|(r, c)| {
                *r >= 0 && *r < grid.len() as isize && *c >= 0 && *c < grid.len() as isize
            })
            .filter(|(r, c)| vertices.contains_key(&(*r as usize, *c as usize)))
            .map(|(r, c)| (r as usize, c as usize))
            .collect();

        for (nb_r, nb_c) in neighbors {
            let new_dist = d + grid[nb_r as usize][nb_c as usize] as i32;
            let cur_dist = *vertices.get(&(nb_r, nb_c)).unwrap();
            let min_dist = cmp::min(cur_dist, new_dist);
            assert!(dist.remove(&(cur_dist, (nb_r, nb_c))));
            dist.insert((min_dist, (nb_r, nb_c)));
            vertices.insert((nb_r, nb_c), min_dist);
        }
    }
}
