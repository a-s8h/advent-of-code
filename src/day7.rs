struct Equation {
    result: u64,
    factors: Vec<u64>,
}

impl Equation {
    fn new(result: u64, factors: Vec<u64>) -> Self {
        Self { result, factors }
    }

    fn from_str(s: &str) -> Self {
        let mut parts = s.trim().split(":");
        let result = parts.next().unwrap().parse::<u64>().unwrap();
        let factors = parts.next().unwrap().trim().split(" ").map(|p| p.parse::<u64>().unwrap()).collect();
        Self::new(result, factors)
    }

    fn is_solvable(&self) -> bool {
        fn rec(result: u64, factors: &[u64]) -> bool {
            if factors.len() == 1 {
                return result == factors[0];
            }

            let (&last, rest) = factors.split_last().unwrap();
            if result % last == 0 && rec(result / last, rest) {
                return true
            }
            if result > last && rec(result - last, rest) {
                return true
            }
            false
        }

        rec(self.result, &self.factors)
    }

    fn is_solvable_2(&self) -> bool {
        fn rec(result: u64, factors: &[u64]) -> bool {
            if factors.len() == 1 {
                return result == factors[0];
            }

            let (&last, rest) = factors.split_last().unwrap();
            if result % last == 0 && rec(result / last, rest) {
                return true
            }
            if result > last && rec(result - last, rest) {
                return true
            }

            let last_len = last.ilog10() + 1;
            let magnitude = 10u64.pow(last_len);
            let target_len = result.ilog10() + 1;
            let ending = result % magnitude;
            if target_len > last_len && last == ending && rec(result / magnitude, rest) {
                return true;
            }
            false
        }

        rec(self.result, &self.factors)
    }
}

fn part1(input: &str) -> u64 {
    input.lines()
        .map(|l| Equation::from_str(l))
        .filter(|e| e.is_solvable())
        .map(|e| e.result)
        .sum()
}

fn part2(input: &str) -> u64 {
    input.lines()
        .map(|l| Equation::from_str(l))
        .filter(|e| e.is_solvable_2())
        .map(|e| e.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day7.txt").unwrap();
        assert_eq!(part1(&input), 6392012777720);
    }
    
    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day7.txt").unwrap();
        assert_eq!(part2(&input), 61561126043536);
    }

    #[test]
    fn test() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part1(input), 3749);
    }
}