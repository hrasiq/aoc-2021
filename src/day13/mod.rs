pub fn part1() -> usize {
    let (pairs, folds) = include_str!("input").split_once("\n\n").unwrap();

    let mut pairs: Vec<(u32, u32)> = pairs.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(),
                       y.parse::<u32>().unwrap()))
        .collect();

    // (is_vertical, line)
    let folds: Vec<(bool, u32)> = folds.lines()
        .map(|l| l.split_ascii_whitespace().last().unwrap())
        .map(|rule| rule.split_once('=').unwrap())
        .map(|(dir, val)| (dir.as_bytes()[0] == b'x',
                           val.parse::<u32>().unwrap()))
        .collect();

    let (dir, pos) = folds[0];
    if dir {
        pairs.iter_mut()
            .filter(|(x, _)| *x > pos)
            .for_each(|(x, _)| {
                *x = 2*pos - *x;
            });
    } else {
        pairs.iter_mut()
            .filter(|(_, y)| *y > pos)
            .for_each(|(_, y)| {
                *y = 2*pos - *y;
            });
    }
    pairs.sort_unstable();
    pairs.dedup();

    pairs.len()
}

#[cfg(test)]
fn part2(print: bool) {
    let (pairs, folds) = include_str!("input").split_once("\n\n").unwrap();

    let mut pairs: Vec<(u32, u32)> = pairs.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(),
                       y.parse::<u32>().unwrap()))
        .collect();

    // (is_vertical, line)
    let folds: Vec<(bool, u32)> = folds.lines()
        .map(|l| l.split_ascii_whitespace().last().unwrap())
        .map(|rule| rule.split_once('=').unwrap())
        .map(|(dir, val)| (dir.as_bytes()[0] == b'x',
                           val.parse::<u32>().unwrap()))
        .collect();

    folds.iter()
        .for_each(|&(dir, pos)| {
            if dir {
                pairs.iter_mut()
                    .filter(|(x, _)| *x > pos)
                    .for_each(|(x, _)| {
                        *x = 2*pos - *x;
                    });
            } else {
                pairs.iter_mut()
                    .filter(|(_, y)| *y > pos)
                    .for_each(|(_, y)| {
                        *y = 2*pos - *y;
                    });
            }
            pairs.sort_unstable();
            pairs.dedup();
        });
    print.then(|| display(&pairs));
}

#[cfg(test)]
fn display(pairs: &[(u32, u32)]) {
    let (mut l, mut h) = (0, 0);
    pairs.iter()
        .for_each(|&(x, y)| {
            l = std::cmp::max(l, x);
            h = std::cmp::max(h, y);
        });

    (0..=h).for_each(|h| {
        (0..=l).for_each(|l| {
            if pairs.contains(&(l, h)) {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_fold() {
        assert_eq!(781, part1());
    }

    #[test]
    fn code() {
        part2(true);
    }
}
