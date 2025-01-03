use std::collections::HashSet;

fn count_paths(grid: &Vec<Vec<u32>>, start: (usize, usize), track_visited: bool) -> u32 {
    let mut stack = vec![start];
    let mut visited = if track_visited { HashSet::from([start]) } else { HashSet::new() };
    let mut count = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((y, x)) = stack.pop() {
        if grid[y][x] == 9 {
            count += 1;
            continue;
        }

        for (dy, dx) in directions {
            let new_y = y as i32 + dy;
            let new_x = x as i32 + dx;

            if new_y >= 0 && new_y < grid.len() as i32 
               && new_x >= 0 && new_x < grid[0].len() as i32 {
                let new_y = new_y as usize;
                let new_x = new_x as usize;
                
                if grid[new_y][new_x] == grid[y][x] + 1 
                   && (!track_visited || !visited.contains(&(new_y, new_x))) {
                    stack.push((new_y, new_x));
                    if track_visited {
                        visited.insert((new_y, new_x));
                    }
                }
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                result += count_paths(&grid, (y, x), true);
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                result += count_paths(&grid, (y, x), false);
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("input/day10.txt").unwrap();
        assert_eq!(part_one(&input), Some(733));
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("input/day10.txt").unwrap();
        assert_eq!(part_two(&input), Some(1514));
    }
}