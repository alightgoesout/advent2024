use itertools::Itertools;
use once_cell::unsync::OnceCell;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::{Error, Result, Solution};

mod input;

#[derive(Default)]
pub struct Day8(OnceCell<AntennaMap<50>>);

impl Day8 {
    fn antenna_map(&self) -> Result<&AntennaMap<50>> {
        self.0.get_or_try_init(|| input::INPUT.parse())
    }
}

impl Solution for Day8 {
    fn part_one(&self) -> Result<String> {
        let nb_antinodes = self.antenna_map()?.get_all_antinodes().len();
        Ok(format!("Number of antinodes: {nb_antinodes}"))
    }

    fn part_two(&self) -> Result<String> {
        let nb_antinodes = self
            .antenna_map()?
            .get_all_antinodes_with_resonant_harmonics()
            .len();
        Ok(format!(
            "Number of antinodes with resonant harmonics: {nb_antinodes}"
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    column: i32,
    row: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
struct Frequency(char);

struct AntennaMap<const N: usize>(HashMap<Frequency, Vec<Position>>);

impl<const N: usize> AntennaMap<N> {
    fn get_antinodes(&self, p1: &Position, p2: &Position) -> Vec<Position> {
        let mut antinodes = Vec::new();
        if let Some(position) = Self::position(2 * p1.column - p2.column, 2 * p1.row - p2.row) {
            antinodes.push(position)
        }
        if let Some(position) = Self::position(2 * p2.column - p1.column, 2 * p2.row - p1.row) {
            antinodes.push(position)
        }
        antinodes
    }

    fn position(column: i32, row: i32) -> Option<Position> {
        if column >= 0 && column < N as i32 && row >= 0 && row < N as i32 {
            Some(Position { column, row })
        } else {
            None
        }
    }

    fn get_all_antinodes(&self) -> HashSet<Position> {
        self.0
            .iter()
            .flat_map(|(_, positions)| positions.iter().tuple_combinations())
            .flat_map(|(p1, p2)| self.get_antinodes(p1, p2))
            .collect()
    }

    fn get_antinodes_with_resonant_harmonics(
        &self,
        p1: &Position,
        p2: &Position,
    ) -> HashSet<Position> {
        let mut antinodes = HashSet::new();

        self.add_antipodes(&mut antinodes, *p1, p1.column - p2.column, p1.row - p2.row);
        self.add_antipodes(&mut antinodes, *p2, p2.column - p1.column, p2.row - p1.row);

        antinodes
    }

    fn add_antipodes(
        &self,
        antinodes: &mut HashSet<Position>,
        mut position: Position,
        column_diff: i32,
        row_diff: i32,
    ) {
        while self.contains(&position) {
            antinodes.insert(position);
            position = Position {
                column: position.column + column_diff,
                row: position.row + row_diff,
            }
        }
    }

    fn contains(&self, &Position { column, row }: &Position) -> bool {
        column >= 0 && column < N as i32 && row >= 0 && row < N as i32
    }

    fn get_all_antinodes_with_resonant_harmonics(&self) -> HashSet<Position> {
        self.0
            .iter()
            .flat_map(|(_, positions)| positions.iter().tuple_combinations())
            .flat_map(|(p1, p2)| self.get_antinodes_with_resonant_harmonics(p1, p2))
            .collect()
    }
}

impl<const N: usize> FromStr for AntennaMap<N> {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut frequencies = HashMap::new();

        s.chars().enumerate().for_each(|(i, c)| match c {
            '.' => {}
            _ => {
                let frequency = Frequency(c);
                let position = Position {
                    column: (i % N) as i32,
                    row: (i / N) as i32,
                };
                frequencies
                    .entry(frequency)
                    .and_modify(|positions: &mut Vec<Position>| positions.push(position))
                    .or_insert_with(|| vec![position]);
            }
        });

        Ok(Self(frequencies))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_antinodes_horizontal() {
        let antenna_map: AntennaMap<7> = "\
..A.A..\
.......\
.......\
.......\
.......\
.......\
.......\
"
        .parse()
        .unwrap();

        let result = antenna_map.get_antinodes(
            &Position { column: 2, row: 0 },
            &Position { column: 4, row: 0 },
        );

        assert_eq!(
            result,
            vec![
                Position { column: 0, row: 0 },
                Position { column: 6, row: 0 },
            ]
        );
    }

    #[test]
    fn get_antinodes_horizontal_min_out() {
        let antenna_map: AntennaMap<7> = "\
.A.A...\
.......\
.......\
.......\
.......\
.......\
.......\
"
        .parse()
        .unwrap();

        let result = antenna_map.get_antinodes(
            &Position { column: 1, row: 0 },
            &Position { column: 3, row: 0 },
        );

        assert_eq!(result, vec![Position { column: 5, row: 0 }]);
    }

    #[test]
    fn get_antinodes_horizontal_max_out() {
        let antenna_map: AntennaMap<7> = "\
...A.A.\
.......\
.......\
.......\
.......\
.......\
.......\
"
        .parse()
        .unwrap();

        let result = antenna_map.get_antinodes(
            &Position { column: 3, row: 0 },
            &Position { column: 5, row: 0 },
        );

        assert_eq!(result, vec![Position { column: 1, row: 0 }]);
    }

    #[test]
    fn get_antinodes_vertical() {
        let antenna_map: AntennaMap<7> = "\
.......\
.......\
A......\
.......\
A......\
.......\
.......\
"
        .parse()
        .unwrap();

        let result = antenna_map.get_antinodes(
            &Position { column: 0, row: 2 },
            &Position { column: 0, row: 4 },
        );

        assert_eq!(
            result,
            vec![
                Position { column: 0, row: 0 },
                Position { column: 0, row: 6 },
            ]
        );
    }

    #[test]
    fn get_antinodes_diagonal_1() {
        let antenna_map: AntennaMap<7> = "\
.......\
.......\
..A....\
.......\
....A..\
.......\
.......\
"
        .parse()
        .unwrap();

        let result = antenna_map.get_antinodes(
            &Position { column: 2, row: 2 },
            &Position { column: 4, row: 4 },
        );

        assert_eq!(
            result,
            vec![
                Position { column: 0, row: 0 },
                Position { column: 6, row: 6 },
            ]
        );
    }

    #[test]
    fn get_antinodes_diagonal_2() {
        let antenna_map: AntennaMap<7> = "\
.......\
.......\
....A..\
.......\
..A....\
.......\
.......\
"
        .parse()
        .unwrap();

        let result = antenna_map.get_antinodes(
            &Position { column: 4, row: 2 },
            &Position { column: 2, row: 4 },
        );

        assert_eq!(
            result,
            vec![
                Position { column: 6, row: 0 },
                Position { column: 0, row: 6 },
            ]
        );
    }

    #[test]
    fn get_all_antinodes_example() {
        let example: AntennaMap<12> = "\
............\
........0...\
.....0......\
.......0....\
....0.......\
......A.....\
............\
............\
........A...\
.........A..\
............\
............\
"
        .parse()
        .unwrap();

        assert_eq!(example.get_all_antinodes().len(), 14);
        assert_eq!(
            example.get_all_antinodes_with_resonant_harmonics().len(),
            34,
        );
    }

    #[test]
    fn get_antinodes_with_resonant_harmonics_example() {
        let antenna_map: AntennaMap<10> = "\
T....#....\
...T......\
.T....#...\
.........#\
..#.......\
..........\
...#......\
..........\
....#.....\
..........\
"
        .parse()
        .unwrap();

        assert_eq!(
            antenna_map.get_antinodes_with_resonant_harmonics(
                &Position { column: 0, row: 0 },
                &Position { column: 1, row: 2 },
            ),
            vec![
                Position { column: 0, row: 0 },
                Position { column: 1, row: 2 },
                Position { column: 2, row: 4 },
                Position { column: 3, row: 6 },
                Position { column: 4, row: 8 },
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(
            antenna_map.get_antinodes_with_resonant_harmonics(
                &Position { column: 0, row: 0 },
                &Position { column: 3, row: 1 },
            ),
            vec![
                Position { column: 0, row: 0 },
                Position { column: 3, row: 1 },
                Position { column: 6, row: 2 },
                Position { column: 9, row: 3 },
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(
            antenna_map.get_antinodes_with_resonant_harmonics(
                &Position { column: 1, row: 2 },
                &Position { column: 3, row: 1 },
            ),
            vec![
                Position { column: 1, row: 2 },
                Position { column: 3, row: 1 },
                Position { column: 5, row: 0 },
            ]
            .into_iter()
            .collect()
        );
    }
}
