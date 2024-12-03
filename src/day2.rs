fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>() 
}

fn is_safe(report: Vec<i32>, err_count: i32) -> bool {
    let mut count_unsafe: i32 = 0;
    let is_asc: bool = report[2] > report[3];

    for i in 0..(report.len() - 1) {
        let diff: i32 = report[i] - report[i + 1];

        if diff.abs() > 3 || diff == 0 || (is_asc && diff < 0) || (!is_asc && diff > 0) {
            count_unsafe += 1;

            if count_unsafe > err_count {
                return false;
            }
        }
    }

    return true;
}

fn part1(reports: Vec<Vec<i32>>) -> usize {
    let mut count: usize = 0; 
    for report in reports {
        if is_safe(report, 0) {
            count += 1;
        }
    }
    count
}

fn part2(reports: Vec<Vec<i32>>) -> usize {
    let mut count: usize = 0; 
    for report in reports {
        if is_safe(report, 1) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let reports = parse_input(std::fs::read_to_string("input/day2.txt").unwrap().as_str());
        assert_eq!(part1(reports), 282);
    }

    #[test]
    fn test_part2() {
        let reports = parse_input(std::fs::read_to_string("input/day2.txt").unwrap().as_str());
        assert_eq!(part2(reports), 349);
    }
}
