use std::collections::{HashMap, HashSet, VecDeque};
use crate::utils::grid::Point;
use crate::utils::bfs;

// Cost constants
const TURN_COST: usize = 1000;
const MOVE_COST: usize = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

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

    fn move_from(&self, pos: Point) -> Option<Point> {
        let Point { x, y } = pos;
        match self {
            Direction::North if y > 0 => Some(Point::new(x, y - 1)),
            Direction::East => Some(Point::new(x + 1, y)),
            Direction::South => Some(Point::new(x, y + 1)),
            Direction::West if x > 0 => Some(Point::new(x - 1, y)),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: Point,
    direction: Direction,
    score: usize,
}

impl State {
    fn new(p: Point, direction: Direction, score: usize) -> Self {
        Self {
            pos: p,
            direction,
            score,
        }
    }
}

type ScoreMatrix = Vec<Vec<HashMap<Direction, usize>>>;

fn get_valid_moves(
    state: State,
    width: i32,
    height: i32,
    maze: &[Vec<char>],
    score_matrix: &ScoreMatrix,
) -> Vec<State> {
    let mut moves = Vec::with_capacity(3);
    let possible_directions = [
        state.direction,
        state.direction.turn_left(),
        state.direction.turn_right(),
    ];

    for &dir in &possible_directions {
        if let Some(Point { x, y }) = dir.move_from(state.pos) {
            if x >= width || y >= height || maze[y as usize][x as usize] == '#' {
                continue;
            }

            let new_score = state.score + if dir != state.direction {
                TURN_COST + MOVE_COST
            } else {
                MOVE_COST
            };

            if !score_matrix[y as usize][x as usize].contains_key(&dir)
                || new_score < score_matrix[y as usize][x as usize][&dir]
            {
                moves.push(State::new(Point::new(x, y), dir, new_score));
            }
        }
    }
    moves
}

fn find_lowest_score(maze: &[Vec<char>]) -> (usize, ScoreMatrix) {
    let height = maze.len();
    let width = maze[0].len();
    let mut score_matrix = vec![vec![HashMap::with_capacity(4); width]; height];
    
    let (start, end) = find_start_end(maze);
    let mut best_end_score = usize::MAX;
    
    let mut queue = VecDeque::new();
    let initial_state = State::new(start, Direction::East, 0);
    queue.push_back(initial_state);
    score_matrix[start.y as usize][start.x as usize].insert(Direction::East, 0);

    while let Some(state) = queue.pop_front() {
        if state.pos == end {
            best_end_score = best_end_score.min(state.score);
            continue;
        }

        for next_move in get_valid_moves(state, width as i32, height as i32, maze, &score_matrix) {
            score_matrix[next_move.pos.y as usize][next_move.pos.x as usize]
                .insert(next_move.direction, next_move.score);
            queue.push_back(next_move);
        }
    }

    (best_end_score, score_matrix)
}

fn find_start_end(maze: &[Vec<char>]) -> (Point, Point) {
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                'S' => start = Point::new(x as i32, y as i32),
                'E' => end = Point::new(x as i32, y as i32),
                _ => continue,
            }
        }
    }
    (start, end)
}

fn count_shortest_path_tiles(maze: &[Vec<char>], score_matrix: &ScoreMatrix) -> usize {
    let (_, end) = find_start_end(maze);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    // Find minimum score at end position
    let (&start_dir, &min_score) = score_matrix[end.y as usize][end.x as usize]
        .iter()
        .min_by_key(|(_, &score)| score)
        .unwrap();
    
    queue.push_back((end, min_score, start_dir));
    visited.insert(end);
    
    while let Some((pos, score, dir)) = queue.pop_front() {
        let back_dir = dir.opposite();
        if let Some(neighbour_pos) = back_dir.move_from(pos) {
            if neighbour_pos.x as usize >= maze[0].len() || neighbour_pos.y as usize >= maze.len() 
                || maze[neighbour_pos.y as usize][neighbour_pos.x as usize] == '#' 
                || visited.contains(&neighbour_pos) {
                continue;
            }

            for (&prev_dir, &prev_score) in &score_matrix[neighbour_pos.y as usize][neighbour_pos.x as usize] {
                if (score == prev_score + MOVE_COST && prev_dir == dir)
                    || (prev_dir != dir && score == prev_score + TURN_COST + MOVE_COST)
                {
                    queue.push_back((neighbour_pos, prev_score, prev_dir));
                    visited.insert(neighbour_pos);
                }
            }
        }
    }
    
    visited.len()
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
        let (score, matrix) = find_lowest_score(&maze);
        assert_eq!(score, 7036);
        assert_eq!(count_shortest_path_tiles(&maze, &matrix), 45);
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
        let (score, matrix) = find_lowest_score(&maze);
        assert_eq!(score, 11048);
        assert_eq!(count_shortest_path_tiles(&maze, &matrix), 64);
    }

    #[test]
    fn run_on_input() {
        let input = std::fs::read_to_string("input/day16.txt").unwrap();
        let maze = parse_input(&input);
        let (score, matrix) = find_lowest_score(&maze);
        println!("Best score to end: {}", score);
        let path_tiles = count_shortest_path_tiles(&maze, &matrix);
        println!("Unique tiles in shortest paths: {}", path_tiles);
    }
}
