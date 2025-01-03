use std::collections::HashSet;
use crate::utils::grid::{Point, parse_grid};
use crate::utils::bfs::get_distances;

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    start: Point,
    end: Point,
}

impl Map {
    fn parse(input: &str) -> Self {
        let grid = parse_grid(input);
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                match grid[y][x] {
                    'S' => start = Point::new(x as i32, y as i32),
                    'E' => end = Point::new(x as i32, y as i32),
                    _ => continue,
                }
            }
        }
        
        Map { grid, start, end }
    }

    fn is_valid(&self, pos: &Point) -> bool {
        pos.x >= 0 && pos.y >= 0 && 
        pos.x < self.grid[0].len() as i32 && 
        pos.y < self.grid.len() as i32
    }

    fn is_wall(&self, pos: &Point) -> bool {
        self.grid[pos.y as usize][pos.x as usize] == '#'
    }

    fn walkable_positions(&self) -> Vec<(Point, i32)> {
        get_distances(
            self.start,
            |pos| pos.neighbors()
                .into_iter()
                .filter(|p| self.is_valid(p) && !self.is_wall(p))
                .collect()
        )
        .into_iter()
        .map(|(pos, dist)| (pos, dist as i32))
        .collect()
    }

    fn count_cheats_with_savings(&self, min_savings: i32, max_chat_length: i32) -> usize {
        let mut unique_cheats = 0;
        let walkable = self.walkable_positions();

        for (pos1, dist1) in &walkable {
            for (pos2, dist2) in &walkable {
                if pos1 == pos2 { continue; }
                let manhattan_dist = (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs();
                if manhattan_dist <= max_chat_length {
                    if dist1 - dist2 - manhattan_dist >= min_savings {
                        unique_cheats += 1;
                    }
                }
            }
        }
        unique_cheats
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_example() {
        let map = Map::parse(EXAMPLE);        
        // Count cheats that save at least 100 picoseconds
        // In the example, no cheats save 100+ picoseconds, so result should be 0
        assert_eq!(map.count_cheats_with_savings(100, 2), 0);
        
        // Additional test to verify some known cheat savings from the example
        assert_eq!(map.count_cheats_with_savings(64, 2), 1); // The 64-picosecond cheat
        assert_eq!(map.count_cheats_with_savings(20, 2), 5); // The 20-picosecond cheat
        assert_eq!(map.count_cheats_with_savings(12, 2), 8); // The three 12-picosecond cheats

        // part 2 example
        assert_eq!(map.count_cheats_with_savings(76, 50), 3);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/day20.txt").expect("Input file should exist");
        let map = Map::parse(&input);
        println!("Day 20 - Part 1 - Cheats saving ≥100 picoseconds: {}", 
                map.count_cheats_with_savings(100, 2));
        println!("Day 20 - Part 2 - Cheats saving ≥100 picoseconds: {} and chat length <= 20", 
                map.count_cheats_with_savings(100, 20));
    }
}
