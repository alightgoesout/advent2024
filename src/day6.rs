use crate::{error, Error, Result, Solution};
use itertools::Itertools;
use once_cell::unsync::OnceCell;
use std::collections::HashSet;
use std::str::FromStr;

mod input;

#[derive(Default)]
pub struct Day6(OnceCell<MappedArea<130>>);

impl Day6 {
    fn mapped_area(&self) -> Result<&MappedArea<130>> {
        self.0.get_or_try_init(|| input::INPUT.parse())
    }
}

impl Solution for Day6 {
    fn day(&self) -> u8 {
        6
    }

    fn part_one(&self) -> Result<String> {
        let guard_path = self.mapped_area()?.clone().guard_path();
        Ok(format!("Length of guard path: {}", guard_path.len()))
    }

    fn part_two(&self) -> Result<String> {
        let looping_obstacles = self.mapped_area()?.all_looping_obstacles();
        Ok(format!(
            "Number of looping obstacles: {}",
            looping_obstacles.len()
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    column: isize,
    row: isize,
}

impl Position {
    fn forward(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Position {
                column: self.column,
                row: self.row - 1,
            },
            Direction::East => Position {
                column: self.column + 1,
                row: self.row,
            },
            Direction::South => Position {
                column: self.column,
                row: self.row + 1,
            },
            Direction::West => Position {
                column: self.column - 1,
                row: self.row,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Guard {
    direction: Direction,
    position: Position,
}

impl Guard {
    fn next_position(&self) -> Position {
        self.position.forward(self.direction)
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

#[derive(Debug, Clone)]
struct MappedArea<const N: usize> {
    obstacles: [[bool; N]; N],
    guard: Guard,
}

impl<const N: usize> MappedArea<N> {
    fn all_looping_obstacles(&self) -> Vec<Position> {
        (0..N)
            .cartesian_product(0..N)
            .map(|(row, column)| Position {
                row: row as isize,
                column: column as isize,
            })
            .filter(|position| {
                if self.has_obstacle(*position) {
                    false
                } else {
                    let mut mapped_area = self.clone();
                    mapped_area.obstacles[position.row as usize][position.column as usize] = true;
                    mapped_area.does_guard_loop()
                }
            })
            .collect()
    }

    fn guard_path(&mut self) -> HashSet<Position> {
        let mut path = HashSet::new();

        while self.is_inside(self.guard.position) {
            path.insert(self.guard.position);
            self.advance_guard();
        }

        path
    }

    fn does_guard_loop(&mut self) -> bool {
        let mut previous_positions = HashSet::new();

        while self.is_inside(self.guard.position) {
            if previous_positions.contains(&self.guard) {
                return true;
            }
            previous_positions.insert(self.guard.clone());
            self.advance_guard();
        }

        false
    }

    fn advance_guard(&mut self) {
        let mut next_position = self.guard.next_position();
        while self.has_obstacle(next_position) {
            self.guard.turn_right();
            next_position = self.guard.next_position();
        }
        self.guard.position = next_position
    }

    fn has_obstacle(&self, position: Position) -> bool {
        self.is_inside(position) && self.obstacles[position.row as usize][position.column as usize]
    }

    fn is_inside(&self, Position { column, row }: Position) -> bool {
        (0..N).contains(&(column as usize)) && (0..N).contains(&(row as usize))
    }
}

impl<const N: usize> FromStr for MappedArea<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut obstacles = [[false; N]; N];
        let mut guard_position = None;

        s.chars().enumerate().for_each(|(i, c)| match c {
            '#' => obstacles[i / N][i % N] = true,
            '^' => {
                guard_position = Some(Guard {
                    direction: Direction::North,
                    position: Position {
                        column: (i % N) as isize,
                        row: (i / N) as isize,
                    },
                })
            }
            _ => {}
        });

        guard_position
            .ok_or_else(|| error!("No guard in mapped area!"))
            .map(|guard| MappedArea { obstacles, guard })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
....#.....\
.........#\
..........\
..#.......\
.......#..\
..........\
.#..^.....\
........#.\
#.........\
......#...\
";

    #[test]
    fn guard_path_example() {
        let mut mapped_area: MappedArea<10> = EXAMPLE.parse().unwrap();

        let result = mapped_area.guard_path();

        assert_eq!(result.len(), 41);
    }

    #[test]
    fn all_looping_obstacles_example() {
        let mapped_area: MappedArea<10> = EXAMPLE.parse().unwrap();

        let result = mapped_area.all_looping_obstacles();

        assert_eq!(result.len(), 6);
    }
}
