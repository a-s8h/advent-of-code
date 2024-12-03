// let's do state machine directly, without regexes

struct Match {
    count: usize, 
    op1: String,
    op2: String,
    op1_done: bool,
    op2_done: bool,
}

impl Match {
    fn new() -> Match {
        Match {
            count: 0,
            op1: String::new(),
            op2: String::new(),
            op1_done: false,
            op2_done: false,
        }
    }

    fn check(&mut self, sym: char) -> (bool, bool) {
        match (sym, self.count) {
            ('m', 0) => {
                self.count += 1;
                (true, false)
            },
            ('u', 1) => {
                self.count += 1;
                (true, false)
            },
            ('l', 2) => {
                self.count += 1;
                (true, false)
            },
            ('(', 3) => {
                self.count += 1;
                (true, false)
            },
            (c, 4..=6) if c.is_digit(10) && !self.op1_done => {
                self.count +=1;
                self.op1.push(c);
                (true, false)
            },
            (',', 5..=7) if !self.op1_done => {
                self.op1_done = true;
                self.count += 1;
                (true, false)
            },
            (c, 6..=11) if c.is_digit(10) && self.op1_done && !self.op2_done => {
                self.count += 1;
                self.op2.push(c);
                (true, false)
            },
            (')', 7..=12) if self.op1_done && !self.op2_done => {
                self.op2_done = true;
                self.count += 1;
                (true, true)
            },
            _ => (false, false),
        }
    }

    fn exec(&self) -> i32 {
        let op1 = self.op1.parse::<i32>().unwrap();
        let op2 = self.op2.parse::<i32>().unwrap();
        op1 * op2
    }
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let mut m = Match::new();
    for c in input.chars() {
        match m.check(c) {
            (false, _) => {
                m = Match::new();
            },
            (true, true) => {
                sum += m.exec();
                m = Match::new();
            },
            _ => (),
        };
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day3.txt").unwrap();
        assert_eq!(part1(&input), 167650499);
    }
}


