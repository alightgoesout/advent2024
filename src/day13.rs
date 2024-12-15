use itertools::Itertools;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::str::FromStr;
use std::sync::OnceLock;

use crate::input::{FilterNotEmpty, ReadLines};
use crate::{error, Error, Result, Solution};

mod input;

#[derive(Default)]
pub struct Day13();

impl Solution for Day13 {
    fn part_one(&self) -> Result<String> {
        let claw_machines = parse_claw_machines(input::INPUT)?;
        let sum_of_tokens_to_win_all: u64 = claw_machines
            .iter()
            .filter_map(ClawMachine::minimum_token_for_prize)
            .sum();
        Ok(format!(
            "Fewest tokens to win all prizes: {sum_of_tokens_to_win_all}"
        ))
    }

    fn part_two(&self) -> Result<String> {
        let claw_machines = parse_claw_machines(input::INPUT)?
            .into_iter()
            .map(|machine| ClawMachine {
                prize_position: Position {
                    x: 10000000000000 + machine.prize_position.x,
                    y: 10000000000000 + machine.prize_position.y,
                },
                ..machine
            })
            .collect::<Vec<_>>();
        let sum_of_tokens_to_win_all: u64 = claw_machines
            .iter()
            .filter_map(ClawMachine::minimum_token_for_prize)
            .sum();
        Ok(format!(
            "Fewest tokens to win all prizes: {sum_of_tokens_to_win_all}"
        ))
    }
}

fn parse_claw_machines(input: &[u8]) -> Result<Vec<ClawMachine>> {
    input
        .read_lines()
        .filter_not_empty()
        .tuples()
        .map(|(a_button, b_button, prize)| ClawMachine::parse(&a_button, &b_button, &prize))
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct ClawMachine {
    prize_position: Position,
    a_button: Button,
    b_button: Button,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: u64,
    y: u64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Button {
    x: u64,
    y: u64,
}

impl ClawMachine {
    fn parse(a_button: &str, b_button: &str, prize: &str) -> Result<Self> {
        Ok(ClawMachine {
            prize_position: parse_prize(prize)?,
            a_button: a_button.parse()?,
            b_button: b_button.parse()?,
        })
    }

    fn minimum_token_for_prize(&self) -> Option<u64> {
        let determinant =
            (self.a_button.x * self.b_button.y) as i64 - (self.a_button.y * self.b_button.x) as i64;
        let a_numerator = (self.prize_position.x * self.b_button.y) as i64
            - (self.prize_position.y * self.b_button.x) as i64;
        let b_numerator = (self.prize_position.y * self.a_button.x) as i64
            - (self.prize_position.x * self.a_button.y) as i64;

        if determinant == 0 || a_numerator % determinant != 0 || b_numerator % determinant != 0 {
            None
        } else {
            let a = (a_numerator / determinant) as u64;
            let b = (b_numerator / determinant) as u64;
            Some(3 * a + b)
        }
    }
}

impl Display for ClawMachine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Button A: X+{}, Y+{}", self.a_button.x, self.a_button.y)?;
        writeln!(f, "Button B: X+{}, Y+{}", self.b_button.x, self.b_button.y)?;
        writeln!(
            f,
            "Prize: X={}, Y={}",
            self.prize_position.x, self.prize_position.y
        )
    }
}

impl Add<Button> for Position {
    type Output = Self;

    fn add(self, rhs: Button) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl FromStr for Button {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        static BUTTON_REGEX: OnceLock<Regex> = OnceLock::new();
        match BUTTON_REGEX
            .get_or_init(|| Regex::new(r"Button [AB]: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap())
            .captures(s)
        {
            Some(captures) => {
                let x = captures.name("x").unwrap().as_str().parse()?;
                let y = captures.name("y").unwrap().as_str().parse()?;
                Ok(Button { x, y })
            }
            None => Err(error!("Invalid button {s}")),
        }
    }
}

fn parse_prize(s: &str) -> Result<Position> {
    static PRIZE_REGEX: OnceLock<Regex> = OnceLock::new();
    match PRIZE_REGEX
        .get_or_init(|| Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap())
        .captures(s)
    {
        Some(captures) => {
            let x = captures.name("x").unwrap().as_str().parse()?;
            let y = captures.name("y").unwrap().as_str().parse()?;
            Ok(Position { x, y })
        }
        None => Err(error!("Invalid prize {s}")),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = b"\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn parse_example() {
        let claw_machines = parse_claw_machines(EXAMPLE).unwrap();

        assert_eq!(
            claw_machines,
            vec![
                ClawMachine {
                    prize_position: Position { x: 8400, y: 5400 },
                    a_button: Button { x: 94, y: 34 },
                    b_button: Button { x: 22, y: 67 },
                },
                ClawMachine {
                    prize_position: Position { x: 12748, y: 12176 },
                    a_button: Button { x: 26, y: 66 },
                    b_button: Button { x: 67, y: 21 },
                },
                ClawMachine {
                    prize_position: Position { x: 7870, y: 6450 },
                    a_button: Button { x: 17, y: 86 },
                    b_button: Button { x: 84, y: 37 },
                },
                ClawMachine {
                    prize_position: Position { x: 18641, y: 10279 },
                    a_button: Button { x: 69, y: 23 },
                    b_button: Button { x: 27, y: 71 },
                },
            ]
        );
    }

    #[test]
    fn minimum_token_for_prize_first_example_machine() {
        let claw_machine = &parse_claw_machines(EXAMPLE).unwrap()[0];

        assert_eq!(claw_machine.minimum_token_for_prize(), Some(280));
    }

    #[test]
    fn minimum_token_for_prize_example() {
        let claw_machines = &parse_claw_machines(EXAMPLE).unwrap();

        let result: u64 = claw_machines
            .iter()
            .filter_map(ClawMachine::minimum_token_for_prize)
            .sum();

        assert_eq!(result, 480);
    }
}
