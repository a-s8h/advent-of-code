use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

pub use aoc_macro::aoc;

#[derive(Clone)]
pub enum AocPart {
    Parse,
    Part1,
    Part2,
}

inventory::collect!(AocSolution);

pub struct AocSolution {
    pub(crate) part: AocPart,
    pub(crate) func: fn(&str) -> Box<dyn Display>,
}

pub struct Runner {
    parse_input: fn(&str) -> Box<dyn Display>,
    part1: fn(&Box<dyn Display>) -> Box<dyn Display>,
    part2: Option<fn(&Box<dyn Display>) -> Box<dyn Display>>,
    day: u32,
    year: u32,
}

pub fn discover_and_run(year: u32, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let solutions: Vec<&AocSolution> = inventory::iter::<AocSolution>
        .into_iter()
        .collect();

    let mut parser = None;
    let mut part1 = None;
    let mut part2 = None;

    for solution in solutions {
        match solution.part {
            AocPart::Parse => parser = Some(solution.func),
            AocPart::Part1 => part1 = Some(solution.func),
            AocPart::Part2 => part2 = Some(solution.func),
        }
    }

    let parser = parser.ok_or("No parser found")?;
    let part1 = part1.ok_or("No part1 solution found")?;

    let runner = Runner {
        parse_input: parser,
        part1,
        part2,
        day,
        year,
    };

    let (part1_result, part2_result) = runner.run()?;
    println!("Part 1: {}", part1_result);
    if let Some(part2) = part2_result {
        println!("Part 2: {}", part2);
    }

    Ok(())
}

impl Runner {
    pub fn run(&self) -> Result<(Box<dyn Display>, Option<Box<dyn Display>>), std::io::Error> {
        let input_path = PathBuf::from("input")
            .join(format!("y{}", self.year))
            .join(format!("day{}.txt", self.day));
            
        let input = fs::read_to_string(input_path)?;
        let parsed = (self.parse_input)(&input);
        let part1_result = (self.part1)(&parsed);
        let part2_result = self.part2.map(|p2| p2(&parsed));
        Ok((part1_result, part2_result))
    }
}
