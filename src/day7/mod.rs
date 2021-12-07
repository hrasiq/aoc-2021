pub fn part1() -> isize {
    let mut crabs: Vec<isize> = include_str!("input")
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    // grab median
    crabs.sort_unstable();
    let mid = crabs.len()/2;
    let median = crabs[mid];

    crabs.into_iter().map(|i| (i-median).abs()).sum()
}

pub fn part2() -> isize {
    let crabs: Vec<isize> = include_str!("input")
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    let avg = crabs.iter().sum::<isize>() / crabs.len() as isize;
    (avg..)
        .take(2) // [floor, ceil]
        .map(|res| crabs
             .iter()
             .map(|&i| {
                 let distance = (i-res).abs();
                 distance * (distance + 1)/2
             })
             .sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_move() {
        assert_eq!(342534, part1());
    }

    #[test]
    fn crab_engineering() {
        assert_eq!(94004208, part2());
    }
}
