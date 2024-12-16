use crate::day15::{Direction, Position};
use crate::{error, Error, Result};
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Space {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

pub struct Map<const WIDTH: usize, const HEIGHT: usize> {
    robot: Position,
    spaces: [[Space; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Map<WIDTH, HEIGHT> {
    pub fn move_robot(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.move_robot_right(),
            Direction::Left => self.move_robot_left(),
            Direction::Down => self.move_robot_down(),
            Direction::Up => self.move_robot_up(),
        }
    }

    pub fn move_robot_right(&mut self) {
        let next_position = self.robot.next(Direction::Right);
        match self[next_position] {
            Space::Empty => {
                self.robot = next_position;
            }
            Space::Wall => {}
            Space::BoxLeft => {
                if self.push_box_right(next_position) {
                    self.robot = next_position;
                }
            }
            Space::BoxRight => unreachable!(),
        }
    }

    fn push_box_right(&mut self, position: Position) -> bool {
        let next_position = Position {
            column: position.column + 2,
            row: position.row,
        };

        match self[next_position] {
            Space::Empty => {
                self[position] = Space::Empty;
                self[position.next(Direction::Right)] = Space::BoxLeft;
                self[next_position] = Space::BoxRight;
                true
            }
            Space::Wall => false,
            Space::BoxLeft => {
                if self.push_box_right(next_position) {
                    self[position] = Space::Empty;
                    self[position.next(Direction::Right)] = Space::BoxLeft;
                    self[next_position] = Space::BoxRight;
                    true
                } else {
                    false
                }
            }
            Space::BoxRight => unreachable!(),
        }
    }

    pub fn move_robot_left(&mut self) {
        let next_position = self.robot.next(Direction::Left);
        match self[next_position] {
            Space::Empty => {
                self.robot = next_position;
            }
            Space::Wall => {}
            Space::BoxLeft => unreachable!(),
            Space::BoxRight => {
                if self.push_box_left(next_position) {
                    self.robot = next_position;
                }
            }
        }
    }

    fn push_box_left(&mut self, position: Position) -> bool {
        let next_position = Position {
            column: position.column - 2,
            row: position.row,
        };

        match self[next_position] {
            Space::Empty => {
                self[position] = Space::Empty;
                self[position.next(Direction::Left)] = Space::BoxRight;
                self[next_position] = Space::BoxLeft;
                true
            }
            Space::Wall => false,
            Space::BoxLeft => unreachable!(),
            Space::BoxRight => {
                if self.push_box_left(next_position) {
                    self[position] = Space::Empty;
                    self[position.next(Direction::Left)] = Space::BoxRight;
                    self[next_position] = Space::BoxLeft;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn move_robot_down(&mut self) {
        let next_position = self.robot.next(Direction::Down);
        match self[next_position] {
            Space::Empty => {
                self.robot = next_position;
            }
            Space::Wall => {}
            Space::BoxLeft => {
                if self.can_move_box_down(next_position) {
                    self.move_box_down(next_position);
                    self.robot = next_position;
                }
            }
            Space::BoxRight => {
                let box_position = Position {
                    column: next_position.column - 1,
                    row: next_position.row,
                };
                if self.can_move_box_down(box_position) {
                    self.move_box_down(box_position);
                    self.robot = next_position;
                }
            }
        }
    }

    fn can_move_box_down(&self, Position { column, row }: Position) -> bool {
        let next_positions = [
            Position {
                column,
                row: row + 1,
            },
            Position {
                column: column + 1,
                row: row + 1,
            },
        ];
        match (self[next_positions[0]], self[next_positions[1]]) {
            (Space::Empty, Space::Empty) => true,
            (Space::BoxLeft, Space::BoxRight) => self.can_move_box_down(next_positions[0]),
            (Space::BoxRight, Space::BoxLeft) => {
                self.can_move_box_down(Position {
                    column: column - 1,
                    row: row + 1,
                }) && self.can_move_box_down(next_positions[1])
            }
            (Space::BoxRight, Space::Empty) => self.can_move_box_down(Position {
                column: column - 1,
                row: row + 1,
            }),
            (Space::Empty, Space::BoxLeft) => self.can_move_box_down(next_positions[1]),
            _ => false,
        }
    }

    fn move_box_down(&mut self, Position { column, row }: Position) {
        let next_positions = [
            Position {
                column,
                row: row + 1,
            },
            Position {
                column: column + 1,
                row: row + 1,
            },
        ];
        if self[next_positions[0]] == Space::BoxLeft {
            self.move_box_down(next_positions[0]);
        }
        if self[next_positions[0]] == Space::BoxRight {
            self.move_box_down(Position {
                column: column - 1,
                row: row + 1,
            });
        }
        if self[next_positions[1]] == Space::BoxLeft {
            self.move_box_down(next_positions[1]);
        }
        self[Position { column, row }] = Space::Empty;
        self[Position {
            column: column + 1,
            row,
        }] = Space::Empty;
        self[next_positions[0]] = Space::BoxLeft;
        self[next_positions[1]] = Space::BoxRight;
    }

    fn move_robot_up(&mut self) {
        let next_position = self.robot.next(Direction::Up);
        match self[next_position] {
            Space::Empty => {
                self.robot = next_position;
            }
            Space::Wall => {}
            Space::BoxLeft => {
                if self.can_move_box_up(next_position) {
                    self.move_box_up(next_position);
                    self.robot = next_position;
                }
            }
            Space::BoxRight => {
                let box_position = Position {
                    column: next_position.column - 1,
                    row: next_position.row,
                };
                if self.can_move_box_up(box_position) {
                    self.move_box_up(box_position);
                    self.robot = next_position;
                }
            }
        }
    }

    fn can_move_box_up(&self, Position { column, row }: Position) -> bool {
        let next_positions = [
            Position {
                column,
                row: row - 1,
            },
            Position {
                column: column + 1,
                row: row - 1,
            },
        ];
        match (self[next_positions[0]], self[next_positions[1]]) {
            (Space::Empty, Space::Empty) => true,
            (Space::BoxLeft, Space::BoxRight) => self.can_move_box_up(next_positions[0]),
            (Space::BoxRight, Space::BoxLeft) => {
                self.can_move_box_up(Position {
                    column: column - 1,
                    row: row - 1,
                }) && self.can_move_box_up(next_positions[1])
            }
            (Space::BoxRight, Space::Empty) => self.can_move_box_up(Position {
                column: column - 1,
                row: row - 1,
            }),
            (Space::Empty, Space::BoxLeft) => self.can_move_box_up(next_positions[1]),
            _ => false,
        }
    }

    fn move_box_up(&mut self, Position { column, row }: Position) {
        let next_positions = [
            Position {
                column,
                row: row - 1,
            },
            Position {
                column: column + 1,
                row: row - 1,
            },
        ];
        if self[next_positions[0]] == Space::BoxLeft {
            self.move_box_up(next_positions[0]);
        }
        if self[next_positions[0]] == Space::BoxRight {
            self.move_box_up(Position {
                column: column - 1,
                row: row - 1,
            });
        }
        if self[next_positions[1]] == Space::BoxLeft {
            self.move_box_up(next_positions[1]);
        }
        self[Position { column, row }] = Space::Empty;
        self[Position {
            column: column + 1,
            row,
        }] = Space::Empty;
        self[next_positions[0]] = Space::BoxLeft;
        self[next_positions[1]] = Space::BoxRight;
    }

    pub fn sum_of_boxes_gps(&self) -> usize {
        (0..WIDTH)
            .cartesian_product(0..HEIGHT)
            .filter_map(|(column, row)| match self.spaces[row][column] {
                Space::BoxLeft => Some(100 * row + column),
                _ => None,
            })
            .sum()
    }

    #[cfg(test)]
    fn from_full_width_string(s: &str) -> Self {
        let mut robot = Position { column: 0, row: 0 };
        let mut spaces = [[Space::Empty; WIDTH]; HEIGHT];

        for (i, c) in s.chars().enumerate() {
            if i % (WIDTH + 1) == WIDTH {
                continue;
            }
            let row = i / (WIDTH + 1);
            let column = i % (WIDTH + 1);
            spaces[row][column] = match c {
                '.' => Space::Empty,
                '#' => Space::Wall,
                '[' => Space::BoxLeft,
                ']' => Space::BoxRight,
                '@' => {
                    robot = Position { column, row };
                    Space::Empty
                }
                _ => panic!("Invalid space: {c}"),
            }
        }

        Self { robot, spaces }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<Position> for Map<WIDTH, HEIGHT> {
    type Output = Space;

    fn index(&self, Position { column, row }: Position) -> &Self::Output {
        &self.spaces[row][column]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<Position> for Map<WIDTH, HEIGHT> {
    fn index_mut(&mut self, Position { column, row }: Position) -> &mut Self::Output {
        &mut self.spaces[row][column]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Display for Map<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..HEIGHT {
            for column in 0..WIDTH {
                let c = match self.spaces[row][column] {
                    Space::Wall => '#',
                    Space::BoxLeft => '[',
                    Space::BoxRight => ']',
                    Space::Empty => {
                        if self.robot == (Position { column, row }) {
                            '@'
                        } else {
                            '.'
                        }
                    }
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Map<WIDTH, HEIGHT> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut robot = Position { column: 0, row: 0 };
        let mut spaces = [[Space::Empty; WIDTH]; HEIGHT];

        for (i, c) in s.chars().enumerate() {
            if i % (HEIGHT + 1) == HEIGHT {
                continue;
            }
            let row = i / (HEIGHT + 1);
            let column = (i % (HEIGHT + 1)) * 2;
            match c {
                '.' => {
                    spaces[row][column] = Space::Empty;
                    spaces[row][column + 1] = Space::Empty;
                }
                '#' => {
                    spaces[row][column] = Space::Wall;
                    spaces[row][column + 1] = Space::Wall;
                }
                'O' => {
                    spaces[row][column] = Space::BoxLeft;
                    spaces[row][column + 1] = Space::BoxRight;
                }
                '@' => {
                    robot = Position { column, row };
                    spaces[row][column] = Space::Empty;
                    spaces[row][column + 1] = Space::Empty;
                }
                _ => return Err(error!("Invalid space: {c}")),
            }
        }

        Ok(Self { robot, spaces })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::day15::parse_moves;

    #[test]
    fn parse_small_example() {
        let map: Map<16, 8> = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
"
        .parse()
        .unwrap();

        assert_eq!(
            map.to_string(),
            "\
################
##....[]..[]..##
####@...[]....##
##......[]....##
##..##..[]....##
##......[]....##
##............##
################
"
        );
    }

    #[test]
    fn parse_large_example() {
        let map: Map<20, 10> = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########
"
        .parse()
        .unwrap();

        assert_eq!(
            map.to_string(),
            "\
####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################
"
        );
    }

    #[test]
    fn push_box_to_the_right() {
        let mut map = Map::<4, 2>::from_full_width_string(
            "\
@[].
....
",
        );

        map.move_robot(Direction::Right);

        assert_eq!(
            map.to_string(),
            "\
.@[]
....
"
        );
    }

    #[test]
    fn push_box_to_the_left() {
        let mut map = Map::<4, 2>::from_full_width_string(
            "\
.[]@
....
",
        );

        map.move_robot(Direction::Left);

        assert_eq!(
            map.to_string(),
            "\
[]@.
....
"
        );
    }

    #[test]
    fn push_box_down() {
        let test_data = [
            [
                "\
.@......
.[].....
........
........
",
                "\
........
.@......
.[].....
........
",
            ],
            [
                "\
..@.....
.[].....
........
........
",
                "\
........
..@.....
.[].....
........
",
            ],
            [
                "\
.@......
.[].....
.[].....
........
",
                "\
........
.@......
.[].....
.[].....
",
            ],
            [
                "\
..@.....
.[].....
.[].....
........
",
                "\
........
..@.....
.[].....
.[].....
",
            ],
            [
                "\
.@......
.[].....
[][]....
........
",
                "\
........
.@......
.[].....
[][]....
",
            ],
            [
                "\
.@......
.[].....
.#......
........
",
                "\
.@......
.[].....
.#......
........
",
            ],
        ];

        for [start, end] in test_data {
            let mut map = Map::<8, 4>::from_full_width_string(start);

            map.move_robot(Direction::Down);

            assert_eq!(map.to_string(), end);
        }
    }

    #[test]
    fn push_box_up() {
        let test_data = [
            [
                "\
........
........
.[].....
.@......
",
                "\
........
.[].....
.@......
........
",
            ],
            [
                "\
........
........
.[].....
..@.....
",
                "\
........
.[].....
..@.....
........
",
            ],
            [
                "\
........
.[].....
.[].....
.@......
",
                "\
.[].....
.[].....
.@......
........
",
            ],
            [
                "\
........
.[].....
.[].....
..@.....
",
                "\
.[].....
.[].....
..@.....
........
",
            ],
            [
                "\
........
[][]....
.[].....
.@......
",
                "\
[][]....
.[].....
.@......
........
",
            ],
        ];

        for [start, end] in test_data {
            let mut map = Map::<8, 4>::from_full_width_string(start);

            map.move_robot(Direction::Up);

            assert_eq!(map.to_string(), end);
        }
    }

    #[test]
    fn small_example() {
        let mut map: Map<14, 7> = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######
"
        .parse()
        .unwrap();
        let directions = parse_moves("<vv<<^^<<^^").unwrap();

        directions
            .iter()
            .for_each(|direction| map.move_robot(*direction));

        assert_eq!(
            map.to_string(),
            "\
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############
"
        );
    }
}
