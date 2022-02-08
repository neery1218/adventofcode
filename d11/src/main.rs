const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn print_grid(grid: &[Vec<u32>]) {
    for row in grid.iter() {
        println!("{:?}", row);
    }
    println!();
}

fn main() {
    let mut grid: Vec<Vec<u32>> = include_str!("test.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let row_size = grid.len();
    let col_size = grid[0].len();

    let n_flashes = (0..100).fold(0, |acc, _| {
        // increment
        grid.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
        let l_flashes = (0..row_size)
            .flat_map(|r| (0..col_size).map(move |c| (r, c)))
            .fold(0, |acc, (r, c)| {
                acc + (grid[r][c] > 9)
                    .then(|| flash(&mut grid, r, c))
                    .unwrap_or(0)
            });
        println!("{}", l_flashes);
        acc + l_flashes
    });
    println!("{}", n_flashes);
}

fn flash(grid: &mut [Vec<u32>], i: usize, j: usize) -> i32 {
    grid[i][j] = 0;
    // all cells have been incremented, so there aren't any cells where cell.value == 0
    // UNLESS it just flashed
    NEIGHBORS
        .map(|(d_y, d_x)| ((d_y + i as isize) as usize, (d_x + j as isize) as usize))
        .into_iter()
        .fold(1, |acc, (r, c)| {
            match grid.get_mut(r).and_then(|row| row.get_mut(c)) {
                Some(cell) if *cell > 0 => {
                    *cell += 1;
                    acc + (*cell > 9).then(|| flash(grid, r, c)).unwrap_or(0)
                }
                _ => acc,
            }
        })
}
