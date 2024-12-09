use regex::Regex;
use std::cell::OnceCell;
use std::sync::OnceLock;

use crate::{Result, Solution};

mod input;

#[derive(Default)]
pub struct Day3(OnceCell<Program>);

impl Day3 {
    fn program(&self) -> &Program {
        self.0.get_or_init(|| Program::parse(input::INPUT))
    }
}

impl Solution for Day3 {
    fn part_one(&self) -> Result<String> {
        let sum_of_multiplications = (&self.program().0)
            .iter()
            .filter_map(|instruction| {
                if let Instruction::Multiplication(multiplication) = instruction {
                    Some(multiplication)
                } else {
                    None
                }
            })
            .map(Multiplication::compute)
            .sum::<u32>();
        Ok(format!("Sum of multiplications : {sum_of_multiplications}"))
    }

    fn part_two(&self) -> Result<String> {
        let result = self.program().execute();
        Ok(format!("Sum wih only enabled multiplications: {result}"))
    }
}

#[derive(Debug, PartialEq)]
struct Program(Vec<Instruction>);

impl Program {
    fn parse(program: &str) -> Program {
        static INSTRUCTIONS_REGEX: OnceLock<Regex> = OnceLock::new();
        let instructions = INSTRUCTIONS_REGEX
            .get_or_init(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap())
            .captures_iter(program)
            .map(|captures| match captures.get(0).unwrap().as_str() {
                "do()" => Instruction::Do,
                "don't()" => Instruction::Dont,
                _ => {
                    let a = captures.get(1).unwrap().as_str().parse().unwrap();
                    let b = captures.get(2).unwrap().as_str().parse().unwrap();
                    Instruction::Multiplication(Multiplication(a, b))
                }
            })
            .collect();
        Program(instructions)
    }

    fn execute(&self) -> u32 {
        let mut enabled = true;
        let mut result = 0;

        for instruction in &self.0 {
            match instruction {
                Instruction::Multiplication(multiplication) => {
                    if enabled {
                        result += multiplication.compute()
                    }
                }
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
            }
        }

        result
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Multiplication(Multiplication),
    Do,
    Dont,
}

#[derive(Debug, PartialEq)]
struct Multiplication(u32, u32);

impl Multiplication {
    fn compute(&self) -> u32 {
        self.0 * self.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_example_part1() {
        let program = Program::parse(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );

        assert_eq!(
            program,
            Program(vec![
                Instruction::Multiplication(Multiplication(2, 4)),
                Instruction::Multiplication(Multiplication(5, 5)),
                Instruction::Multiplication(Multiplication(11, 8)),
                Instruction::Multiplication(Multiplication(8, 5)),
            ]),
        );
    }

    #[test]
    fn parse_example_part2() {
        let program = Program::parse(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        assert_eq!(
            program,
            Program(vec![
                Instruction::Multiplication(Multiplication(2, 4)),
                Instruction::Dont,
                Instruction::Multiplication(Multiplication(5, 5)),
                Instruction::Multiplication(Multiplication(11, 8)),
                Instruction::Do,
                Instruction::Multiplication(Multiplication(8, 5)),
            ]),
        );
    }
}
