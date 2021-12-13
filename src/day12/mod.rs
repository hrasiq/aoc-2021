use std::collections::HashMap;

const START: u8 = 0;
const END  : u8 = 1;

pub fn part1() -> usize {
    let mut graph: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut id = HashMap::from([("start", START), ("end", END)]);
    let mut small_caves = vec![];

    // closures
    let mut add_branch = |u, v| {
        let entry = graph.entry(u).or_insert_with(Vec::new);
        entry.push(v);
    };
    let mut to_id = |s| {
        let i = id.len() as u8;
        let id = *id.entry(s).or_insert(i);
        (s.as_bytes()[0] > b'Z').then(|| small_caves.push(id));
        id
    };

    // build graph
    include_str!("input")
        .lines()
        .for_each(|l| {
            let (u, v) = l.split_once('-').unwrap();
            let (u, v) = (to_id(u), to_id(v));
            add_branch(u, v);
            add_branch(v, u);
        });

    paths(&graph, START, &mut Vec::new(), &small_caves, true)
}

pub fn part2() -> usize {
    let mut graph: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut id = HashMap::from([("start", START), ("end", END)]);
    let mut small_caves = vec![];

    // closures
    let mut add_branch = |u, v| {
        let entry = graph.entry(u).or_insert_with(Vec::new);
        entry.push(v);
    };
    let mut to_id = |s| {
        let i = id.len() as u8;
        let id = *id.entry(s).or_insert(i);
        (s.as_bytes()[0] > b'Z').then(|| small_caves.push(id));
        id
    };

    // build graph
    include_str!("input")
        .lines()
        .for_each(|l| {
            let (u, v) = l.split_once('-').unwrap();
            let (u, v) = (to_id(u), to_id(v));
            add_branch(u, v);
            add_branch(v, u);
        });

    paths(&graph, START, &mut Vec::new(), &small_caves, false)
}

fn paths(graph: &HashMap<u8, Vec<u8>>,
         src: u8,
         path: &mut Vec<u8>,
         small_caves: &[u8],
         mut revisited: bool) -> usize {
    if src == END {
        return 1;
    }
    if small_caves.contains(&src) && path.contains(&src) {
        if revisited || src == START {
            return 0;
        }
        revisited = true;
    }
    path.push(src);
    let p = graph[&src].iter()
        .map(|&n| paths(graph, n, path, small_caves, revisited))
        .sum();
    path.pop();
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_visit() {
        assert_eq!(4495, part1());
    }

    #[test]
    fn dual_visit() {
        assert_eq!(131254, part2());
    }
}
