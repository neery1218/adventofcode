#[derive(Debug)]
struct Bingo {
    board: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
    completed: bool,
}

impl Bingo {
    fn new() -> Self {
        Bingo {
            board: [[0; 5]; 5],
            marked: [[false; 5]; 5],
            completed: false,
        }
    }

    fn is_complete(&self) -> bool {
        return (0..5).into_iter().any(|i| {
            (0..5).into_iter().all(|j| self.marked[i][j])
                || (0..5).into_iter().all(|j| self.marked[j][i])
        });
    }

    fn draw(&mut self, num: u32) {
        if self.completed {
            return;
        }

        for (i, row) in self.board.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val == num {
                    self.marked[i][j] = true;
                }
            }
        }

        if self.is_complete() {
            self.completed = true;
        }
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for (i, row) in self.board.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if !self.marked[i][j] {
                    sum += val;
                }
            }
        }
        sum
    }
}

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let scores = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut boards: Vec<Bingo> = Vec::new();
    while let Some(l) = lines.next() {
        assert!(l == "");

        let mut b: Bingo = Bingo::new();
        for i in 0..5 {
            for (j, val) in lines.next().unwrap().split_whitespace().enumerate() {
                b.board[i][j] = val.parse().unwrap();
            }
        }

        boards.push(b);
    }

    // part 1 and 2
    let num_boards = boards.len();
    let mut num_completed = 0;
    for s in &scores {
        for b in &mut boards {
            if b.completed {
                continue;
            }

            b.draw(*s);
            if b.completed {
                num_completed += 1;
                if num_completed == 1 || num_completed == num_boards {
                    println!("Winner! {}", b.sum_unmarked() * s);
                }
            }
        }
    }
}
