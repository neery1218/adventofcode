use std::collections::HashSet;
use std::collections::VecDeque;

fn print_grid(grid: &[Vec<u32>]) {
    for row in grid.iter() {
        println!("{:?}", row);
    }
    println!();
}

fn main() {
    let mut grid: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut n_flashes = 0;

    for step in 0..100 {
        // increase energy level of all octopi
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                grid[i][j] += 1;
            }
        }

        n_flashes += flash(&mut grid);

        if grid.iter().all(|row| row.iter().all(|cell| *cell == 0)) {
            println!("All flashed on step {}", step + 1);
        }

        print_grid(&grid);
    }

    println!("{}", n_flashes);
}

fn add(
    i: i32,
    j: i32,
    grid: &mut [Vec<u32>],
    flashed: &mut HashSet<(usize, usize)>,
    q: &mut VecDeque<(usize, usize)>,
) {
    if i >= 0 && i < (grid.len() as i32) && j >= 0 && j < (grid[0].len() as i32) {
        grid[i as usize][j as usize] += 1;
        if grid[i as usize][j as usize] > 9 && !flashed.contains(&(i as usize, j as usize)) {
            q.push_back((i as usize, j as usize));
        }
    }
}

fn flash(grid: &mut [Vec<u32>]) -> i32 {
    let mut num_flashes = 0;
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // initialize queue with flash-ready octopi
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] > 9 {
                queue.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = queue.pop_front() {
        if grid[i][j] <= 9 || flashed.contains(&(i, j)) {
            continue;
        }
        flashed.insert((i, j));
        num_flashes += 1;

        let i_ = i as i32;
        let j_ = j as i32;
        add(i_ - 1, j_ - 1, grid, &mut flashed, &mut queue);
        add(i_ - 1, j_, grid, &mut flashed, &mut queue);
        add(i_ - 1, j_ + 1, grid, &mut flashed, &mut queue);
        add(i_, j_ + 1, grid, &mut flashed, &mut queue);
        add(i_, j_ - 1, grid, &mut flashed, &mut queue);
        add(i_ + 1, j_ + 1, grid, &mut flashed, &mut queue);
        add(i_ + 1, j_, grid, &mut flashed, &mut queue);
        add(i_ + 1, j_ - 1, grid, &mut flashed, &mut queue);
        grid[i][j] = 0;
    }

    for (i, j) in flashed {
        grid[i][j] = 0;
    }

    num_flashes
}
