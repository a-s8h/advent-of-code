use std::collections::HashMap;
use crate::utils::grid::Point;
use crate::utils::bfs::find_all_paths;

const BFS_DIRECTIONS: [(char, (i32, i32)); 4] = [
    ('^', (0, -1)),
    ('>', (1, 0)),
    ('v', (0, 1)),
    ('<', (-1, 0)),
];

fn init_keypad() -> HashMap<char, Point> {
    let mut keypad = HashMap::new();
    keypad.insert('7', Point::new(0, 0));
    keypad.insert('8', Point::new(1, 0));
    keypad.insert('9', Point::new(2, 0));
    keypad.insert('4', Point::new(0, 1));
    keypad.insert('5', Point::new(1, 1));
    keypad.insert('6', Point::new(2, 1));
    keypad.insert('1', Point::new(0, 2));
    keypad.insert('2', Point::new(1, 2));
    keypad.insert('3', Point::new(2, 2));
    keypad.insert('X', Point::new(0, 3));
    keypad.insert('0', Point::new(1, 3));
    keypad.insert('A', Point::new(2, 3));
    keypad
}

fn init_directions() -> HashMap<char, Point> {
    let mut directions = HashMap::new();
    directions.insert('X', Point::new(0, 0));
    directions.insert('^', Point::new(1, 0));
    directions.insert('A', Point::new(2, 0));
    directions.insert('<', Point::new(0, 1));
    directions.insert('v', Point::new(1, 1));
    directions.insert('>', Point::new(2, 1));
    directions
}

fn get_command(input: &HashMap<char, Point>, start: char, end: char) -> Vec<String> {
    if start == end {
        return vec!["A".to_string()];
    }

    let start_pos = input[&start];
    let end_pos = input[&end];
    let blank_pos = input[&'X'];

    let paths = find_all_paths(
        start_pos,
        |pos| *pos == end_pos,
        |pos| {
            BFS_DIRECTIONS.iter()
                .map(|(_, (dx, dy))| Point::new(pos.x + dx, pos.y + dy))
                .filter(|new_pos| {
                    *new_pos != blank_pos && 
                    input.values().any(|&button| button == *new_pos)
                })
                .collect()
        }
    );

    let mut all_paths = paths.into_iter()
        .map(|path| {
            let mut command = String::new();
            for window in path.windows(2) {
                let dx = window[1].x - window[0].x;
                let dy = window[1].y - window[0].y;
                let dir = BFS_DIRECTIONS.iter()
                    .find(|(_, (dx2, dy2))| *dx2 == dx && *dy2 == dy)
                    .unwrap().0;
                command.push(dir);
            }
            command + "A"
        })
        .collect::<Vec<_>>();

    all_paths.sort_by_key(|path| path.len());
    all_paths
}

fn get_key_presses(
    input: &HashMap<char, Point>,
    code: &str,
    robot: i64,
    memo: &mut HashMap<(String, i64), i64>,
) -> i64 {
    let key = (code.to_string(), robot);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let mut current = 'A';
    let mut length: i64 = 0;

    for c in code.chars() {
        let moves = get_command(input, current, c);
        if robot == 0 {
            length += moves[0].len() as i64;
        } else {
            length += moves
                .iter()
                .map(|move_str| get_key_presses(&init_directions(), move_str, robot - 1, memo))
                .min()
                .unwrap_or(0);
        }
        current = c;
    }

    memo.insert(key, length);
    length
}

pub fn part1(input: &str) -> i64 {
    let keypad = init_keypad();
    let mut memo = HashMap::new();

    input
        .trim()
        .lines()
        .map(|code| {
            let numerical: String = code.chars().filter(|c| c.is_digit(10)).collect();
            let num = numerical.parse::<i64>().unwrap();
            num * get_key_presses(&keypad, code, 2, &mut memo)
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let keypad = init_keypad();
    let mut memo = HashMap::new();

    input
        .trim()
        .lines()
        .map(|code| {
            let numerical: String = code.chars().filter(|c| c.is_digit(10)).collect();
            let num = numerical.parse::<i64>().unwrap();
            num * get_key_presses(&keypad, code, 25, &mut memo)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/day21.txt").expect("Input file should exist");
        assert_eq!(part1(&input), 270084);
        assert_eq!(part2(&input), 329431019997766);
    }
}
