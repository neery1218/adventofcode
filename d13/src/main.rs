use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    Vertical(i32),
    Horizontal(i32),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

fn main() {
    let (points, folds, _): (HashSet<Point>, Vec<Fold>, bool) =
        include_str!("input.txt").lines().fold(
            (HashSet::new(), Vec::new(), false),
            |(mut points, mut folds, parse_points), line| match parse_points {
                false => match line.split_once(',') {
                    None => (points, folds, true),
                    Some((x, y)) => {
                        points.insert(Point(x.parse().unwrap(), y.parse().unwrap()));
                        (points, folds, false)
                    }
                },
                true => {
                    let fold_str = line.split(' ').last().unwrap().split_once('=').unwrap();
                    match fold_str {
                        ("x", x) => folds.push(Fold::Vertical(x.parse().unwrap())),
                        ("y", y) => folds.push(Fold::Horizontal(y.parse().unwrap())),
                        _ => unreachable!(),
                    };
                    (points, folds, true)
                }
            },
        );

    // println!("{:?}", points);
    // println!("{:?}", folds);

    // q1. only use first fold.
    let final_points = compute_fold(&points, folds.first().unwrap());
    println!("{}", final_points.len());

    // q2 compute all folds
    let final_points = folds
        .iter()
        .fold(points, |ps, fold| compute_fold(&ps, fold));

    let max_x = final_points.iter().map(|p| p.0).max().unwrap();
    let max_y = final_points.iter().map(|p| p.1).max().unwrap();

    for r in 0..=max_y {
        for c in 0..max_x {
            if final_points.contains(&Point(c, r)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("#");
    }
}

fn compute_fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    points.iter().fold(HashSet::new(), |mut final_points, p| {
        match fold {
            Fold::Vertical(x) => {
                if p.0 > *x {
                    let new_x = *x - (p.0 - *x);
                    final_points.insert(Point(new_x, p.1));
                } else {
                    final_points.insert(*p);
                }
            }
            Fold::Horizontal(y) => {
                if p.1 > *y {
                    let new_y = *y - (p.1 - *y);
                    final_points.insert(Point(p.0, new_y));
                } else {
                    final_points.insert(*p);
                }
            }
        };
        final_points
    })
}
