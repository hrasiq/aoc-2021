pub fn part1() -> usize {
    include_str!("input")
        .lines()
        .flat_map(|l| l.split('|').last().unwrap().split_whitespace())
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count()
}

pub fn part2() -> usize {
    include_str!("input")
        .lines()
        .map(|l| {
            let (pattern, output)  = l.split_once('|').unwrap();
            let pattern: Vec<&str> = pattern.split_whitespace().collect();

            // grab pattern for 1 and 4
            let one  = pattern.iter().find(|&&d| d.len() == 2).unwrap();
            let four = pattern.iter().find(|&&d| d.len() == 4).unwrap();

            let digits = output
                .split_whitespace()
                .map(|d| {
                    // how many segments are shared with pattern?
                    let shares_one  = d.chars().filter(|&c| one.contains(c)).count();
                    let shares_four = d.chars().filter(|&c| four.contains(c)).count();

                    match (d.len(), shares_one, shares_four) {
                        (2, _, _) => 1,
                        (5, 1, 2) => 2,
                        (5, 2, 3) => 3,
                        (4, _, _) => 4,
                        (5, 1, 3) => 5,
                        (6, 1, 3) => 6,
                        (3, _, _) => 7,
                        (7, _, _) => 8,
                        (6, 2, 4) => 9,
                        (6, 2, 3) => 0,
                        _ => panic!(),
                    }
                });

            // convert to base10
            digits
                .rev()
                .enumerate()
                .fold(0, |value, (i, d)| value + d*(10_usize.pow(i as u32)))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_segment_count() {
        assert_eq!(274, part1());
    }

    #[test]
    fn decode_sum() {
        assert_eq!(1012089, part2());
    }
}
