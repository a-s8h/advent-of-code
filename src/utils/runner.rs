use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

pub struct Runner<I, P1, P2>
where
    P1: FnOnce(&I) -> Box<dyn Display>,
    P2: FnOnce(&I) -> Box<dyn Display>,
{
    parse_input: fn(&str) -> I,
    part1: P1,
    part2: Option<P2>,
    day: u32,
    year: u32,
}

pub struct RunnerBuilder<I, P1, P2>
where
    P1: FnOnce(&I) -> Box<dyn Display>,
    P2: FnOnce(&I) -> Box<dyn Display>,
{
    parse_input: Option<fn(&str) -> I>,
    part1: Option<P1>,
    part2: Option<P2>,
    day: Option<u32>,
    year: Option<u32>,
}

impl<I, P1, P2> RunnerBuilder<I, P1, P2>
where
    P1: FnOnce(&I) -> Box<dyn Display>,
    P2: FnOnce(&I) -> Box<dyn Display>,
{
    pub fn new() -> Self {
        RunnerBuilder {
            parse_input: None,
            part1: None,
            part2: None,
            day: None,
            year: None,
        }
    }

    pub fn with_parser(mut self, parser: fn(&str) -> I) -> Self {
        self.parse_input = Some(parser);
        self
    }

    pub fn with_part1(mut self, part1: P1) -> Self {
        self.part1 = Some(part1);
        self
    }

    pub fn with_part2(mut self, part2: P2) -> Self {
        self.part2 = Some(part2);
        self
    }

    pub fn with_day(mut self, day: u32) -> Self {
        self.day = Some(day);
        self
    }

    pub fn with_year(mut self, year: u32) -> Self {
        self.year = Some(year);
        self
    }

    pub fn build(self) -> Result<Runner<I, P1, P2>, &'static str> {
        let parse_input = self.parse_input.ok_or("Parser function is required")?;
        let part1 = self.part1.ok_or("Part 1 function is required")?;
        let day = self.day.ok_or("Day is required")?;
        let year = self.year.ok_or("Year is required")?;

        Ok(Runner {
            parse_input,
            part1,
            part2: self.part2,
            day,
            year,
        })
    }
}

impl<I, P1, P2> Runner<I, P1, P2>
where
    P1: FnOnce(&I) -> Box<dyn Display>,
    P2: FnOnce(&I) -> Box<dyn Display>,
{
    pub fn builder() -> RunnerBuilder<I, P1, P2> {
        RunnerBuilder::new()
    }

    pub fn run(&self) -> Result<(Box<dyn Display>, Option<Box<dyn Display>>), std::io::Error> {
        let input_path = PathBuf::from("input")
            .join(format!("y{}", self.year))
            .join(format!("day{}.txt", self.day));
            
        let input = fs::read_to_string(input_path)?;
        let parsed = (self.parse_input)(&input);
        let part1_result = (self.part1)(&parsed);
        let part2_result = self.part2.as_ref().map(|p2| p2(&parsed));
        Ok((part1_result, part2_result))
    }
}
