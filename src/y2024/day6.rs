#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Visited(Direction),
    Obstacle,
    HitObstacle(HitDirections),
    Guard(Direction),
}

#[derive(Copy, Clone, PartialEq)]
struct HitDirections {
    from_left: bool,
    from_right: bool,
    from_above: bool,
    from_below: bool,
}

impl HitDirections {
    fn hit(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.from_below,
            Direction::Right => self.from_left,
            Direction::Down => self.from_above,
            Direction::Left => self.from_right,
        }
    }

    fn new(direction: Direction) -> Self { 
        let mut ret = Self {
            from_left: false,
            from_right: false,
            from_above: false,
            from_below: false,
        };
        match direction {
            Direction::Up => ret.from_below = true,
            Direction::Right => ret.from_left = true,
            Direction::Down => ret.from_above = true,
            Direction::Left => ret.from_right = true,
        }
        ret
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum NextMap {
    InProgress(Vec<Vec<Cell>>),
    Finished(FinishedMap),
}

enum FinishedMap {
    Exited(Vec<Vec<Cell>>),
    Loop(Vec<Vec<Cell>>),
}

impl NextMap {
    fn take_and_is_finished(self) -> (Vec<Vec<Cell>>, bool) {
        match self {
            NextMap::InProgress(vec) => (vec, false),
            NextMap::Finished(f) => match f {
                FinishedMap::Exited(vec) => (vec, true),
                FinishedMap::Loop(vec) => (vec, true),
            },
        }
    }
}

fn turn(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn cell_from_char(c: char) -> Cell {
    match c {
        '.' => Cell::Empty,
        '#' => Cell::Obstacle,
        '>' => Cell::Guard(Direction::Right),
        'v' => Cell::Guard(Direction::Down),
        '<' => Cell::Guard(Direction::Left),
        '^' => Cell::Guard(Direction::Up),
        _ => panic!("Invalid character in map"),
    }
}

fn char_from_cell(c: &Cell) -> char {
    match c {
        Cell::Empty => '.',
        Cell::Obstacle => '#',
        Cell::HitObstacle(_) => '@',
        Cell::Guard(Direction::Right) | Cell::Visited(Direction::Right) => '>',
        Cell::Guard(Direction::Down) | Cell::Visited(Direction::Down) => 'v',
        Cell::Guard(Direction::Left) | Cell::Visited(Direction::Left) => '<',
        Cell::Guard(Direction::Up) | Cell::Visited(Direction::Up) => '^',
    }
}

fn text_to_map(text: &str) -> Vec<Vec<Cell>> {
    text.lines()
        .map(|line| line.chars().map(cell_from_char).collect())
        .collect()
}

fn print_map(map: &[Vec<Cell>]) {
    for row in map {
        for cell in row {
            print!("{}", char_from_cell(cell));
        }
        println!();
    }
}

fn next_map(mut map: Vec<Vec<Cell>>) -> NextMap {
    let (direction, row_idx, col_idx) = map.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter().enumerate().find_map(|(col_idx, cell)| {
            if let Cell::Guard(direction) = cell {
                Some((*direction, row_idx, col_idx))
            } else {
                None
            }
        })
    }).unwrap();

    map[row_idx][col_idx] = Cell::Visited(direction);

    let (next_row_idx, next_col_idx) = match direction {
        Direction::Up => (row_idx.checked_sub(1), Some(col_idx)),
        Direction::Right => (Some(row_idx), Some(col_idx + 1)),
        Direction::Down => (Some(row_idx + 1), Some(col_idx)),
        Direction::Left => (Some(row_idx), col_idx.checked_sub(1)),
    };

    if let (Some(next_row_idx), Some(next_col_idx)) = (next_row_idx, next_col_idx) {
        if next_row_idx < map.len() && next_col_idx < map[0].len() {
            match &map[next_row_idx][next_col_idx] {
                Cell::Empty | Cell::Visited(_) => map[next_row_idx][next_col_idx] = Cell::Guard(direction),
                Cell::Obstacle => {
                    map[next_row_idx][next_col_idx] = Cell::HitObstacle(HitDirections::new(direction));
                    map[row_idx][col_idx] = Cell::Guard(turn(direction));
                }
                Cell::HitObstacle(mut directions) => {
                    if directions.hit(direction) {
                        return NextMap::Finished(FinishedMap::Loop(map));
                    }
                    map[next_row_idx][next_col_idx] = Cell::HitObstacle(directions);
                    map[row_idx][col_idx] = Cell::Guard(turn(direction));
                }
                Cell::Guard(_) => unreachable!(),
            }
            return NextMap::InProgress(map);
        }
    }
    NextMap::Finished(FinishedMap::Exited(map))
}

fn get_last_map(mut map: Vec<Vec<Cell>>) -> FinishedMap {
    loop {
        match next_map(map) {
            NextMap::InProgress(next_map) => map = next_map,
            NextMap::Finished(finished_map) => return finished_map,
        }
    }
}

fn count_locations(s: &str) -> usize {
    let mut map = text_to_map(s);
    loop {
        let (next_map, is_finished) = next_map(map).take_and_is_finished();
        map = next_map;
        if is_finished {
            break;
        }
    }
    print_map(&map);
    map.iter().flatten().filter(|&&cell| matches!(cell, Cell::Visited(_))).count()
}

fn count_obstacles_that_cause_loops(s: &str) -> usize {
    let (tx, rx) = std::sync::mpsc::channel();
    let map = text_to_map(s);
    let FinishedMap::Exited(exited_map) = get_last_map(map.clone()) else {
        panic!("Initial input shouldn't loop");
    };

    for (row_idx, row) in exited_map.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if matches!(cell, Cell::Visited(_)) {
                let mut test_map = map.clone();
                let tx = tx.clone();
                std::thread::spawn(move || {
                    test_map[row_idx][col_idx] = Cell::Obstacle;
                    if matches!(get_last_map(test_map), FinishedMap::Loop(_)) {
                        tx.send(1).unwrap();
                    }
                });
            }
        }
    }
    drop(tx);
    rx.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part1() {
        let input = std::fs::read_to_string("input/day6.txt").unwrap();
        assert_eq!(count_locations(&input), 4758);
    }

    #[test]
    pub fn test_part2() {
        let input = std::fs::read_to_string("input/day6.txt").unwrap();
        assert_eq!(count_obstacles_that_cause_loops(&input), 1670);
    }
}