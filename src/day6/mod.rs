const CYCLE: usize = 9;

pub fn part1() -> usize {
    let mut state = include_str!("input")
        .trim()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .fold([0; CYCLE], |mut state, i| {
            state[i] += 1;
            state
        });
    for _ in 0..80 {
        let new = state[0];
        for i in 0..CYCLE-1 {
            state[i] = state[i+1];
        }
        state[CYCLE-1] = new;
        state[CYCLE-3] += new;
    }
    state.into_iter().sum()
}

pub fn part2() -> usize {
    let mut state = include_str!("input")
        .trim()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .fold([0; CYCLE], |mut state, i| {
            state[i] += 1;
            state
        });
    for _ in 0..256 {
        let new = state[0];
        for i in 0..CYCLE-1 {
            state[i] = state[i+1];
        }
        state[CYCLE-1] = new;
        state[CYCLE-3] += new;
    }
    state.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_80() {
        assert_eq!(383160, part1());
    }

    #[test]
    fn simulate_256() {
        assert_eq!(1721148811504, part2());
    }
}
