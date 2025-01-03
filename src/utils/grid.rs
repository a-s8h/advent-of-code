use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> Vec<Point> {
        CARDINAL_DIRECTIONS.iter()
            .map(|(dx, dy)| Point::new(self.x + dx, self.y + dy))
            .collect()
    }

    pub fn neighbors_with_bounds(&self, max_x: i32, max_y: i32) -> Vec<Point> {
        self.neighbors()
            .into_iter()
            .filter(|p| p.x >= 0 && p.x < max_x && p.y >= 0 && p.y < max_y)
            .collect()
    }
}

/// Common 8-directional movement vectors for grid traversal
pub const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // right
    (1, 0),   // down
    (1, 1),   // down-right
    (1, -1),  // down-left
    (0, -1),  // left
    (-1, 0),  // up
    (-1, -1), // up-left
    (-1, 1),  // up-right
];

/// Common 4-directional movement vectors (cardinal directions)
pub const CARDINAL_DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),   // right
    (1, 0),   // down
    (0, -1),  // left
    (-1, 0),  // up
];

/// Check if coordinates are within grid bounds
pub fn is_in_bounds<T>(x: i32, y: i32, grid: &[Vec<T>]) -> bool {
    x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len()
}

/// Parse a grid from string input where each line represents a row
pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

/// Find positions of specific elements in a grid
pub fn find_positions<T: PartialEq>(grid: &[Vec<T>], target: &T) -> Vec<Point> {
    let mut positions = Vec::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if cell == target {
                positions.push(Point::new(x as i32, y as i32));
            }
        }
    }
    positions
}

/// Print a grid for debugging
pub fn print_grid<T: Debug>(grid: &[Vec<T>]) {
    for row in grid {
        println!("{:?}", row);
    }
}
