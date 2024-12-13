use std::collections::HashSet;

pub struct Grid<T = char> {
    pub cells: Vec<Vec<T>>,
    pub height: isize,
    pub width: isize,
}

impl Grid<char> {
    pub fn new_with_chars(data: &str) -> Self {
        Grid::_get_grid(data, |l| l.chars().collect())
    }
}

impl<T> Grid<T> {
    pub fn get(&self, pos: (isize, isize)) -> Option<&T> {
        self.cells.get(pos.0 as usize)?.get(pos.1 as usize)
    }

    fn _get_grid(data: &str, mapper: impl FnMut(&str) -> Vec<T>) -> Self {
        let cells: Vec<_> = data.lines().map(mapper).collect();
        let height = cells.len() as isize;
        let width = cells[0].len() as isize;
        Self { cells, height, width }
    }
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn flood(
    grid: &Grid,
    cell: (isize, isize),
    check: &char,
    visited: &mut HashSet<(isize, isize)>
) -> (usize, usize) {
    if !visited.insert(cell) {
        return (0, 0);
    }

    if let Some(item) = grid.get(cell) {
        if item != check {
            visited.remove(&cell);
            return (0, 0);
        }

        let mut area = 1;
        let mut perimeter = 0;

        for &dir in &DIRS {
            let next = (cell.0 + dir.0, cell.1 + dir.1);
            match grid.get(next) {
                Some(other) if other == item => {
                    let (a, p) = flood(grid, next, check, visited);
                    area += a;
                    perimeter += p;
                }
                _ => perimeter += 1
            }
        }

        (area, perimeter)
    } else {
        (0, 0)
    }
}

fn part_one(grid: &Grid) -> usize {
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for (r, row) in grid.cells.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            let coords = (r as isize, c as isize);
            if !visited.contains(&coords) {
                let (area, perimeter) = flood(grid, coords, cell, &mut visited);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn collect_cells(
    grid: &Grid,
    cell: (isize, isize),
    check: &char,
    visited: &HashSet<(isize, isize)>,
    region: &mut HashSet<(isize, isize)>
) {
    if region.contains(&cell) || visited.contains(&cell) {
        return;
    }

    if let Some(&item) = grid.get(cell) {
        if item != *check {
            return;
        }
        region.insert(cell);

        for &dir in &DIRS {
            let next = (cell.0 + dir.0, cell.1 + dir.1);
            collect_cells(grid, next, check, visited, region);
        }
    }
}

fn scan_perimeters(region: &HashSet<(isize, isize)>) -> usize {
    DIRS.iter().map(|&missing_dir| {
        let mut found = HashSet::new();
        let mut sides = 0;

        for &cell in region {
            if found.contains(&cell) {
                continue;
            }

            let check_dir = (cell.0 + missing_dir.0, cell.1 + missing_dir.1);
            if region.contains(&check_dir) {
                continue;
            }

            found.insert(cell);
            sides += 1;

            for &lr_dir in &[(missing_dir.1, missing_dir.0), (-missing_dir.1, -missing_dir.0)] {
                let mut cur = cell;
                while let next = (cur.0 + lr_dir.0, cur.1 + lr_dir.1) {
                    let check = (next.0 + missing_dir.0, next.1 + missing_dir.1);
                    if region.contains(&next) && !region.contains(&check) {
                        found.insert(next);
                        cur = next;
                    } else {
                        break;
                    }
                }
            }
        }
        sides
    }).sum()
}

fn part_two(grid: &Grid) -> usize {
    // get areas, then scan for perimeters in each
    let mut visited = HashSet::new();
    let mut regions = vec![];
    let mut total_price = 0;

    for (r, row) in grid.cells.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            let coords = (r as isize, c as isize);

            let mut region = HashSet::new();

            collect_cells(grid, coords, cell, &visited, &mut region);

            // this is all crazy
            for &v in region.iter() {
                visited.insert(v);
            }
            regions.push(region);
        }
    }

    for region in regions.iter() {
        let perimeter = scan_perimeters(region);
        let area = region.len();

        total_price += area * perimeter;
    }

    total_price
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day12.txt").unwrap();
        let grid = Grid::new_with_chars(&input);
        assert_eq!(part_one(&grid), 1456082);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day12.txt").unwrap();
        let grid = Grid::new_with_chars(&input);
        assert_eq!(part_two(&grid), 872382);
    }
}