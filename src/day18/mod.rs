const OPEN:  i8 = -1;
const CLOSE: i8 = -2;
const COMMA: i8 = -3;

pub fn part1() -> usize {
    let numbers = include_str!("input")
        .lines()
        .map(|l| l.bytes()
             .flat_map(|b| match b {
                 b'0'..=b'9' => Some((b - b'0') as i8),
                 b'[' => Some(OPEN),
                 b']' => Some(CLOSE),
                 b',' => Some(COMMA),
                 _ => None,
             })
             .collect())
        .collect::<Vec<Vec<_>>>();

    let result = numbers.iter()
        .skip(1)
        .fold(numbers[0].clone(), |acc, sn| {
            add(&acc, sn)
        });

    magnitude(&result)
}

pub fn part2() -> usize {
    let numbers = include_str!("input")
        .lines()
        .map(|l| l.bytes()
             .flat_map(|b| match b {
                 b'0'..=b'9' => Some((b - b'0') as i8),
                 b'[' => Some(OPEN),
                 b']' => Some(CLOSE),
                 b',' => Some(COMMA),
                 _ => None,
             })
             .collect())
        .collect::<Vec<Vec<_>>>();

    numbers.iter()
        .map(|x| {
            numbers.iter()
                .map(|y| {
                    if x != y {magnitude(&add(x, y))}
                    else {0}
                })
                .max().unwrap()
        })
        .max().unwrap()
}

fn magnitude(sn: &[i8]) -> usize {
    let mut res = 0;
    let mut l = 1;

    for i in sn {
        match *i {
            OPEN => l *= 3,
            CLOSE => l /= 2,
            COMMA => l = l / 3 * 2,
            val => res += l * val as usize,
        }
    }
    res
}

fn add(lhs: &[i8], rhs: &[i8]) -> Vec<i8> {
    let mut res = vec![OPEN];

    res.extend(lhs.iter());
    res.push(COMMA);
    res.extend(rhs.iter());
    res.push(CLOSE);

    while explode(&mut res) || split(&mut res) {}
    res
}

fn explode(sn: &mut Vec<i8>) -> bool {
    let mut depth = 0;

    // find first depth > 4
    if let Some(pos) = sn.iter().position(|&i| {
        match i {
            OPEN => depth += 1,
            CLOSE => depth -= 1,
            _ => (),
        }
        depth > 4
    }) {
        let (x, y) = (sn[pos+1], sn[pos+3]);

        // propagate left
        if let Some(val) = sn[..pos].iter_mut().rev().find(|&&mut i| i >= 0) {
            *val += x;
        }

        // propagate right
        if let Some(val) = sn[pos+4..].iter_mut().find(|&&mut i| i >= 0) {
            *val += y;
        }

        // replace
        sn.splice(pos..pos+5, [0]);

        return true;
    }

    false
}

fn split(sn: &mut Vec<i8>) -> bool {
    if let Some(pos) = sn.iter().position(|&i| i > 9) {
        let val = sn[pos];
        sn.splice(pos..pos+1, [OPEN, val >> 1, COMMA, (val+1) >> 1, CLOSE]);
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_magnitude() {
        assert_eq!(3216, part1());
    }

    #[test]
    fn largest_magnitude() {
        assert_eq!(4643, part2());
    }
}
