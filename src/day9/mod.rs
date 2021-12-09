use std::collections::HashSet;

const SIZE: isize = 100;

pub fn part1() -> u32 {
    let map: Vec<u32> = include_str!("input")
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    map.iter()
        .enumerate()
        .filter(|&(i, _)| local_min(&map, i))
        .fold(0, |sum, (_, &val)| sum + val+1)
}

pub fn part2() -> u32 {
    let map: Vec<u32> = include_str!("input")
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    let mut visited = HashSet::new();
    let mut sizes: Vec<u32> = map.iter()
        .enumerate()
        .filter(|&(i, _)| local_min(&map, i))
        .map(|(i, _)| basin_size(&map, i, &mut visited))
        .collect();

    sizes.sort_unstable();
    sizes.into_iter()
        .rev()
        .take(3)
        .product()
}

// DFS from local min
fn basin_size(map: &[u32], pos: usize, visited: &mut HashSet<usize>) -> u32 {
    let mut size = 0;
    let mut to_visit = vec![pos];

    // while to_visit != empty
    while let Some(node) = to_visit.pop() {
        if visited.contains(&node) {
            continue;
        }

        let neighbors = adj(node);

        // filter and push into to_visit
        neighbors.into_iter()
            .map(|n| (n, map[n]))
            .filter(|&(_, val)| val < 9)
            .filter(|&(_, val)| val > map[node])
            .filter(|&(n, ___)| !visited.contains(&n))
            .for_each(|(n, _)| to_visit.push(n));

        // mark node as visited
        visited.insert(node);
        size += 1;
    }

    size
}

// check if pos is local_min
fn local_min(map: &[u32], pos: usize) -> bool {
    let neighbors = adj(pos);

    neighbors.iter()
        .map(|&n| map[n])
        .all(|i| i > map[pos])
}

// build adjacency list
fn adj(pos: usize) -> Vec<usize> {
    let p = pos as isize;
    let mut adj: &[isize] = &[p-1, p-SIZE, p+SIZE, p+1];

    if p%SIZE == 0 {
        adj = &adj[1..];  // remove first if left edge
    } else if p%SIZE == SIZE-1 {
        adj = &adj[0..3]; // remove last if right edge
    }

    // keep adj within bounds
    adj.iter()
        .filter(|&i| (0..SIZE*SIZE).contains(i))
        .map(|&i| i as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_points() {
        assert_eq!(570, part1());
    }

    #[test]
    fn basin_sizes() {
        assert_eq!(899392, part2());
    }
}
