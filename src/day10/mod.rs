const OPEN: &str = "([{<";

pub fn part1() -> usize {
    include_str!("input")
        .lines()
        .map(|l| l.chars()
             .scan(vec![], |s, c| match c {
                 c if OPEN.contains(c) => {s.push(c); Some(0)},
                 ')' => {if s.pop().unwrap() == '(' {Some(0)} else {Some(3)}},
                 ']' => {if s.pop().unwrap() == '[' {Some(0)} else {Some(57)}},
                 '}' => {if s.pop().unwrap() == '{' {Some(0)} else {Some(1197)}},
                 '>' => {if s.pop().unwrap() == '<' {Some(0)} else {Some(25137)}},
                 _ => Some(0),
             })
            .sum::<usize>()
        )
        .sum()
}

pub fn part2() -> usize {
    let mut scores = include_str!("input")
        .lines()
        .filter_map(|l| {
            let mut s = vec![];
            l.chars()
                .all(|c| match c {
                    c if OPEN.contains(c) => {s.push(c); true},
                    ')' => s.pop().unwrap() == '(',
                    ']' => s.pop().unwrap() == '[',
                    '}' => s.pop().unwrap() == '{',
                    '>' => s.pop().unwrap() == '<',
                    _ => false
                })
                .then(|| s)
        })
        .map(|s| s.into_iter()
             .rev()
             .map(|c| match c {
                 '(' => 1,
                 '[' => 2,
                 '{' => 3,
                 '<' => 4,
                 _ => 0
             })
             .fold(0_usize, |acc, val| val + acc*5)
        )
        .collect::<Vec<_>>();

    let mid = scores.len()/2;
    *scores.select_nth_unstable(mid).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrupted() {
        assert_eq!(339411, part1());
    }

    #[test]
    fn incomplete() {
        assert_eq!(2289754624, part2());
    }
}
