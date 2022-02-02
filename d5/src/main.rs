use std::cmp;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let points: Vec<(Point, Point)> = include_str!("input.txt")
        .lines()
        .map(|l| {
            let toks: Vec<&str> = l.splitn(3, ' ').collect();
            let left: Vec<i32> = toks[0].splitn(2, ',').map(|t| t.parse().unwrap()).collect();
            let right: Vec<i32> = toks[2].splitn(2, ',').map(|t| t.parse().unwrap()).collect();
            (
                Point {
                    x: left[0],
                    y: left[1],
                },
                Point {
                    x: right[0],
                    y: right[1],
                },
            )
        })
        .collect();

    // only keep horizontal and vertical and diagonal lines
    let lines: Vec<(Point, Point)> = points
        .into_iter()
        .filter(|(p1, p2)| {
            p1.x == p2.x || p1.y == p2.y || (p1.x - p2.x).abs() == (p1.y - p2.y).abs()
        })
        .collect();

    let mut seen: HashSet<Point> = HashSet::new();
    let mut seen_twice: HashSet<Point> = HashSet::new();
    for (p1, p2) in &lines {
        if p1.x == p2.x {
            let (min_y, max_y) = (cmp::min(p1.y, p2.y), cmp::max(p1.y, p2.y));
            for y in min_y..(max_y + 1) {
                let p = Point { x: p1.x, y };
                if seen.contains(&p) {
                    seen_twice.insert(p);
                } else {
                    seen.insert(p);
                }
            }
        } else if p1.y == p2.y {
            let (min_x, max_x) = (cmp::min(p1.x, p2.x), cmp::max(p1.x, p2.x));
            for x in min_x..(max_x + 1) {
                let p = Point { x, y: p1.y };
                if seen.contains(&p) {
                    seen_twice.insert(p);
                } else {
                    seen.insert(p);
                }
            }
        } else {
            let step_x = if p2.x - p1.x > 0 { 1 } else { -1 };
            let step_y = if p2.y - p1.y > 0 { 1 } else { -1 };
            let mut i = 0;
            while i <= (p2.x - p1.x).abs() {
                let p = Point {
                    x: p1.x + step_x * i,
                    y: p1.y + step_y * i,
                };
                if seen.contains(&p) {
                    seen_twice.insert(p);
                } else {
                    seen.insert(p);
                }
                i += 1;
            }
        }
    }
    println!("{}", seen_twice.len());
}
