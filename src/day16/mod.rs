struct State<'a> {
    ver: usize,
    subs: Vec::<usize>,
    bits: &'a [u8],
}

// part 1
pub fn eval_version() -> usize {
    let bits = include_str!("input")
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).map(move |i| (1 & n >> (3 - i)) as u8)
        })
        .collect::<Vec<_>>();

    packet(&bits).ver
}

// part 2
pub fn eval_result() -> usize {
    let bits = include_str!("input")
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).map(move |i| (1 & n >> (3 - i)) as u8)
        })
        .collect::<Vec<_>>();

    packet(&bits).subs[0]
}

fn packet(bits: &[u8]) -> State {
    let mut ver = num(&bits[0..3]);
    let tid = num(&bits[3..6]);

    let mut bits = &bits[6..];

    if tid == 4 {
        let mut val = vec![];
        while {
            let last = bits[0] == 0;
            val.extend(&bits[1..5]);
            bits = &bits[5..];
            !last
        } {}; // sneaky do/while
        return State {ver, bits, subs: vec![num(&val)]}
    }

    let mut subs: Vec<usize> = vec![];
    let (lid, mut bits) = bits.split_at(1);

    if lid[0] == 0 {
        let len = num(&bits[0..15]) as usize;
        bits = &bits[15..];
        let (mut payload, rest) = bits.split_at(len);
        bits = rest;
        while !payload.is_empty() {
            let s = packet(payload);
            payload = s.bits;
            ver += s.ver;
            subs.extend(s.subs.iter())
        }
    } else {
        let (subpackets, rest) = bits.split_at(11);
        bits = rest;
        (0..num(subpackets)).for_each(|_| {
            let s = packet(bits);
            bits = s.bits;
            ver += s.ver;
            subs.extend(s.subs.iter());
        });
    }

    match tid {
        0 => State {ver, bits, subs: vec![subs.iter().sum()]},
        1 => State {ver, bits, subs: vec![subs.iter().product()]},
        2 => State {ver, bits, subs: vec![*subs.iter().min().unwrap()]},
        3 => State {ver, bits, subs: vec![*subs.iter().max().unwrap()]},
        5 => State {ver, bits, subs: vec![(subs[0] > subs[1]) as usize]},
        6 => State {ver, bits, subs: vec![(subs[0] < subs[1]) as usize]},
        7 => State {ver, bits, subs: vec![(subs[0] == subs[1]) as usize]},
        _ => State {ver, bits, subs},
    }
}

fn num(b: &[u8]) -> usize {
    b.iter()
        .enumerate()
        .map(|(i, &x)| (x as usize) << (b.len() - i - 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_sum() {
        assert_eq!(883, eval_version())
    }

    #[test]
    fn eval() {
        assert_eq!(1675198555015, eval_result())
    }
}
