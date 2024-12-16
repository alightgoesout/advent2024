use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use super::{Direction, Position};
use crate::{error, Error};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Space {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Clone)]
pub struct Map<const N: usize> {
    robot: Position,
    spaces: [[Space; N]; N],
}

impl<const N: usize> Map<N> {
    pub fn move_robot(&mut self, direction: Direction) {
        let next_position = self.robot.next(direction);
        if self.empty_space(next_position, direction) {
            self.spaces[next_position.row][next_position.column] = Space::Empty;
            self.robot = next_position;
        }
    }

    fn empty_space(&mut self, position: Position, direction: Direction) -> bool {
        let space = &self.spaces[position.row][position.column];
        match space {
            Space::Empty => true,
            Space::Wall => false,
            Space::Box => {
                let next_position = position.next(direction);
                if self.empty_space(next_position, direction) {
                    self.spaces[next_position.row][next_position.column] = Space::Box;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn sum_of_boxes_gps(&self) -> usize {
        (0..N)
            .cartesian_product(0..N)
            .filter_map(|(column, row)| match self.spaces[row][column] {
                Space::Box => Some(100 * row + column),
                Space::Empty | Space::Wall => None,
            })
            .sum()
    }
}

impl<const N: usize> Display for Map<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..N {
            for column in 0..N {
                let c = match self.spaces[row][column] {
                    Space::Wall => '#',
                    Space::Box => 'O',
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

impl<const N: usize> FromStr for Map<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot = Position { column: 0, row: 0 };
        let mut spaces = [[Space::Empty; N]; N];

        for (i, c) in s.chars().enumerate() {
            if i % (N + 1) == N {
                continue;
            }
            let row = i / (N + 1);
            let column = i % (N + 1);
            spaces[row][column] = match c {
                '.' => Space::Empty,
                '#' => Space::Wall,
                'O' => Space::Box,
                '@' => {
                    robot = Position { column, row };
                    Space::Empty
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

    const SMALL_EXAMPLE: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
";

    const LARGE_EXAMPLE: &str = "\
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
";

    #[test]
    fn small_example() {
        let mut map: Map<8> = SMALL_EXAMPLE.parse().unwrap();
        let directions = parse_moves("<^^>>>vv<v>>v<<").unwrap();

        directions
            .iter()
            .for_each(|direction| map.move_robot(*direction));

        assert_eq!(
            map.to_string(),
            "\
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
"
        );
    }

    #[test]
    fn large_example() {
        let mut map: Map<10> = LARGE_EXAMPLE.parse().unwrap();
        let directions = parse_moves(
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        )
        .unwrap();

        directions
            .iter()
            .for_each(|direction| map.move_robot(*direction));

        assert_eq!(
            map.to_string(),
            "\
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
"
        );
    }

    #[test]
    fn sum_of_boxes_gps_small_example() {
        let map: Map<8> = "\
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
"
        .parse()
        .unwrap();

        assert_eq!(map.sum_of_boxes_gps(), 2028);
    }

    #[test]
    fn sum_of_boxes_gps_large_example() {
        let map: Map<10> = "\
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
"
        .parse()
        .unwrap();

        assert_eq!(map.sum_of_boxes_gps(), 10092);
    }
}
