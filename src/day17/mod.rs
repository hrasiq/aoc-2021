pub fn part1() -> isize {
    let area = include_str!("input").trim().split_once("=").unwrap().1;
    let y = area.split_once(", y=").unwrap().1;
    let (y, yy) = y.split_once("..").unwrap();

    let (y, yy): (isize, isize) = (y.parse().unwrap(), yy.parse().unwrap());
    let y = y.min(yy).abs();

    y * (y-1) / 2
}

pub fn part2() -> usize {
    let area = include_str!("input").trim().split_once("=").unwrap().1;
    let (x, y) = area.split_once(", y=").unwrap();
    let (x, xx) = x.split_once("..").unwrap();
    let (y, yy) = y.split_once("..").unwrap();

    let (x, xx): (isize, isize) = (x.parse().unwrap(), xx.parse().unwrap());
    let (y, yy): (isize, isize) = (y.parse().unwrap(), yy.parse().unwrap());

    let maxy = y.min(yy).abs();

    let mut n = 0;
    (0..=xx).for_each(|vx| {
        (y.min(yy)..=maxy).for_each(|vy| {
            if hit((x, xx), (y, yy), (vx, vy)) {
                n += 1;
            }
        })
    });
    n
}

fn hit((xmin, xmax): (isize, isize),
       (ymin, ymax): (isize, isize),
       (mut vx, mut vy): (isize, isize)) -> bool {
    let (mut x, mut y) = (0, 0);

    while x < xmax && y > ymin {
        x += vx;
        y += vy;

        if (xmin..=xmax).contains(&x) && (ymin..=ymax).contains(&y) {
            return true;
        }

        vy -= 1;
        vx -= 1;
        vx = vx.max(0);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_height() {
        assert_eq!(9180, part1())
    }

    #[test]
    fn initial_velocities() {
        assert_eq!(3767, part2())
    }
}
