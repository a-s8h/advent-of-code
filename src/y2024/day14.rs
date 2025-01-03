#[derive(Debug, Clone, Copy)]
struct Robot<const WIDTH: usize, const HEIGHT: usize> {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}

impl<const WIDTH: usize, const HEIGHT: usize> Robot<WIDTH, HEIGHT> {
    fn update_position_in_seconds(&mut self, seconds: isize) {
        self.x =
            ((self.x + WIDTH) as isize + ((self.dx * seconds) % WIDTH as isize)) as usize % WIDTH;
        self.y = ((self.y + HEIGHT) as isize + ((self.dy * seconds) % HEIGHT as isize)) as usize
            % HEIGHT;
    }

    fn update_position(&mut self) {
        self.x = ((self.x + WIDTH) as isize + self.dx) as usize % WIDTH;
        self.y = ((self.y + HEIGHT) as isize + self.dy) as usize % HEIGHT;
    }

    fn parse(input: &str) -> Vec<Robot<WIDTH, HEIGHT>> {
        input
            .lines()
            .filter_map(|line| {
                let mut iter = line.split_whitespace();
                let (pos, vel) = (iter.next()?, iter.next()?);

                let mut pos_iter = pos.trim_start_matches("p=").split(',');
                let mut vel_iter = vel.trim_start_matches("v=").split(',');

                let x = pos_iter.next()?.parse::<usize>().unwrap();
                let y = pos_iter.next()?.parse::<usize>().unwrap();
                let dx = vel_iter.next()?.parse::<isize>().unwrap() % WIDTH as isize;
                let dy = vel_iter.next()?.parse::<isize>().unwrap() % HEIGHT as isize;

                Some(Robot { x, y, dx, dy })
            })
            .collect()
    }

    pub fn quadrant(&self) -> Option<u8> {
        match (
            self.x < WIDTH / 2,
            self.x > WIDTH / 2,
            self.y < HEIGHT / 2,
            self.y > HEIGHT / 2,
        ) {
            (false, false, _, _) => None,
            (_, _, false, false) => None,
            (_, right, _, bottom) => Some(if right { 1 } else { 0 } + if bottom { 2 } else { 0 }),
        }
    }
}

fn safety_factor_for_quadrants<const WIDTH: usize, const HEIGHT: usize>(
    robots: Vec<Robot<WIDTH, HEIGHT>>,
) -> (i32, i32, i32, i32) {
    let mut quadrant_counts = (0, 0, 0, 0);
    for robot in &robots {
        if let Some(quadrant) = robot.quadrant() {
            match quadrant {
                0 => quadrant_counts.0 += 1,
                1 => quadrant_counts.1 += 1,
                2 => quadrant_counts.2 += 1,
                3 => quadrant_counts.3 += 1,
                _ => unreachable!(),
            }
        }
    }
    quadrant_counts
}

fn part_1(input: &str) -> i32 {
    let mut robots = Robot::<101, 103>::parse(input);
    for robot in &mut robots {
        robot.update_position_in_seconds(100);
    }

    let quadrant_counts = safety_factor_for_quadrants(robots);
    quadrant_counts.0 * quadrant_counts.1 * quadrant_counts.2 * quadrant_counts.3
}

fn standard_deviation(data: &[usize]) -> f32 {
    let sum = data.iter().sum::<usize>() as f32;
    let count = data.len() as f32;
    let mean = sum / count;
    let variance = data
        .iter()
        .map(|&value| {
            let distance = mean - value as f32;
            distance * distance
        })
        .sum::<f32>()
        / count;

    variance.sqrt()
}

fn find_picture_of_tree<const WIDTH: usize, const HEIGHT: usize>(
    robots: &mut [Robot<WIDTH, HEIGHT>],
) -> usize {
    let mut seconds = 0;
    loop {
        robots.iter_mut().for_each(|robot| robot.update_position());
        seconds += 1;

        let (xs, ys): (Vec<usize>, Vec<usize>) =
            robots.iter().map(|robot| (robot.x, robot.y)).unzip();

        let x_score = standard_deviation(&xs);
        let y_score = standard_deviation(&ys);

        if x_score < 25.0 && y_score < 25.0 {
            return seconds;
        }
    }
}

fn part2(input: &str) -> usize {
    let mut robots = Robot::<101, 103>::parse(input);
    find_picture_of_tree(&mut robots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = std::fs::read_to_string("input/day14.txt").unwrap();
        assert_eq!(part_1(&input), 216027840);
    }

    #[test]
    fn part2_test() {
        let input = std::fs::read_to_string("input/day14.txt").unwrap();
        assert_eq!(part2(&input), 6876);
    }
}
