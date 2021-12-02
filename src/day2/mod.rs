pub fn part1() -> usize {
    let (horizontal, vertical) = include_str!("input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(h, v), (direction, value)| {
            match (direction, value.parse::<usize>().unwrap()) {
                ("forward", x) => (h+x, v),
                ("up", x)      => (h, v-x),
                ("down", x)    => (h, v+x),
                _              => (h, v),
            }
        });
    horizontal * vertical
}

pub fn part2() -> usize {
    let (horizontal, vertical, _) = include_str!("input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(h, v, a), (direction, value)| {
            match (direction, value.parse::<usize>().unwrap()) {
                ("forward", x) => (h+x, v+(x*a), a),
                ("up", x)      => (h, v, a-x),
                ("down", x)    => (h, v, a+x),
                _              => (h, v, a)
            }
        });
    horizontal * vertical
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_direction() {
        assert_eq!(1660158, part1())
    }

    #[test]
    fn accurate_instructions() {
        assert_eq!(1604592846, part2())
    }
}
