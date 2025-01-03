use std::collections::HashSet;
use crate::utils::grid::Point;
use crate::utils::bfs::find_shortest_path;

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

fn find_path(corrupted: &HashSet<Point>, max_coord: i32) -> Option<usize> {
    let start = Point::new(0, 0);
    let target = Point::new(max_coord, max_coord);
    
    find_shortest_path(
        start,
        &|p: &Point| *p == target,
        &|p: &Point| p.neighbors_with_bounds(max_coord + 1, max_coord + 1)
            .into_iter()
            .filter(|p| !corrupted.contains(p))
            .collect()
    )
}

pub fn part1(input: &str) -> usize {
    let points = parse_input(input);
    let corrupted: HashSet<_> = points.iter().take(1024).copied().collect();
    find_path(&corrupted, 70).expect("No path found")
}

pub fn part2(input: &str) -> Point {
    let points = parse_input(input);
    let max_coord = 70;

    for (i, &point) in points.iter().enumerate() {
        let corrupted: HashSet<_> = points.iter().take(i + 1).copied().collect();
        if find_path(&corrupted, max_coord).is_none() {
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
