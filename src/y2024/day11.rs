use std::collections::HashMap;

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new = HashMap::with_capacity(stones.len() * 2);
    for (&stone, &amount) in stones {
        if stone == 0 {
            *new.entry(1).or_default() += amount;
            continue;
        }
        
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let magnitude = 10u64.pow(digits / 2);
            let lower = stone % magnitude;
            let upper = stone / magnitude;
            *new.entry(lower).or_default() += amount;
            *new.entry(upper).or_default() += amount;
        } else {
            *new.entry(stone * 2024).or_default() += amount;
        }
    }
    new
}

fn solve(input: &str, times: u8) -> usize {
    let mut stones = HashMap::new();
    for n in input.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()) {
        *stones.entry(n).or_default() += 1;
    }
    for _ in 0..times {
        stones = blink(&stones);
    }
    stones.values().sum::<u64>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = std::fs::read_to_string("input/day11.txt").unwrap();
        assert_eq!(solve(&input, 25), 235850);
    }

    #[test]
    fn test_part_2() {
        let input = std::fs::read_to_string("input/day11.txt").unwrap();
        assert_eq!(solve(&input, 75), 279903140844645);
    }
}