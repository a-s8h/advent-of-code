use itertools::Itertools;

fn parse_two_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines()
    .filter_map(|line| line.split_whitespace().next_tuple())
    .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
    .unzip()
}

fn part1(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    list1.iter().sorted().zip(list2.iter().sorted())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    let mut right_list_counts = std::collections::HashMap::<u32, u32>::new();
    for num in list2.iter() {
        *right_list_counts.entry(*num).or_default() += 1;
    }

    list1.iter()
        .map(|&num| *right_list_counts.entry(num).or_default() * num)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("input/day1.txt").unwrap();
        let (list1, list2) = parse_two_lists(&input);
        assert_eq!(part1(list1, list2), 2430334);
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("input/day1.txt").unwrap();
        let (list1, list2) = parse_two_lists(&input);
        assert_eq!(part2(list1, list2), 28786472);
    }
}
