#![feature(box_patterns)]
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
enum Node {
    Number(u32),
    Pair { left: Box<Node>, right: Box<Node> },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Number(n) => write!(f, "{}", n),
            Node::Pair { left, right } => {
                write!(f, "[{},{}]", left, right)
            }
        }
    }
}

fn parse_snailfish_num(line: &mut std::str::Chars<'_>) -> Option<Node> {
    match line.next() {
        None => None,
        Some('[') => {
            let left = parse_snailfish_num(line)?;
            assert!(line.next() == Some(','));

            let right = parse_snailfish_num(line)?;
            assert!(line.next() == Some(']'));

            Some(Node::Pair {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        Some(d @ '0'..='9') => Some(Node::Number(d.to_digit(10)?)),
        _ => unreachable!(),
    }
}

fn add(left: Node, right: Node) -> Node {
    Node::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}

enum ExplodeAction {
    Exploded(u32, u32),
    ExplodedLeft(u32),
    ExplodedRight(u32),
    DoneExploding,
    Nothing,
}

fn find_rightmost_num(num: &mut Node) -> &mut Node {
    match num {
        Node::Number(_) => num,
        Node::Pair { left: _, right } => find_rightmost_num(right),
    }
}

fn find_leftmost_num(num: &mut Node) -> &mut Node {
    match num {
        Node::Number(_) => num,
        Node::Pair { left, right: _ } => find_leftmost_num(left),
    }
}

fn explode(num: &mut Node, depth: u8) -> ExplodeAction {
    match num {
        Node::Number(_) => ExplodeAction::Nothing,
        Node::Pair {
            left: box Node::Number(l),
            right: box Node::Number(r),
        } if depth >= 4 => {
            let action = ExplodeAction::Exploded(*l, *r);
            *num = Node::Number(0);
            action
        }
        Node::Pair { left, right } => {
            match explode(left, depth + 1) {
                action @ ExplodeAction::DoneExploding => return action,
                action @ ExplodeAction::Nothing => action,
                action @ ExplodeAction::ExplodedLeft(_) => return action,
                ExplodeAction::ExplodedRight(r) => match find_leftmost_num(right) {
                    Node::Pair { .. } => unreachable!(),
                    Node::Number(n) => {
                        *n += r;
                        return ExplodeAction::DoneExploding;
                    }
                },
                ExplodeAction::Exploded(l, r) => {
                    match find_leftmost_num(right) {
                        Node::Pair { .. } => unreachable!(),
                        Node::Number(n) => {
                            *n += r;
                            return ExplodeAction::ExplodedLeft(l); // carry l up one branch
                        }
                    }
                }
            };

            match explode(right, depth + 1) {
                action @ ExplodeAction::DoneExploding => action,
                action @ ExplodeAction::Nothing => action,
                action @ ExplodeAction::ExplodedRight(_) => action,
                ExplodeAction::ExplodedLeft(l) => match find_rightmost_num(left) {
                    Node::Pair { .. } => unreachable!(),
                    Node::Number(n) => {
                        *n += l;
                        ExplodeAction::DoneExploding
                    }
                },
                ExplodeAction::Exploded(l, r) => {
                    match find_rightmost_num(left) {
                        Node::Pair { .. } => unreachable!(),
                        Node::Number(n) => {
                            *n += l;
                            ExplodeAction::ExplodedRight(r) // carry l up one branch
                        }
                    }
                }
            }
        }
    }
}

enum SplitAction {
    Split,
    NoSplit,
}

fn split(node: &mut Node) -> SplitAction {
    match node {
        &mut Node::Number(n) => {
            if n > 9 {
                let left = n / 2;
                let right = n - left;
                *node = Node::Pair {
                    left: Box::new(Node::Number(left)),
                    right: Box::new(Node::Number(right)),
                };
                SplitAction::Split
            } else {
                SplitAction::NoSplit
            }
        }
        Node::Pair { left, right } => {
            match split(left) {
                SplitAction::NoSplit => (),
                a @ SplitAction::Split => return a,
            };

            split(right)
        }
    }
}

fn reduce(mut s: Node) -> Node {
    loop {
        match explode(&mut s, 0) {
            ExplodeAction::Nothing => (),
            _ => continue,
        }
        match split(&mut s) {
            SplitAction::NoSplit => break,
            SplitAction::Split => continue,
        }
    }

    s
}

fn magnitude(node: &Node) -> u32 {
    match node {
        &Node::Number(n) => n,
        Node::Pair { left, right } => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

fn main() {
    let mut nums: VecDeque<Node> = include_str!("test2.txt")
        .lines()
        .map(|l| parse_snailfish_num(&mut l.chars()).unwrap())
        .collect();

    let first = nums.pop_front().unwrap();
    let sum = nums.into_iter().fold(first, |acc, n| {
        let s = add(acc, n);
        reduce(s)
    });

    println!("{}", sum);
    println!("{}", magnitude(&sum));
}
