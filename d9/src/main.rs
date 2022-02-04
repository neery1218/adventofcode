use std::collections::HashSet;

fn main() {
    let grid: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // part 1
    let mut risk_level = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if i > 0 && grid[i - 1][j] <= grid[i][j]
                || j > 0 && grid[i][j - 1] <= grid[i][j]
                || i + 1 < grid.len() && grid[i + 1][j] <= grid[i][j]
                || j + 1 < grid[0].len() && grid[i][j + 1] <= grid[i][j]
            {
                continue;
            }

            risk_level += grid[i][j] + 1;
        }
    }
    println!("Risk level: {}", risk_level);

    // part 2
    let mut visited = HashSet::new();
    let mut basin_sizes = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val != 9 && !visited.contains(&(i, j)) {
                basin_sizes.push(count_basin_size(grid.as_slice(), &mut visited, i as i32, j as i32));
            }
        }
    }

    basin_sizes.sort_unstable();
    let product: u32 = basin_sizes.iter().rev().take(3).product();
    println!("{:?} {}", basin_sizes, product);
}

fn count_basin_size(
    grid: &[Vec<u32>],
    visited: &mut HashSet<(usize, usize)>,
    i: i32,
    j: i32,
) -> u32 {
    if i < 0 || j < 0 || i as usize == grid.len() || j as usize == grid[0].len() || grid[i as usize][j as usize] == 9 || visited.contains(&(i as usize, j as usize)) {
        return 0;
    }

    visited.insert((i as usize, j as usize));
    count_basin_size(grid, visited, i - 1, j)
        + count_basin_size(grid, visited, i + 1, j)
        + count_basin_size(grid, visited, i, j - 1)
        + count_basin_size(grid, visited, i, j + 1)
        + 1
}
