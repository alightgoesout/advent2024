use crate::{error, Result, Solution};
use std::fmt::{Display, Formatter};

mod input;
mod part1;
mod part2;

pub struct Day15;

impl Solution for Day15 {
    fn part_one(&self) -> Result<String> {
        let mut map: part1::Map<50> = input::MAP.parse()?;
        parse_moves(input::MOVES)?
            .iter()
            .for_each(|direction| map.move_robot(*direction));
        let sum_of_boxes_gps = map.sum_of_boxes_gps();
        Ok(format!(
            "Sum of boxes GPS after all moves: {sum_of_boxes_gps}"
        ))
    }

    fn part_two(&self) -> Result<String> {
        let mut map: part2::Map<100, 50> = input::MAP.parse()?;
        parse_moves(input::MOVES)?
            .iter()
            .for_each(|direction| map.move_robot(*direction));
        let sum_of_boxes_gps = map.sum_of_boxes_gps();
        Ok(format!(
            "Sum of boxes GPS after all moves: {sum_of_boxes_gps}"
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    column: usize,
    row: usize,
}

impl Position {
    fn next(&self, m: Direction) -> Self {
        match m {
            Direction::Up => Position {
                column: self.column,
                row: self.row - 1,
            },
            Direction::Right => Position {
                column: self.column + 1,
                row: self.row,
            },
            Direction::Down => Position {
                column: self.column,
                row: self.row + 1,
            },
            Direction::Left => Position {
                column: self.column - 1,
                row: self.row,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
        }
    }
}

fn parse_moves(s: &str) -> crate::Result<Vec<Direction>> {
    s.chars()
        .map(|c| match c {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => Err(error!("Invalid direction: {c}")),
        })
        .collect()
}
