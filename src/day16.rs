use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn move_forward(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
    score: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_lowest_score(maze: &[Vec<char>]) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in maze.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (j, i);
            } else if cell == 'E' {
                end = (j, i);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State {
        x: start.0,
        y: start.1,
        direction: Direction::East,
        score: 0,
    });
    visited.insert((start.0, start.1));

    while let Some(state) = heap.pop() {
        if (state.x, state.y) == end {
            return state.score;
        }

        for &new_direction in &[
            state.direction,
            state.direction.turn_left(),
            state.direction.turn_right(),
        ] {
            let (new_x, new_y) = new_direction.move_forward(state.x, state.y);
            let new_score = if new_direction != state.direction {
                state.score + 1000 + 1
            } else {
                state.score + 1
            };
            if maze[new_y][new_x] != '#' && !visited.contains(&(new_x, new_y)) {
                heap.push(State {
                    x: new_x,
                    y: new_y,
                    direction: new_direction,
                    score: new_score,
                });
                visited.insert((new_x, new_y));
            }
        }
    }

    usize::MAX
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest_score() {
        let maze = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let maze = parse_input(maze);
        assert_eq!(find_lowest_score(&maze), 7036);
    }

    #[test]
    fn test_find_lowest_score2() {
        let maze = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let maze = parse_input(maze);
        assert_eq!(find_lowest_score(&maze), 11048);
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day16.txt").unwrap();
        let maze = parse_input(&input);
        println!("{}", find_lowest_score(&maze));
    }
}
