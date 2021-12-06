const SIZE: usize = 5;
type Board = Vec<isize>;

pub fn part1() -> usize {
    let mut input = include_str!("input").split("\n\n");

    let draws: Vec<isize> = input
        .next()
        .unwrap()
        .split(',')
        .map(|val| val.parse::<isize>().unwrap())
        .collect();

    let mut boards: Vec<Board> = input
        .map(|b| b.split_whitespace()
             .map(|i| i.parse().unwrap()).collect())
        .collect();

    for draw in draws {
        for b in boards.iter_mut() {
            mark_board(&draw, b);
            if let Some(score) = check_score(b) {
                return (score * draw) as usize;
            }
        }
    }
    0
}

pub fn part2() -> usize {
    let mut input = include_str!("input").split("\n\n");

    let draws: Vec<isize> = input
        .next()
        .unwrap()
        .split(',')
        .map(|val| val.parse::<isize>().unwrap())
        .collect();

    let mut boards: Vec<Board> = input
        .map(|b| b.split_whitespace()
             .map(|i| i.parse().unwrap()).collect())
        .collect();

    let mut last_score = 0;
    for draw in draws {
        boards = boards.into_iter().filter(|b| check_score(b).is_none()).collect();
        for b in boards.iter_mut() {
            mark_board(&draw, b);
            if let Some(score) = check_score(b) {
                last_score = score * draw;
            }
        }
    }
    last_score as usize
}

fn mark_board(val: &isize, board: &mut Board) {
    for entry in board.iter_mut().filter(|x| *x == val) {
        *entry = -1;
    }
}

fn check_score(b: &Board) -> Option<isize> {
    let row_victory: bool =
        (0..SIZE)
        .map(|i|
             b.iter()
             .skip(SIZE*i)
             .take(SIZE)
             .all(|&val| val < 0))
        .any(|i| i);
    let column_victory: bool =
        (0..SIZE)
        .map(|i|
             b.iter()
             .skip(i)
             .step_by(SIZE)
             .all(|&val| val < 0))
        .any(|i| i);

    if row_victory || column_victory {
        Some(b.iter().filter(|&&val| val > 0).copied().sum())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_to_win() {
        assert_eq!(16674, part1());
    }

    #[test]
    fn last_to_win() {
        assert_eq!(7075, part2());
    }
}
