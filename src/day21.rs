use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}

const BFS_DIRECTIONS: [(char, Direction); 4] = [
    ('^', Direction { x: 0, y: -1 }),
    ('>', Direction { x: 1, y: 0 }),
    ('v', Direction { x: 0, y: 1 }),
    ('<', Direction { x: -1, y: 0 }),
];

fn init_keypad() -> HashMap<char, Pos> {
    let mut keypad = HashMap::new();
    keypad.insert('7', Pos { x: 0, y: 0 });
    keypad.insert('8', Pos { x: 1, y: 0 });
    keypad.insert('9', Pos { x: 2, y: 0 });
    keypad.insert('4', Pos { x: 0, y: 1 });
    keypad.insert('5', Pos { x: 1, y: 1 });
    keypad.insert('6', Pos { x: 2, y: 1 });
    keypad.insert('1', Pos { x: 0, y: 2 });
    keypad.insert('2', Pos { x: 1, y: 2 });
    keypad.insert('3', Pos { x: 2, y: 2 });
    keypad.insert('X', Pos { x: 0, y: 3 });
    keypad.insert('0', Pos { x: 1, y: 3 });
    keypad.insert('A', Pos { x: 2, y: 3 });
    keypad
}

fn init_directions() -> HashMap<char, Pos> {
    let mut directions = HashMap::new();
    directions.insert('X', Pos { x: 0, y: 0 });
    directions.insert('^', Pos { x: 1, y: 0 });
    directions.insert('A', Pos { x: 2, y: 0 });
    directions.insert('<', Pos { x: 0, y: 1 });
    directions.insert('v', Pos { x: 1, y: 1 });
    directions.insert('>', Pos { x: 2, y: 1 });
    directions
}

fn get_command(input: &HashMap<char, Pos>, start: char, end: char) -> Vec<String> {
    if start == end {
        return vec!["A".to_string()];
    }

    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    let mut all_paths = Vec::new();

    queue.push_back((input[&start], String::new()));

    while let Some((current, path)) = queue.pop_front() {
        if current == input[&end] {
            all_paths.push(path.clone() + "A");
            continue;
        }

        let key = (current.x, current.y);
        if let Some(&dist) = distances.get(&key) {
            if dist < path.len() {
                continue;
            }
        }

        for (dir_name, dir) in BFS_DIRECTIONS.iter() {
            let position = Pos {
                x: current.x + dir.x,
                y: current.y + dir.y,
            };

            // Skip if position is in blank area (X)
            if position == input[&'X'] {
                continue;
            }

            // Check if position has a button
            if input.values().any(|&button| button == position) {
                let mut new_path = path.clone();
                new_path.push(*dir_name);
                let pos_key = (position.x, position.y);

                if !distances.contains_key(&pos_key) || distances[&pos_key] >= new_path.len() {
                    queue.push_back((position, new_path.clone()));
                    distances.insert(pos_key, new_path.len());
                }
            }
        }
    }

    all_paths.sort_by_key(|path| path.len());
    all_paths
}

fn get_key_presses(
    input: &HashMap<char, Pos>,
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
