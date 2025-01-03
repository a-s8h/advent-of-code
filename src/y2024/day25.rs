use itertools::Itertools;

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for e in input.split("\n\n") {
        let l = e
            .lines()
            .map(|l| l.chars().map(|c| (c == '#') as u8).collect_vec())
            .fold(vec![0, 0, 0, 0, 0], |acc, x| sum(&acc, &x));
        if e.starts_with('#') {
            locks.push(l)
        } else {
            keys.push(l)
        };
    }
    (keys, locks)
}

fn sum(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a + b)
        .collect_vec()
        .try_into()
        .unwrap()
}

fn part1(input: &str) -> usize {
    let (locks, keys) = parse(input);
    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter_map(|(l, k)| sum(l, k).iter().all(|v| *v <= 7).then_some(1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/day25.txt").expect("Input file should exist");
        assert_eq!(part1(&input), 2933);
    }
}
