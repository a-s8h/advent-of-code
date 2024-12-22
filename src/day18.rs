use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn neighbors(&self, max_coord: i32) -> Vec<Point> {
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        dirs.iter()
            .map(|(dx, dy)| Point::new(self.x + dx, self.y + dy))
            .filter(|p| p.x >= 0 && p.x <= max_coord && p.y >= 0 && p.y <= max_coord)
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            Point::new(x, y)
        })
        .collect()
}

fn find_shortest_path(corrupted: &HashSet<Point>, max_coord: i32) -> Option<usize> {
    let start = Point::new(0, 0);
    let target = Point::new(max_coord, max_coord);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((pos, steps)) = queue.pop_front() {
        if pos == target {
            return Some(steps);
        }

        for next in pos.neighbors(max_coord) {
            if !corrupted.contains(&next) && !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, steps + 1));
            }
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    let points = parse_input(input);
    let corrupted: HashSet<_> = points.iter().take(1024).copied().collect();
    find_shortest_path(&corrupted, 70).expect("No path found")
}

pub fn part2(input: &str) -> Point {
    let points = parse_input(input);
    let max_coord = 70;

    for (i, &point) in points.iter().enumerate() {
        let corrupted: HashSet<_> = points.iter().take(i + 1).copied().collect();
        if find_shortest_path(&corrupted, max_coord).is_none() {
            return point;
        }
    }
    unreachable!("Solution must exist")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day18.txt").unwrap();
        assert_eq!(part1(&input), 264);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day18.txt").unwrap();
        assert_eq!(part2(&input), Point { x: 41, y: 26 });
    }
}
