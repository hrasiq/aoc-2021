use std::{collections::BinaryHeap, cmp::Ordering};

const SIZE: usize = 100;
const START: usize = 0;

pub fn risk_level(scale: usize) -> usize {
    let map = include_str!("input")
        .lines()
        .flat_map(|l| l.as_bytes().to_vec())
        .map(|c| c - b'0')
        .collect::<Vec<_>>();

    let end = SIZE*SIZE*scale*scale -1;
    shortest_path(&map, START, end, scale)
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: usize,
}

impl Ord for State {
    // lower cost, higher pos
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra
fn shortest_path(m: &[u8], start: usize, goal: usize, scale: usize) -> usize {
    let size = scale*scale*SIZE*SIZE;
    let mut dist = (0..size).map(|_| usize::MAX).collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {cost: 0, pos: start});

    while let Some(State {cost, pos}) = heap.pop() {
        if pos == goal {
            return cost;
        }

        for n in around(m, pos, &scale) {
            let next = State {cost: cost+n.cost as usize, pos: n.pos};
            if next.cost < dist[n.pos] {
                heap.push(next);
                dist[n.pos] = next.cost;
            }
        }
    }
    0
}

fn around(m: &[u8], pos: usize, scale: &usize) -> Vec<State> {
    let size = SIZE*scale;
    let (x, y) = ((pos/size) as isize, (pos%size) as isize);
    let next = [(x-1, y), (x+1, y), (x, y-1), (x, y+1),];

    next.into_iter()
        .filter(|&(x, y)|
                (0..size).contains(&(x as usize)) &&
                (0..size).contains(&(y as usize)))
        .map(|(x, y)| {
            let (x, y) = (x as usize, y as usize);
            let additional_cost = x/SIZE + y/SIZE;
            let p = (x%SIZE)*SIZE + y%SIZE;
            State {
                pos: x*size + y,
                cost: (m[p] as usize + additional_cost-1) % 9 + 1,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_map() {
        assert_eq!(498, risk_level(1));
    }

    #[test]
    fn full_map() {
        assert_eq!(2901, risk_level(5));
    }
}
