use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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
    fn all() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }

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

    fn move_from(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = pos;
        match self {
            Direction::North if y > 0 => Some((x, y - 1)),
            Direction::East => Some((x + 1, y)),
            Direction::South => Some((x, y + 1)),
            Direction::West if x > 0 => Some((x - 1, y)),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    direction: Direction,
    score: usize,
}

impl State {
    fn new(x: usize, y: usize, direction: Direction, score: usize) -> Self {
        Self {
            pos: (x, y),
            direction,
            score,
        }
    }
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

type ScoreMatrix = Vec<Vec<HashMap<Direction, usize>>>;

fn find_lowest_score(maze: &[Vec<char>]) -> (usize, ScoreMatrix) {
    let height = maze.len();
    let width = maze[0].len();
    let mut score_matrix = vec![vec![HashMap::with_capacity(4); width]; height];
    
    let (start, end) = find_start_end(maze);
    let mut best_end_score = usize::MAX;
    
    let mut heap = BinaryHeap::new();
    let initial_state = State::new(start.0, start.1, Direction::East, 0);
    heap.push(initial_state);
    score_matrix[start.1][start.0].insert(Direction::East, 0);

    while let Some(state) = heap.pop() {
        if state.pos == end {
            best_end_score = best_end_score.min(state.score);
            continue;
        }

        for next_move in get_valid_moves(state, width, height, maze, &score_matrix) {
            score_matrix[next_move.pos.1][next_move.pos.0]
                .insert(next_move.direction, next_move.score);
            heap.push(next_move);
        }
    }

    (best_end_score, score_matrix)
}

fn get_valid_moves(
    state: State,
    width: usize,
    height: usize,
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
        if let Some((new_x, new_y)) = dir.move_from(state.pos) {
            if new_x >= width || new_y >= height || maze[new_y][new_x] == '#' {
                continue;
            }

            let new_score = state.score + if dir != state.direction {
                TURN_COST + MOVE_COST
            } else {
                MOVE_COST
            };

            if !score_matrix[new_y][new_x].contains_key(&dir)
                || new_score < score_matrix[new_y][new_x][&dir]
            {
                moves.push(State::new(new_x, new_y, dir, new_score));
            }
        }
    }
    moves
}

fn find_start_end(maze: &[Vec<char>]) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                'S' => start = (x, y),
                'E' => end = (x, y),
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
    let (&start_dir, &min_score) = score_matrix[end.1][end.0]
        .iter()
        .min_by_key(|(_, &score)| score)
        .unwrap();
    
    queue.push_back((end, min_score, start_dir));
    visited.insert(end);
    
    while let Some((pos, score, dir)) = queue.pop_front() {
        let back_dir = dir.opposite();
        if let Some(neighbour_pos) = back_dir.move_from(pos) {
            if neighbour_pos.0 >= maze[0].len() || neighbour_pos.1 >= maze.len() 
                || maze[neighbour_pos.1][neighbour_pos.0] == '#' 
                || visited.contains(&neighbour_pos) {
                continue;
            }

            for (&prev_dir, &prev_score) in &score_matrix[neighbour_pos.1][neighbour_pos.0] {
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
