const WIDTH: usize = 12;
const COUNT: usize = 1000;

pub fn part1() -> usize {
    let bitsum: [usize; WIDTH] = include_str!("input")
        .lines()
        .fold([0; WIDTH], | mut vec, l| {
            for (i, c) in l.char_indices() {
                vec[i] += if c == '1' {1} else {0};
            }
            vec
        });

    let gamma = bitsum
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &value)| {
            let bit = (value > COUNT/2) as usize;
            acc + (bit << i)
        });

    // epsilon == one's complement, masked to first 12 bits
    let epsilon = !gamma & ((1 << WIDTH) - 1);
    gamma * epsilon
}

pub fn part2() -> usize {
    let numbers: Vec<usize> = include_str!("input")
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();

    let is_bit_set = |bit: usize, x: &usize| (x & (1<<bit)) >> bit;
    let most_frequent_bit = |pos: usize, v: &Vec<usize>| {
        let sum: usize = v.iter().map(|number| (number & (1<<pos)) >> pos).sum();
        (sum >= (v.len() + 1)/2) as usize
    };

    let mut bit = WIDTH;
    let mut oxygen = numbers.clone();
    while oxygen.len() > 1 {
        bit -= 1;
        let b = most_frequent_bit(bit, &oxygen);
        oxygen = oxygen
            .into_iter()
            .filter(|x| is_bit_set(bit, x) == b)
            .collect();
    }
    let oxygen = oxygen[0];
    
    let mut bit = WIDTH;
    let mut co2 = numbers;
    while co2.len() > 1 {
        bit -= 1;
        let b = most_frequent_bit(bit, &co2);
        co2 = co2
            .into_iter()
            .filter(|x| is_bit_set(bit, x) != b)
            .collect();
    }
    let co2 = co2[0];

    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_consumption() {
        assert_eq!(3374136, part1())
    }

    #[test]
    fn life_support() {
        assert_eq!(4432698, part2())
    }
}
