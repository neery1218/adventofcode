fn main() {
    let mut grid: Vec<Vec<char>> = include_str!("test.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut n = 0;
    loop {
        let mut moved = false;

        // east
        for i in 0..grid.len() {
            let mut j = 0;

            let first_char = grid[i][0];
            while j < grid[0].len() {
                let next_index = (j + 1) % grid[0].len();
                match (grid[i][j], grid[i][next_index]) {
                    ('>', '.') if j == grid[0].len() - 1 && first_char == '>' => {
                        grid[i][j] = '>';
                        j += 1;
                    }
                    ('>', '.') => {
                        grid[i][j] = '.';
                        grid[i][next_index] = '>';
                        j += 2;
                        moved = true;
                    }
                    (a, b) => {
                        grid[i][j] = a;
                        grid[i][next_index] = b;
                        j += 1;
                    }
                }
            }
        }

        // south: go column by column
        for j in 0..grid[0].len() {
            let mut i = 0;
            let first_char = grid[0][j];
            while i < grid.len() {
                let next_index = (i + 1) % grid.len();
                match (grid[i][j], grid[next_index][j]) {
                    ('v', '.') if i == grid.len() - 1 && first_char == 'v' => {
                        grid[i][j] = 'v';
                        i += 1;
                    }
                    ('v', '.') => {
                        grid[i][j] = '.';
                        grid[next_index][j] = 'v';
                        i += 2;
                        moved = true;
                    }
                    (a, b) => {
                        grid[i][j] = a;
                        grid[next_index][j] = b;
                        i += 1;
                    }
                }
            }
        }

        n += 1;

        if n % 10 == 0 {
            println!("{}", n);
            for row in &grid {
                println!("{:?}", row);
            }
            println!();
        }

        if !moved {
            println!("{}", n);
            return;
        }
    }
}
