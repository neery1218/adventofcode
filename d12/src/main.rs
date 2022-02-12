use std::collections::HashMap;

fn main() {
    let mut adjacency_list: HashMap<&str, Vec<&str>> = HashMap::new();
    include_str!("input.txt")
        .lines()
        .flat_map(|l| {
            let toks = l.split_once('-').unwrap();
            [(toks.0, toks.1), (toks.1, toks.0)]
        })
        .for_each(|(u, v)| {
            adjacency_list
                .entry(u)
                .and_modify(|neighbors| neighbors.push(v))
                .or_insert_with(|| vec![v]);
        });

    println!("{}", find_all_paths(&adjacency_list, vec!["start"]));
    println!(
        "{}",
        find_all_paths_two(&adjacency_list, vec!["start"], false)
    );
}

fn find_all_paths<'a>(adj_list: &'a HashMap<&str, Vec<&'a str>>, cur_path: Vec<&'a str>) -> u32 {
    let cur_node = cur_path.last().unwrap();
    if cur_node.eq(&"end") {
        return 1;
    }

    let mut total_paths = 0;
    for n in adj_list.get(cur_node).unwrap() {
        let is_uppercase = n.chars().all(|c| c.is_uppercase());
        if is_uppercase || !cur_path.contains(n) {
            let mut new_path = cur_path.clone();
            new_path.push(n);
            total_paths += find_all_paths(adj_list, new_path);
        }
    }

    total_paths
}

fn find_all_paths_two<'a>(
    adj_list: &'a HashMap<&str, Vec<&'a str>>,
    cur_path: Vec<&'a str>,
    visited_twice: bool,
) -> u32 {
    let cur_node = cur_path.last().unwrap();
    if cur_node.eq(&"end") {
        return 1;
    }

    let mut total_paths = 0;
    for n in adj_list.get(cur_node).unwrap() {
        // under what conditions can we _not_ add n to the path?
        // 1. n in ['start', 'end'] and n already in path
        // 2. n is lowercase, n is already in path, and we already visited a small cave twice
        let is_uppercase = n.chars().all(|c| c.is_uppercase());
        let n_count = cur_path.iter().filter(|v| v.eq(&n)).count();

        if n.eq(&"start") {
            continue;
        }

        if !is_uppercase && (n_count == 1 && visited_twice || n_count == 2) {
            continue;
        }

        let mut new_path = cur_path.clone();
        new_path.push(n);
        total_paths += find_all_paths_two(
            adj_list,
            new_path,
            visited_twice || (!is_uppercase && n_count == 1),
        );
    }

    total_paths
}
