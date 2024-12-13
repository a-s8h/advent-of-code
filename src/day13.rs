const COST_MOVE_A: usize = 3;
const COST_MOVE_B: usize = 1;

struct Machine {
    move_a: Position,
    move_b: Position,
    target: Position,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Machine {
    fn min_cost_to_win(&self, target_offset: isize) -> Option<usize> {
        let target_x = self.target.x + target_offset;
        let target_y = self.target.y + target_offset;
        
        // Calculate coefficients for the linear system
        let numerator = self.move_b.x * target_y - self.move_b.y * target_x;
        let denominator = self.move_a.y * self.move_b.x - self.move_a.x * self.move_b.y;
        
        // Check if solution exists
        if numerator % denominator != 0 {
            return None;
        }

        let moves_a = numerator / denominator;
        let remainder_x = target_x - (moves_a * self.move_a.x);
        
        if remainder_x % self.move_b.x != 0 {
            return None;
        }

        let moves_b = remainder_x / self.move_b.x;
        debug_assert_eq!(target_y, moves_a * self.move_a.y + moves_b * self.move_b.y);
        
        Some(moves_a as usize * COST_MOVE_A + moves_b as usize * COST_MOVE_B)
    }
}

fn parse_coordinates(s: &str) -> (&str, &str) {
    s.split_once(": ").unwrap().1.split_once(", ").unwrap()
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.trim().lines();
            let (x1, y1) = parse_coordinates(lines.next().unwrap());
            let (x2, y2) = parse_coordinates(lines.next().unwrap());
            let (x3, y3) = parse_coordinates(lines.next().unwrap());
            
            let [a_x, a_y, b_x, b_y, target_x, target_y] = 
                [x1, y1, x2, y2, x3, y3].map(|s| s[2..].parse::<isize>().unwrap());
            
            Machine {
                move_a: Position { x: a_x, y: a_y },
                move_b: Position { x: b_x, y: b_y },
                target: Position { x: target_x, y: target_y },
            }
        })
        .collect()
}

fn part_1(input: &[Machine]) -> usize {
    input
        .iter()
        .filter_map(|machine| machine.min_cost_to_win(0))
        .sum()
}

fn part_2(input: &[Machine]) -> usize {
    input
        .iter()
        .filter_map(|machine| machine.min_cost_to_win(10000000000000))
        .sum()
}


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("input/day13.txt").unwrap();
        let machines = parse(&input);
        assert_eq!(part_1(&machines), 36571);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("input/day13.txt").unwrap();
        let machines = parse(&input);
        assert_eq!(part_2(&machines), 85527711500010);
    }
}