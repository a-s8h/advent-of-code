use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct TrieNode {
    is_end: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, pattern: &str) {
        let mut current = self;
        for c in pattern.chars() {
            current = current.children.entry(c).or_default();
        }
        current.is_end = true;
    }
}

fn parse_input(input: &str) -> (TrieNode, Vec<String>) {
    let mut parts = input.split("\n\n");
    let mut trie = TrieNode::new();

    // Insert patterns into trie
    parts
        .next()
        .unwrap()
        .split(", ")
        .for_each(|pattern| trie.insert(pattern));

    let designs = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    (trie, designs)
}

fn can_make_pattern_with_cache(target: &str, trie: &TrieNode, cache: &mut HashSet<String>) -> bool {
    if target.is_empty() {
        return true;
    }

    if cache.contains(target) {
        return false;
    }

    let mut current = trie;
    let chars: Vec<char> = target.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if let Some(next) = current.children.get(&c) {
            if next.is_end && can_make_pattern_with_cache(&target[i + 1..], trie, cache) {
                return true;
            }
            current = next;
        } else {
            break;
        }
    }

    cache.insert(target.to_string());
    false
}

fn count_pattern_ways(target: &str, trie: &TrieNode, cache: &mut HashMap<String, i64>) -> i64 {
    if target.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(target) {
        return count;
    }

    let mut total = 0;
    let mut current = trie;
    let chars: Vec<char> = target.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if let Some(next) = current.children.get(&c) {
            if next.is_end {
                total += count_pattern_ways(&target[i + 1..], trie, cache);
            }
            current = next;
        } else {
            break;
        }
    }

    cache.insert(target.to_string(), total);
    total
}

pub fn part1(input: &str) -> i64 {
    let (trie, designs) = parse_input(input);
    let mut cache = HashSet::new();

    designs
        .iter()
        .filter(|design| can_make_pattern_with_cache(design, &trie, &mut cache))
        .count() as i64
}

pub fn part2(input: &str) -> i64 {
    let (trie, designs) = parse_input(input);
    let mut cache = HashMap::new();

    designs
        .iter()
        .map(|design| count_pattern_ways(design, &trie, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/day19.txt").unwrap();
        assert_eq!(part1(&input), 311);
    }

    #[test]
    fn test_part2() {
        let input =
            "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";
        assert_eq!(part2(input), 16);
    }

    #[test]
    fn test_input_part2() {
        let input = std::fs::read_to_string("input/day19.txt").unwrap();
        assert_eq!(part2(&input), 616234236468263);
    }
}
