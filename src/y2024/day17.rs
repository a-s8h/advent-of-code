use std::{collections::VecDeque, usize};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv = 0, // Divide A by 2^operand -> A
    Bxl = 1, // XOR B with literal -> B
    Bst = 2, // Set B to operand mod 8
    Jnz = 3, // Jump if A != 0
    Bxc = 4, // XOR B with C -> B
    Out = 5, // Output operand mod 8
    Bdv = 6, // Divide A by 2^operand -> B
    Cdv = 7, // Divide A by 2^operand -> C
}

impl TryFrom<u8> for Instruction {
    type Error = String;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jnz),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            _ => Err(format!("Invalid instruction: {}", value)),
        }
    }
}

struct Computer {
    registers: [i64; 3], // A, B, C
    instruction_ptr: usize,
    program: Vec<u8>,
    output: VecDeque<u8>,
}

impl Computer {
    fn new(program: Vec<u8>, a: i64, b: i64, c: i64) -> Self {
        Self {
            registers: [a, b, c],
            instruction_ptr: 0,
            program,
            output: VecDeque::new(),
        }
    }

    fn get_operand_value(&self, operand: u8, is_literal: bool) -> i64 {
        if is_literal {
            operand as i64
        } else {
            match operand {
                0..=3 => operand as i64,
                4 => self.registers[0], // A
                5 => self.registers[1], // B
                6 => self.registers[2], // C
                _ => panic!("Invalid combo operand: {}", operand),
            }
        }
    }

    fn execute_instruction(&mut self) -> bool {
        if self.instruction_ptr >= self.program.len() {
            return false;
        }

        let opcode = self.program[self.instruction_ptr];
        let operand = self.program[self.instruction_ptr + 1];
        let instruction = Instruction::try_from(opcode).unwrap();

        match instruction {
            Instruction::Adv => {
                let power = self.get_operand_value(operand, false);
                self.registers[0] /= 1 << power;
            }
            Instruction::Bxl => {
                self.registers[1] ^= operand as i64;
            }
            Instruction::Bst => {
                self.registers[1] = self.get_operand_value(operand, false) % 8;
            }
            Instruction::Jnz => {
                if self.registers[0] != 0 {
                    self.instruction_ptr = operand as usize;
                    return true;
                }
            }
            Instruction::Bxc => {
                self.registers[1] ^= self.registers[2];
            }
            Instruction::Out => {
                let value = (self.get_operand_value(operand, false) % 8) as u8;
                self.output.push_back(value);
            }
            Instruction::Bdv => {
                let power = self.get_operand_value(operand, false);
                self.registers[1] = self.registers[0] / (1 << power);
            }
            Instruction::Cdv => {
                let power = self.get_operand_value(operand, false);
                self.registers[2] = self.registers[0] / (1 << power);
            }
        }

        self.instruction_ptr += 2;
        true
    }

    fn run(&mut self) -> String {
        while self.execute_instruction() {}
        self.output
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn run_with_a(&mut self, initial_a: i64) -> Vec<u8> {
        self.registers[0] = initial_a;
        self.registers[1] = 0;
        self.registers[2] = 0;
        self.instruction_ptr = 0;
        self.output.clear();
        
        while self.execute_instruction() {}
        
        self.output.iter().copied().collect()
    }

    fn verify_self_copy(&mut self, initial_a: i64) -> bool {
        let output = self.run_with_a(initial_a);
        output == self.program
    }
}

fn decoded_iteration(a: i64) -> i64 {
    let mut b = a % 8;
    b ^= 7;
    let c = a / (2 as i64).pow(b as u32);
    b ^= 7;
    b ^= c;
    b % 8
}

fn find(a: i64, program: &[u8], index: usize, results: &mut Vec<i64>) {
    if decoded_iteration(a) != program[index] as i64 {
        return;
    }

    if index == 0 {
        results.push(a);
        return;
    }

    for i in 0..8 {
        find(a * 8 + i, program, index - 1, results);
    }
}

fn parse_input(input: &str) -> (Vec<u8>, i64, i64, i64) {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .trim_start_matches("Register A: ")
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .trim_start_matches("Register B: ")
        .parse()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .trim_start_matches("Register C: ")
        .parse()
        .unwrap();
    
    lines.next(); // Skip empty line
    let program = lines
        .next()
        .unwrap()
        .trim_start_matches("Program: ")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    (program, a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let (program, a, b, c) = parse_input(input);
        let mut computer = Computer::new(program, a, b, c);
        assert_eq!(computer.run(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn run_input() {
        let input = std::fs::read_to_string("input/day17.txt").unwrap();
        let (program, a, b, c) = parse_input(&input);
        let mut computer = Computer::new(program, a, b, c);
        println!("Output: {}", computer.run());
    }

//     #[test]
//     fn test_example_part2() {
//         let input = "Register A: 2024
// Register B: 0
// Register C: 0

// Program: 0,3,5,4,3,0";
//         let (program, _, _, _) = parse_input(input);
//         assert_eq!(get_a_brute_force(&program), 117440);
//     }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day17.txt").unwrap();
        let (program, _, _, _) = parse_input(&input);
        let mut result = Vec::new();
        for a in 0..8 {
            find(a, &program, program.len() - 1, &mut result);
        }
        println!("Lowest working initial A: {}", result.iter().min().unwrap());
    }
}
