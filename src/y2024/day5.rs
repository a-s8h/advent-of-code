use std::{cmp::Ordering, collections::{HashMap, HashSet}};

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut parts = input.split("\n\n");
    
    let rules: Vec<(i32, i32)> = parts
        .next()
        .expect("Missing rules section")
        .lines()
        .map(|line| {
            let mut number_pairs = line.split('|');
            (
                number_pairs.next().expect("Missing first number").parse::<i32>().expect("Invalid first number"),
                number_pairs.next().expect("Missing second number").parse::<i32>().expect("Invalid second number")
            )
        }).collect();

    let print_orders = parts
        .next()
        .expect("Missing print orders section")
        .lines()
        .map(|line| line.split(',').map(|s| s.parse::<i32>().expect("Invalid number")).collect())
        .collect();

    (rules, print_orders)
}

fn is_ordered(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> bool {
    let positions: HashMap<_, _> = update.iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();
    rules.iter()
        .all(|(first, second)| {
            match (positions.get(first), positions.get(second)) {
                (Some(&f), Some(&s)) => f < s,
                _ => true
            }
        })
}

fn part1(input: &str) -> i32 {
    let (rules, orders) = parse_input(input);
    let mut result = 0;
    for o in orders.iter() {
        if is_ordered(&rules, o) {
            result += o[o.len() / 2];
        }
    }
    result
}

fn fix_ordering(rules: &HashMap<i32, HashSet<i32>>, update: &mut Vec<i32>) {
    update.sort_by(|&a, &b| {
        if rules.get(&a).map_or(false, |set| set.contains(&b)) {
            Ordering::Less
        } else if rules.get(&b).map_or(false, |set| set.contains(&a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

fn part2(input: &str) -> i32 {
    let (rules, mut updates) = parse_input(input);
    let rules_map: HashMap<i32, HashSet<i32>> = rules.iter()
        .fold(HashMap::new(), |mut map, &(l, r)| {
            map.entry(l)
                .or_insert(HashSet::new())
                .insert(r);
            map
        });
    updates.iter_mut()
        .filter(|update| !is_ordered(&rules, update))
        .map(|update| {
            fix_ordering(&rules_map, update);
            update[update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day5.txt").unwrap();
        assert_eq!(part1(&input), 4996);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day5.txt").unwrap();
        assert_eq!(part2(&input), 6311);
    }
}
