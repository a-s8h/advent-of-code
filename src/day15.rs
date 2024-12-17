use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Warehouse {
    width: usize,
    height: usize,
    robot: Position,
    boxes: HashSet<Position>,
    walls: HashSet<Position>,
}

impl Warehouse {
    fn new(map: &str) -> Self {
        let mut robot = Position { x: 0, y: 0 };
        let mut boxes = HashSet::new();
        let mut walls = HashSet::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in map.lines().enumerate() {
            height += 1;
            width = line.len();
            for (x, ch) in line.chars().enumerate() {
                let pos = Position { x, y };
                match ch {
                    '@' => robot = pos,
                    'O' => {
                        boxes.insert(pos);
                    }
                    '#' => {
                        walls.insert(pos);
                    }
                    _ => {}
                }
            }
        }

        Warehouse {
            width,
            height,
            robot,
            boxes,
            walls,
        }
    }

    fn move_robot(&mut self, direction: char) {
        let (dx, dy) = match direction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
        };
        let new_pos = Position {
            x: (self.robot.x as isize + dx) as usize,
            y: (self.robot.y as isize + dy) as usize,
        };
        if self.try_move(new_pos, dx, dy) {
            self.robot = new_pos;
        }
    }

    fn try_move(&mut self, new_pos: Position, dx: isize, dy: isize) -> bool {
        if self.walls.contains(&new_pos) {
            return false;
        }
        if self.boxes.contains(&new_pos) {
            let next_pos = Position {
                x: (new_pos.x as isize + dx) as usize,
                y: (new_pos.y as isize + dy) as usize,
            };
            if self.try_move(next_pos, dx, dy) {
                self.boxes.remove(&new_pos);
                self.boxes.insert(next_pos);
            } else {
                return false;
            }
        }
        true
    }

    fn simulate(&mut self, moves: &str) {
        for direction in moves.chars() {
            self.move_robot(direction);
        }
    }

    fn calculate_gps_sum(&self) -> usize {
        self.boxes.iter().map(|pos| pos.y * 100 + pos.x).sum()
    }
}

fn parse_input(input: &str) -> (Warehouse, &str) {
    let mut parts = input.split("\n\n");
    let map = parts.next().unwrap();
    let moves = parts.next().unwrap();
    (Warehouse::new(map), moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warehouse() {
        let input = std::fs::read_to_string("input/day15.txt").unwrap();
        let (mut warehouse, moves) = parse_input(&input);
        warehouse.simulate(moves);
        let gps_sum = warehouse.calculate_gps_sum();
        assert_eq!(gps_sum, 1478649);
    }
}
