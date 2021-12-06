const SIZE: usize = 1000;

pub fn part1() -> usize {
    let map = include_str!("input")
        .lines()
        .map(|l| l.split(" -> ").flat_map(|csv| csv.split(',')))
        .map(|mut csv| {
            // would be cleaner with itertools or nom, but let's keep to std
            let x = csv.next().unwrap().parse::<usize>().unwrap();
            let y = csv.next().unwrap().parse::<usize>().unwrap();
            let u = csv.next().unwrap().parse::<usize>().unwrap();
            let v = csv.next().unwrap().parse::<usize>().unwrap();

            (x.min(u), y.min(v), u.max(x), v.max(y)) // keep coords ordered
        })
        .filter(|(x,y,u,v)| x == u || y == v) // filter for "straight" lines
        .fold(vec![0usize; SIZE*SIZE], |mut map, (x,y,u,v)| {
            if x == u {
                (y..=v).for_each(|y| map[x + y*SIZE] += 1);
            } else {
                (x..=u).for_each(|x| map[x + y*SIZE] += 1);
            }
            map
        });
    map.into_iter().filter(|&i| i > 1).count()
}

pub fn part2() -> usize {
    let map = include_str!("input")
        .lines()
        .map(|l| l.split(" -> ").flat_map(|csv| csv.split(',')))
        .map(|mut csv| {
            let x = csv.next().unwrap().parse::<usize>().unwrap();
            let y = csv.next().unwrap().parse::<usize>().unwrap();
            let u = csv.next().unwrap().parse::<usize>().unwrap();
            let v = csv.next().unwrap().parse::<usize>().unwrap();

            (x, y, u, v)
        })
        .fold(vec![0usize; SIZE*SIZE], |mut map, (x,y,u,v)| {
            if x == u {
                (y.min(v)..=v.max(y)).for_each(|y| map[x + y*SIZE] += 1);
            } else if y == v {
                (x.min(u)..=u.max(x)).for_each(|x| map[x + y*SIZE] += 1);
            } else {
                let range = |x: usize, y: usize| {
                    if x < y {
                        (x..=y).into_iter().collect::<Vec<_>>()
                    } else {
                        (y..=x).rev().collect::<Vec<_>>()
                    }
                };
                range(x, u).into_iter().zip(range(y,v)).for_each(|(x,y)| map[x + y*SIZE] += 1);
            }
            map
        });
    map.into_iter().filter(|&i| i > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_vertical() {
        assert_eq!(7674, part1());
    }

    #[test]
    fn diagonal() {
        assert_eq!(20898, part2());
    }
}
