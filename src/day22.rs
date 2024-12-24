use itertools::{iterate, Itertools};

/// The mask for 24 bits (2^24 - 1)
const MASK: i64 = (1 << 24) - 1;
/// Number of iterations for part 1
const ITERATIONS: usize = 2000;
/// Range of possible price changes
const PRICE_RANGE: i64 = 9;
/// Base for indexing price changes (PRICE_RANGE * 2 + 1)
const BASE: i64 = 19;

/// Calculates the next secret number using the specified algorithm
/// 
/// The algorithm performs three operations:
/// 1. Left shift by 6 and XOR
/// 2. Right shift by 5 and XOR
/// 3. Left shift by 11 and XOR
/// Each step is masked to 24 bits
#[inline]
fn step(secret: &i64) -> i64 {
    let mut secret = (secret ^ secret << 6) & MASK;
    secret ^= secret >> 5;
    (secret ^ secret << 11) & MASK
}

/// Converts a sequence of price changes into a unique index
/// 
/// Each change can be in range -9..=9, so we add 9 to make it 0..=18
/// and use base-19 encoding for the sequence
#[inline]
fn sequence_to_index(a: i64, b: i64, c: i64, d: i64) -> usize {
    (BASE.pow(3) * (a + PRICE_RANGE) + 
     BASE.pow(2) * (b + PRICE_RANGE) + 
     BASE * (c + PRICE_RANGE) + 
     (d + PRICE_RANGE)) as usize
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|p| p.parse::<i64>().unwrap())
        .map(|initial| iterate(initial, step).nth(ITERATIONS).unwrap())
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let sequence_count = BASE.pow(4) as usize;
    let mut sums = vec![0; sequence_count];
    let mut last_seen = vec![0; sequence_count];
    
    // Process each initial number
    for (line, buyer_id) in input.lines().zip(1..) {
        let initial = line.parse::<i64>().unwrap();
        
        // Generate price sequence and look for patterns
        iterate(initial, step)
            .take(ITERATIONS + 1)
            .map(|n| n % 10)
            .tuple_windows()
            .for_each(|(e, d, c, b, a)| {
                let idx = sequence_to_index(d - e, c - d, b - c, a - b);
                if last_seen[idx] != buyer_id {
                    last_seen[idx] = buyer_id;
                    sums[idx] += a;
                }
            });
    }
    
    sums.into_iter().max().unwrap()
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

