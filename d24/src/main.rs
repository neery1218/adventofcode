struct Registers {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Registers {
    fn new() -> Self {
        Registers {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn load(&self, symbol: &str) -> i64 {
        match symbol {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            _ => unreachable!("{}", symbol),
        }
    }

    fn store(&mut self, symbol: &str, val: i64) {
        match symbol {
            "w" => self.w = val,
            "x" => self.x = val,
            "y" => self.y = val,
            "z" => self.z = val,
            _ => unreachable!("{}", symbol),
        }
    }
}

fn process_line(l: &str) -> (&str, &str, Option<&str>) {
    let mut toks = l.split(' ');
    (toks.next().unwrap(), toks.next().unwrap(), toks.next())
}

fn parse_right(reg: &Registers, b: &str) -> i64 {
    match b {
        s @ ("w" | "x" | "y" | "z") => reg.load(s),
        s => s.parse::<i64>().unwrap(),
    }
}

fn run_program(reg: &mut Registers, input: &mut dyn Iterator<Item = char>) {
    include_str!("test.txt").lines().for_each(|line| {
        let (op, a, b) = process_line(line);
        match op {
            "inp" => {
                let val = input.next().unwrap().to_digit(10).unwrap() as i64;
                reg.store(a, val);
            }
            "add" => {
                let right = parse_right(&reg, b.unwrap());
                reg.store(a, right + reg.load(a));
            }
            "mul" => {
                let right = parse_right(&reg, b.unwrap());
                reg.store(a, right * reg.load(a));
            }
            "div" => {
                let right = parse_right(&reg, b.unwrap());
                reg.store(a, reg.load(a) / right);
            }
            "mod" => {
                let right = parse_right(&reg, b.unwrap());
                reg.store(a, reg.load(a) % right);
            }
            "eql" => {
                let right = parse_right(&reg, b.unwrap());
                reg.store(a, (reg.load(a) == right).then(|| 1).unwrap_or(0));
            }
            _ => unreachable!("{}", op),
        }
    });
}

fn main() {
    // let mut lowest = i64::max_value();

    let mut input = "59998426997979".chars();
    let mut reg = Registers::new();
    run_program(&mut reg, &mut input);
    println!(
        "register output: wxyz: {} {} {} {}",
        reg.w, reg.x, reg.y, reg.z
    );

    let mut input = Vec::new();
    println!("{} {:?}", solve(0, 0, &mut input), input);
}

const A: [i32; 14] = [12, 12, 12, 10, -11, -13, 15, 10, -2, -6, 14, 0, -15, -4];
const B: [i32; 14] = [8, 13, 8, 10, 12, 1, 13, 5, 10, 3, 2, 2, 12, 7];
const DIV: [i32; 14] = [1, 1, 1, 1, 26, 26, 1, 1, 26, 26, 1, 26, 26, 26];

fn solve(z: i32, i: usize, w: &mut Vec<usize>) -> bool {
    if i == 14 {
        return true;
    }

    match DIV[i] {
        1 => {
            for digit in 1..=9 {
                w.push(digit);
                if solve(26 * z + digit as i32 + B[i], i + 1, w) {
                    return true;
                }
                w.pop();
            }

            false
        }
        26 => match (z % 26) + A[i] {
            digit @ 1..=9 => {
                w.push(digit as usize);
                match solve(z / 26, i + 1, w) {
                    true => true,
                    false => {
                        w.pop();
                        false
                    }
                }
            }
            _ => false,
        },
        _ => unreachable!(),
    }
}
