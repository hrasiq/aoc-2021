pub fn part1() -> usize {
    let depths : Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    depths.iter()
        .enumerate()
        .skip(1)
        .filter(|(i, &val)| val > depths[i-1])
        .count()
}

pub fn part2() -> usize {
    let depths : Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    depths.iter()
        .enumerate()
        .skip(3)
        .filter(|(i, &val)| val > depths[i-3])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_increase() {
        assert_eq!(1121, part1());
    }

    #[test]
    fn sliding_window() {
        assert_eq!(1065, part2());
    }
}
