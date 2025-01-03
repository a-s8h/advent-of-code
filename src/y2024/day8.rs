use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_same_freequency_anthenas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut result = HashMap::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c != '.' {
                result.entry(c).or_insert(vec![]).push((x as i32, y as i32));
            }
        }
    }
    result
}

fn check_bounds(p: (i32, i32), grid_len: (i32, i32)) -> bool {
    p.0>= 0 && p.0 < grid_len.0 && p.1 >= 0 && p.1 < grid_len.1
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    let anthenas = find_same_freequency_anthenas(&grid);
    let mut antinodes = HashSet::<(i32, i32)>::new();

    let (x_len, y_len) = (grid.len() as i32, grid[0].len() as i32);

    for (_, positions) in anthenas.iter() {
        for p1 in positions.iter() {
            for p2 in positions.iter() {
                if p1 == p2 {
                    continue;
                }

                let (x_diff, y_diff) = (p1.0 - p2.0, p1.1 - p2.1);
                if check_bounds((p1.0 + x_diff, p1.1 + y_diff), (x_len, y_len)) {
                    antinodes.insert((p1.0 + x_diff, p1.1 + y_diff));
                }

                let (x_diff, y_diff) = (p2.0 - p1.0, p2.1 - p1.1);
                if check_bounds((p2.0 + x_diff, p2.1 + y_diff), (x_len, y_len)) {
                    antinodes.insert((p2.0 + x_diff, p2.1 + y_diff));
                }
            }
        }
    }
    antinodes.len()
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    let anthenas = find_same_freequency_anthenas(&grid);
    let mut antinodes = HashSet::<(i32, i32)>::new();
    let (x_len, y_len) = (grid.len() as i32, grid[0].len() as i32);

    for (_, positions) in anthenas.iter() {
        for p1 in positions.iter() {
            for p2 in positions.iter() {
                if p1 == p2 {
                    continue;
                }

                antinodes.insert(*p1);
                let (x_diff, y_diff) = (p1.0 - p2.0, p1.1 - p2.1);
                let mut l = (p1.0 + x_diff, p1.1 + y_diff);
                while check_bounds( l, (x_len, y_len)) {
                    antinodes.insert(l);
                    l = (l.0 + x_diff, l.1 + y_diff);
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = std::fs::read_to_string("input/day8.txt").unwrap();
        assert_eq!(part_1(&input), 348);
    }

    #[test]
    fn test_part_2() {
        let input = std::fs::read_to_string("input/day8.txt").unwrap();
        assert_eq!(part_2(&input), 1221);
    }
}
