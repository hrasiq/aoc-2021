use std::collections::HashSet;

const ROUNDS: usize = 100;

pub fn part1() -> usize {
    let mut dumbos: Vec<u8> = include_str!("input")
        .lines()
        .flat_map(|l| l.bytes().map(|i| i - b'0'))
        .collect();

    (0..ROUNDS).fold(0, |mut flashes, _| {
        let mut flashed: HashSet<usize> = HashSet::new();
        let mut stack = vec![];

        // increment step
        dumbos.iter_mut().for_each(|i| *i += 1);

        // push >nines onto stack
        dumbos.iter()
            .enumerate()
            .filter(|(_, &val)| val > 9)
            .for_each(|(pos, _)| stack.push(pos));

        while let Some(pos) = stack.pop() {
            // flash dumbo
            flashes += 1;
            flashed.insert(pos);

            // increment neighbors
            around(pos).for_each(|p| dumbos[p] += 1);

            // push surrounding dumbos
            for n in around(pos)
                .filter(|&p| dumbos[p] > 9)
                .filter(|p| !flashed.contains(p)) {
                    if !stack.contains(&n) {
                        stack.push(n);
                    }
                }
        }

        // zero flashed dumbos
        flashed.into_iter()
            .for_each(|p| dumbos[p] = 0);

        flashes
    })
}

pub fn part2() -> usize {
    let mut dumbos: Vec<u8> = include_str!("input")
        .lines()
        .flat_map(|l| l.bytes().map(|i| i - b'0'))
        .collect();

    for step in 0.. {
        let mut flashed: HashSet<usize> = HashSet::new();
        let mut stack = vec![];

        // increment step
        dumbos.iter_mut().for_each(|i| *i += 1);

        // push >nines onto stack
        dumbos.iter()
            .enumerate()
            .filter(|(_, &val)| val > 9)
            .for_each(|(pos, _)| stack.push(pos));

        while let Some(pos) = stack.pop() {
            // flash dumbo
            flashed.insert(pos);

            // increment neighbors
            around(pos).for_each(|p| dumbos[p] += 1);

            // push surrounding dumbos
            for n in around(pos)
                .filter(|&p| dumbos[p] > 9)
                .filter(|p| !flashed.contains(p)) {
                    if !stack.contains(&n) {
                        stack.push(n);
                    }
                }
        }

        if flashed.len() == 100 {
            return step+1;
        }

        // zero flashed dumbos
        flashed.into_iter()
            .for_each(|p| dumbos[p] = 0);
    }
    0
}

fn around(pos: usize) -> impl Iterator<Item=usize> {
    let (x, y) = ((pos/10) as isize, (pos%10) as isize);
    let mut v = vec![];

    for i in -1..=1 {
        for j in -1..=1 {
            v.push((x+i, y+j));
        }
    }

    // includes (x, y), but will be ignored due to HashSet filter
    v.into_iter()
        .filter(|(x, y)| (0..10).contains(x) && (0..10).contains(y))
        .map(|(x, y)| (x*10 + y) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_100() {
        assert_eq!(1705, part1());
    }

    #[test]
    fn synchronize() {
        assert_eq!(265, part2());
    }
}
