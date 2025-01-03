use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use super::grid::Point;

/// Generic BFS function for finding shortest path between points
pub fn find_shortest_path<F>(
    start: Point,
    is_target: impl Fn(&Point) -> bool,
    get_neighbors: F,
) -> Option<usize>
where
    F: Fn(&Point) -> Vec<Point>,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((pos, steps)) = queue.pop_front() {
        if is_target(&pos) {
            return Some(steps);
        }

        for next in get_neighbors(&pos) {
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, steps + 1));
            }
        }
    }
    None
}

/// Generic BFS function for finding all reachable positions
pub fn find_reachable_positions<T: Hash + Eq + Copy>(
    start: T,
    get_neighbors: impl Fn(&T) -> Vec<T>,
) -> HashSet<T> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(pos) = queue.pop_front() {
        for next in get_neighbors(&pos) {
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back(next);
            }
        }
    }
    visited
}

/// BFS function that returns distances to all reachable positions
pub fn get_distances<T: Hash + Eq + Copy>(
    start: T,
    get_neighbors: impl Fn(&T) -> Vec<T>,
) -> Vec<(T, usize)> {
    let mut distances = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((pos, dist)) = queue.pop_front() {
        distances.push((pos, dist));
        
        for next in get_neighbors(&pos) {
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, dist + 1));
            }
        }
    }
    distances
}

/// BFS function that allows collecting multiple paths
pub fn find_all_paths<T: Hash + Eq + Copy>(
    start: T,
    is_target: impl Fn(&T) -> bool,
    get_neighbors: impl Fn(&T) -> Vec<T>,
) -> Vec<Vec<T>> {
    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, vec![start]));

    while let Some((pos, path)) = queue.pop_front() {
        if is_target(&pos) {
            paths.push(path);
            continue;
        }

        for next in get_neighbors(&pos) {
            if !path.contains(&next) {
                let mut new_path = path.clone();
                new_path.push(next);
                queue.push_back((next, new_path));
            }
        }
    }
    paths
}
