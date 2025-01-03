static DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (1, 1),
    (1, -1),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
];

static XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

use crate::utils::grid::parse_grid;

fn xmas_count(grid: Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                for d in DIRECTIONS.iter() {
                    if check_word(&grid, i as i32, j as i32, d) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn check_word(grid: &Vec<Vec<char>>, mut x: i32, mut y: i32, direction: &(i32, i32)) -> bool {
    for c in XMAS.iter() {
        if x < 0
            || y < 0
            || x >= grid.len() as i32
            || y >= grid[0].len() as i32
            || grid[x as usize][y as usize] != *c
        {
            return false;
        }

        x += direction.0;
        y += direction.1;
    }
    true
}

fn x_mas_count(grid: &Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;
    for (x, line) in grid.iter().enumerate() {
        for (y, char) in line.iter().enumerate() {
            if *char == 'A' && y > 0 && y + 1 < line.len() && x > 0 && x + 1 < grid.len() {
                let chars: Vec<char> = vec![
                    grid[x - 1][y - 1],
                    grid[x - 1][y + 1],
                    grid[x + 1][y + 1],
                    grid[x + 1][y - 1],
                ];
                if chars == ['M', 'M', 'S', 'S']
                    || chars == ['M', 'S', 'S', 'M']
                    || chars == ['S', 'S', 'M', 'M']
                    || chars == ['S', 'M', 'M', 'S']
                {
                    count += 1
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas_count() {
        assert_eq!(xmas_count(parse_grid(&std::fs::read_to_string("input/day4.txt").unwrap())), 2336);
    }

    #[test]
    fn test_x_mas_count() {
        assert_eq!(x_mas_count(&parse_grid(&std::fs::read_to_string("input/day4.txt").unwrap())), 1831);
    }
}
