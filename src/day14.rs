use once_cell::unsync::OnceCell;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;
use std::sync::OnceLock;

use crate::input::{ParseExt, ReadLines};
use crate::{error, Error, Result, Solution};

mod input;

#[derive(Default)]
pub struct Day14(OnceCell<Area<101, 103>>);

impl Day14 {
    fn area(&self) -> Result<&Area<101, 103>> {
        self.0
            .get_or_try_init(|| parse_area::<101, 103>(input::INPUT))
    }
}

impl Solution for Day14 {
    fn part_one(&self) -> Result<String> {
        let mut area = self.area()?.clone();
        (0..100).for_each(|_| area.move_robots());
        Ok(format!(
            "Safety factor after 100 seconds: {}",
            area.safety_factor()
        ))
    }

    fn part_two(&self) -> Result<String> {
        let mut area = self.area()?.clone();
        let mut seconds = 0;

        (0..read_number()?).for_each(|_| {
            seconds += 1;
            area.move_robots()
        });

        loop {
            println!("{area}");
            println!("Seconds: {seconds}");
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;
            if buffer != "\n" {
                break;
            }
            seconds += 1;
            area.move_robots();
        }

        Ok(format!(
            "Number of seconds for the robots to display the Easter egg: {seconds}"
        ))
    }
}

fn read_number() -> Result<usize> {
    loop {
        let mut buffer = String::new();
        print!("Number of seconds to skip: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut buffer)?;
        if let Ok(result) = buffer.trim_end().parse::<usize>() {
            break Ok(result);
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

#[derive(Debug, Clone)]
struct Area<const X: usize, const Y: usize> {
    robots: Vec<Robot>,
}

impl<const X: usize, const Y: usize> Area<X, Y> {
    fn move_robots(&mut self) {
        self.robots
            .iter_mut()
            .for_each(|Robot { position, velocity }| {
                let new_x = (position.x as isize + velocity.x) % X as isize;
                let new_y = (position.y as isize + velocity.y) % Y as isize;
                *position = Position {
                    x: if new_x < 0 {
                        (new_x + X as isize) as usize
                    } else {
                        new_x as usize
                    },
                    y: if new_y < 0 {
                        (new_y + Y as isize) as usize
                    } else {
                        new_y as usize
                    },
                };
            })
    }

    fn safety_factor(&self) -> usize {
        let mut quadrants = [[0, 0], [0, 0]];

        for Robot { position, .. } in &self.robots {
            match (position.x.cmp(&(X / 2)), position.y.cmp(&(Y / 2))) {
                (Ordering::Less, Ordering::Less) => quadrants[0][0] += 1,
                (Ordering::Greater, Ordering::Less) => quadrants[0][1] += 1,
                (Ordering::Less, Ordering::Greater) => quadrants[1][0] += 1,
                (Ordering::Greater, Ordering::Greater) => quadrants[1][1] += 1,
                _ => {}
            }
        }

        quadrants[0][0] * quadrants[0][1] * quadrants[1][0] * quadrants[1][1]
    }
}

impl<const X: usize, const Y: usize> Display for Area<X, Y> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut area = [[' '; X]; Y];
        for Robot { position, .. } in &self.robots {
            area[position.y][position.x] = '■';
        }

        for y in 0..Y {
            for x in 0..X {
                write!(f, "{}", area[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        static ROBOT_REGEX: OnceLock<Regex> = OnceLock::new();
        match ROBOT_REGEX
            .get_or_init(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap())
            .captures(s)
        {
            Some(captures) => {
                let (_, [px, py, vx, vy]) = captures.extract();
                Ok(Robot {
                    position: Position {
                        x: px.parse()?,
                        y: py.parse()?,
                    },
                    velocity: Velocity {
                        x: vx.parse()?,
                        y: vy.parse()?,
                    },
                })
            }
            None => Err(error!("Invalid robot: {s}")),
        }
    }
}

fn parse_area<const X: usize, const Y: usize>(input: &[u8]) -> Result<Area<X, Y>> {
    let robots = input
        .read_lines()
        .parse::<Robot>()
        .collect::<Result<Vec<_>>>()?;
    Ok(Area { robots })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_robots_stays_in_area() {
        let mut area = Area::<4, 4> {
            robots: vec![Robot {
                position: Position { x: 0, y: 0 },
                velocity: Velocity { x: 1, y: 1 },
            }],
        };

        area.move_robots();

        assert_eq!(area.robots[0].position, Position { x: 1, y: 1 });
    }

    #[test]
    fn move_robots_negative_position() {
        let mut area = Area::<4, 4> {
            robots: vec![Robot {
                position: Position { x: 0, y: 0 },
                velocity: Velocity { x: -1, y: -1 },
            }],
        };

        area.move_robots();

        assert_eq!(area.robots[0].position, Position { x: 3, y: 3 });
    }

    #[test]
    fn move_robots_negative_position_out_of_area() {
        let mut area = Area::<4, 4> {
            robots: vec![Robot {
                position: Position { x: 2, y: 2 },
                velocity: Velocity { x: 3, y: 3 },
            }],
        };

        area.move_robots();

        assert_eq!(area.robots[0].position, Position { x: 1, y: 1 });
    }
}
