use std::{collections::HashSet, io::Empty, isize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Object,
    LeftBox,
    RightBox,
    Robot,
}

struct Warehouse {
    width: usize,
    height: usize,
    robot: Position,
    grid: Vec<Vec<Tile>>,
}

impl Warehouse {
    fn new(map: &str) -> Self {
        let mut robot = Position { x: 0, y: 0 };
        let mut grid = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in map.lines().enumerate() {
            height += 1;
            width = line.len();
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let pos = Position { x, y };
                let tile = match ch {
                    '@' => {
                        robot = pos;
                        Tile::Robot
                    }
                    'O' => Tile::Object,
                    '#' => Tile::Wall,
                    _ => Tile::Empty,
                };
                row.push(tile);
            }
            grid.push(row);
        }

        Warehouse {
            width,
            height,
            robot,
            grid,
        }
    }

    fn move_robot(&mut self, direction: char) {
        let (dx, dy) = match direction {
            '^' => (0, -1 as isize),
            'v' => (0, 1),
            '<' => (-1 as isize, 0),
            '>' => (1, 0),
            _ => (0, 0),
        };
        let new_pos = Position {
            x: (self.robot.x as isize + dx) as usize,
            y: (self.robot.y as isize + dy) as usize,
        };
        if self.can_move(new_pos, dx, dy) {
            self.move_tile(self.robot, dx, dy);
            self.robot = new_pos;
        }
    }

    fn can_move(&self, new_pos: Position, dx: isize, dy: isize) -> bool {
        let next_pos = Position {
            x: (new_pos.x as isize + dx) as usize,
            y: (new_pos.y as isize + dy) as usize,
        };
        match self.grid[new_pos.y][new_pos.x] {
            Tile::Wall => false,
            Tile::Object => self.can_move(next_pos, dx, dy),
            Tile::LeftBox => {
                if dx == -1 {
                    self.can_move(new_pos, dx, dy)
                } else if dx == 1 {
                    self.can_move(Position { x: next_pos.x + 1, y: next_pos.y }, dx, dy)
                } else {
                    self.can_move(next_pos, dx, dy) && self.can_move(Position { x: next_pos.x + 1, y: next_pos.y }, dx, dy)
                }
            }
            Tile::RightBox => {
                if dx == -1 {
                    self.can_move(Position { x: next_pos.x - 1, y: next_pos.y }, dx, dy)
                } else if dx == 1 {
                    self.can_move(next_pos, dx, dy)
                } else {
                    self.can_move(Position { x: next_pos.x - 1, y: next_pos.y }, dx, dy) && self.can_move(next_pos, dx, dy)
                }
            }
            _ => true,
        }
    }

    fn move_tile(&mut self, pos: Position, dx: isize, dy: isize) -> () {
        let next_pos = Position {
            x: (pos.x as isize + dx) as usize,
            y: (pos.y as isize + dy) as usize,
        };
        match self.grid[next_pos.y][next_pos.x] {
            Tile::Wall | Tile::Robot => { return; }
            Tile::Empty => {},
            Tile::Object => {
                self.move_tile(next_pos, dx, dy);
            },
            Tile::LeftBox => {
                if dx == -1 {
                    self.move_tile(next_pos, dx, dy);
                } else if dx == 1 {
                    self.move_tile( Position { x: next_pos.x + 1, y: next_pos.y }, dx, dy);
                    self.grid[next_pos.y][next_pos.x + 1] = self.grid[next_pos.y][next_pos.x];
                    self.grid[next_pos.y][next_pos.x] = Tile::Empty;
                } else {
                    self.move_tile(next_pos, dx, dy);
                    self.move_tile(Position { x: next_pos.x + 1, y: next_pos.y }, dx, dy);
                    self.grid[next_pos.y][next_pos.x + 1] = self.grid[next_pos.y][next_pos.x];
                    self.grid[next_pos.y][next_pos.x] = Tile::Empty;
                }
            },
            Tile::RightBox => {
                if dx == -1 {
                    self.move_tile(Position { x: next_pos.x - 1, y: next_pos.y }, dx, dy);
                    self.grid[next_pos.y][next_pos.x - 1] = self.grid[next_pos.y][next_pos.x];
                    self.grid[next_pos.y][next_pos.x] = Tile::Empty;
                } else if dx == 1 {
                    self.move_tile(next_pos, dx, dy);
                } else {
                    self.move_tile(Position { x: next_pos.x - 1, y: next_pos.y }, dx, dy);
                    self.move_tile(next_pos, dx, dy);
                    self.grid[next_pos.y][next_pos.x - 1] = self.grid[next_pos.y][next_pos.x];
                    self.grid[next_pos.y][next_pos.x] = Tile::Empty;
                }
            },
        }
        self.grid[next_pos.y][next_pos.x] = self.grid[pos.y][pos.x];
        self.grid[pos.y][pos.x] = Tile::Empty;
    }

    fn simulate(&mut self, moves: &str) {
        for direction in moves.chars() {
            self.move_robot(direction);
        }
    }

    fn calculate_gps_sum(&self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == Tile::LeftBox || self.grid[y][x] == Tile::Object {
                    score += 100 * y + x;
                }
            }
        }
        score
    }

    fn scale_width(&mut self) {
        let mut new_grid = vec![vec![Tile::Empty; self.width * 2]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.grid[y][x];
                new_grid[y][x * 2 + 1] = match tile {
                    Tile::Wall => {
                        new_grid[y][x * 2] = Tile::Wall;
                        Tile::Wall
                    }
                    Tile::Object => {
                        new_grid[y][x * 2] = Tile::LeftBox;
                        Tile::RightBox
                    }
                    _ => Tile::Empty,
                };
                if tile == Tile::Robot {
                    self.robot = Position { x: x * 2, y };
                    new_grid[y][x * 2] = Tile::Robot;
                }
            }
        }
        self.width *= 2;
        self.grid = new_grid;
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
    fn test_part1() {
        let input = std::fs::read_to_string("input/day15.txt").unwrap();
        let (mut warehouse, moves) = parse_input(&input);
        warehouse.simulate(moves);
        let gps_sum = warehouse.calculate_gps_sum();
        assert_eq!(gps_sum, 1478649);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day15.txt").unwrap();
        let (mut warehouse, moves) = parse_input(&input);
        warehouse.scale_width();
        warehouse.simulate(moves);
        let gps_sum = warehouse.calculate_gps_sum();
        assert_eq!(gps_sum, 1495455);
    }

    #[test]
    fn test_warehouse_short() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let (mut warehouse, moves) = parse_input(input);
        warehouse.scale_width();
        warehouse.simulate(moves);
        let gps_sum = warehouse.calculate_gps_sum();
        assert_eq!(gps_sum, 618);
    }

    #[test]
    fn test_warehouse_example() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let (mut warehouse, moves) = parse_input(input);
        warehouse.scale_width();
        warehouse.simulate(moves);
        let gps_sum = warehouse.calculate_gps_sum();
        assert_eq!(gps_sum, 9021);
    }

    #[test]
    fn another_example() {
        let input = "#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^";
        let (mut warehouse, moves) = parse_input(input);
        warehouse.scale_width();
        warehouse.simulate(moves);
        let gps_sum = warehouse.calculate_gps_sum();
        assert_eq!(gps_sum, 509);
    }
}
