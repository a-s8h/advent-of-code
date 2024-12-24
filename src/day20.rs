use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos(self.0 + 1, self.1),
            Pos(self.0 - 1, self.1),
            Pos(self.0, self.1 + 1),
            Pos(self.0, self.1 - 1),
        ]
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut start = Pos(0, 0);
        let mut end = Pos(0, 0);
        
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'S' { start = Pos(x as i32, y as i32); }
                if cell == 'E' { end = Pos(x as i32, y as i32); }
            }
        }
        
        Map { grid, start, end }
    }


    fn is_valid(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && 
        pos.0 < self.grid[0].len() as i32 && 
        pos.1 < self.grid.len() as i32
    }

    fn is_wall(&self, pos: &Pos) -> bool {
        self.grid[pos.1 as usize][pos.0 as usize] == '#'
    }

    fn walkable_positions(&self) -> Vec<(Pos, i32)> {
        let mut scores = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back((self.start, 0));
        visited.insert(self.start);
        scores.push((self.start, 0));
    
        while let Some((pos, dist)) = queue.pop_front() {
            for next in pos.neighbors() {
                if self.is_valid(&next) && 
                   !self.is_wall(&next) && 
                   !visited.contains(&next) {
                    visited.insert(next);
                    scores.push((next, dist + 1));
                    queue.push_back((next, dist + 1));
                }
            }
        }
        scores
    }

    fn count_cheats_with_savings(&self, min_savings: i32, max_chat_length: i32) -> usize {
        let mut unique_cheats = 0;
        let walkable = self.walkable_positions();

        // Check all pairs of walkable positions that are distance 2 apart
        for (pos1, dist1) in &walkable {
            for (pos2, dist2) in &walkable {
                if pos1 == pos2 { continue; }
                let manhatten_dist = (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs();
                // Check if positions are exactly 2 steps apart (Manhattan distance)
                if manhatten_dist <= max_chat_length {
                    // work only for 2 steps, but should chat paths contain only walls?
                    // // The wall position would be between these two positions
                    // let wall_x = (pos1.0 + pos2.0) / 2;
                    // let wall_y = (pos1.1 + pos2.1) / 2;
                    // let wall_pos = Pos(wall_x, wall_y);
                    // if self.is_valid(&wall_pos) && self.is_wall(&wall_pos) {
                    //     if dist1 - dist2 - manhatten_dist >= min_savings {
                    //         unique_cheats += 1;
                    //     }
                    // }
                    if dist1 - dist2 - manhatten_dist >= min_savings {
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
