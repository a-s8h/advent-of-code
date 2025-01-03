use itertools::{iterate, Itertools};

const MASK: i64 = (1 << 24) - 1;

#[inline]
fn step(secret: &i64) -> i64 {
    let mut secret = (secret ^ secret << 6) & MASK;
    secret ^= secret >> 5;
    (secret ^ secret << 11) & MASK
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(|p| iterate(p.parse::<i64>().unwrap(), step).nth(2000).unwrap()).sum()
}

// Range of -9..=9 possible for each, 19.pow(4) possible values
fn idx(a: i64, b: i64, c: i64, d: i64) -> usize {
    (6859 * (a + 9) + 361 * (b + 9) + 19 * (c + 9) + d + 9) as usize
}

pub fn part2(input: &str) -> i64 {
    let mut dp = vec![0; 19_usize.pow(4)];
    let mut dpi = vec![0; 19_usize.pow(4)];
    for (p, i) in input.lines().zip(1..) {
        for (e, d, c, b, a) in iterate(p.parse::<i64>().unwrap(), step).take(2001).map(|n| n % 10).tuple_windows() {
            let idx = idx(d - e, c - d, b - c, a - b);
            if dpi[idx] != i {
                dpi[idx] = i;
                dp[idx] += a;
            }
        }
    }
    dp.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "1\n10\n100\n2024";
        assert_eq!(part1(input), 37327623);
    }

    #[test]
    fn test_price_sequence() {
        let input = "1\n2\n3\n2024";
        assert_eq!(part2(input), 23);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/day22.txt").expect("Input file should exist");
        assert_eq!(part1(&input), 17965282217);
        assert_eq!(part2(&input), 2152);
    }
}

