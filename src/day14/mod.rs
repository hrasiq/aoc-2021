pub fn simulate(steps: usize) -> usize {
    let (template, rules) = include_str!("input").split_once("\n\n").unwrap();

    let template = template.as_bytes().iter().map(|&b| b - b'A').collect::<Vec<_>>();

    // AB -> C => ((A, B), C)
    let mut rules = rules.lines()
        .map(|l| {
            let x = l.as_bytes()[0] - b'A';
            let y = l.as_bytes()[1] - b'A';
            let z = l.as_bytes().iter().last().unwrap() - b'A';
            ((x, y), z)
        })
        .collect::<Vec<_>>();
    rules.sort_unstable_by_key(|&r| r.0);

    // ([rule], index, index)
    let rules = rules.iter()
        .map(|&((x, y), z)| {
            ([x, y], // can lookup template.windows(2) directly
             rules.binary_search_by_key(&(x, z), |&r| r.0).unwrap(),
             rules.binary_search_by_key(&(z, y), |&r| r.0).unwrap())
        })
        .collect::<Vec<_>>();

    // init state from template
    let (mut state, mut next) = (vec![0; rules.len()], vec![0; rules.len()]);
    template.windows(2).for_each(|key| {
        let index =  rules.binary_search_by_key(&key, |r| &r.0).unwrap();
        state[index] += 1;
    });

    // build next, clear state, swap
    (0..steps).for_each(|_| {
        state.iter_mut().zip(&rules).for_each(|(n, r)| {
            next[r.1] += *n;
            next[r.2] += *n;
            *n = 0;
        });
        std::mem::swap(&mut state, &mut next);
    });

    let mut count = [0; 26];
    count[*template.last().unwrap() as usize] += 1;
    state.iter()
        .zip(rules)
        .for_each(|(n, r)| count[r.0[0] as usize] += n);

    let (min, max) = count.into_iter()
        .filter(|&n| n != 0)
        .fold((usize::MAX, 0), |(min, max), n| (min.min(n), max.max(n)));
    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_10() {
        assert_eq!(2947, simulate(10));
    }

    #[test]
    fn simulate_40() {
        assert_eq!(3232426226464, simulate(40));
    }
}
