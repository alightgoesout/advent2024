use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Index;
use std::str::FromStr;

use crate::{error, Error, Result, Solution};

mod input;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self) -> Result<String> {
        let hiking_map: HikingMap<41> = input::INPUT.parse()?;
        let sum_of_all_trail_starts_score = hiking_map.sum_of_all_trail_starts_score();
        Ok(format!(
            "Sum of all trail starts score: {sum_of_all_trail_starts_score}"
        ))
    }

    fn part_two(&self) -> Result<String> {
        let hiking_map: HikingMap<41> = input::INPUT.parse()?;
        let sum_of_all_trail_starts_rating = hiking_map.sum_of_all_trail_starts_rating();
        Ok(format!(
            "Sum of all trail starts rating: {sum_of_all_trail_starts_rating}"
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position<const N: usize> {
    column: usize,
    row: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Path<const N: usize>(Vec<Position<N>>);

struct HikingMap<const N: usize>([[u8; N]; N]);

impl<const N: usize> HikingMap<N> {
    fn sum_of_all_trail_starts_score(&self) -> usize {
        self.find_all_trail_starts()
            .iter()
            .map(|trail_start| self.find_reachable_trail_ends_from(*trail_start).len())
            .sum()
    }

    fn sum_of_all_trail_starts_rating(&self) -> usize {
        self.find_all_trail_starts()
            .iter()
            .map(|trail_start| self.find_all_paths_from(*trail_start).len())
            .sum()
    }

    fn find_all_trail_starts(&self) -> HashSet<Position<N>> {
        (0..N)
            .cartesian_product(0..N)
            .map(|(column, row)| Position { column, row })
            .filter(|position| self[*position] == 0)
            .collect()
    }

    fn find_reachable_trail_ends_from(&self, start: Position<N>) -> HashSet<Position<N>> {
        let mut trail_ends = HashSet::new();

        let mut positions = vec![start];

        while let Some(position) = positions.pop() {
            if self[position] == 9 {
                trail_ends.insert(position);
            } else {
                let mut next_positions = self.next_positions(position);
                positions.append(&mut next_positions)
            }
        }

        trail_ends
    }

    fn find_all_paths_from(&self, start: Position<N>) -> Vec<Path<N>> {
        let mut paths = Vec::new();

        let mut current_paths = vec![vec![start]];

        while let Some(path) = current_paths.pop() {
            let last_position = path[path.len() - 1];
            if self[last_position] == 9 {
                paths.push(Path(path))
            } else {
                for next in self.next_positions(last_position) {
                    let mut new_path = path.clone();
                    new_path.push(next);
                    current_paths.push(new_path);
                }
            }
        }

        paths
    }

    fn next_positions(&self, Position { column, row }: Position<N>) -> Vec<Position<N>> {
        let expected_height = self.0[row][column] + 1;

        let mut next_positions = Vec::new();

        if column > 0 && self.0[row][column - 1] == expected_height {
            next_positions.push(Position {
                column: column - 1,
                row,
            });
        }
        if column < (N - 1) && self.0[row][column + 1] == expected_height {
            next_positions.push(Position {
                column: column + 1,
                row,
            });
        }
        if row > 0 && self.0[row - 1][column] == expected_height {
            next_positions.push(Position {
                column,
                row: row - 1,
            });
        }
        if row < (N - 1) && self.0[row + 1][column] == expected_height {
            next_positions.push(Position {
                column,
                row: row + 1,
            });
        }

        next_positions
    }
}

impl<const N: usize> Index<Position<N>> for HikingMap<N> {
    type Output = u8;

    fn index(&self, Position { column, row }: Position<N>) -> &Self::Output {
        &self.0[row][column]
    }
}

impl<const N: usize> Display for HikingMap<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..N {
            for column in 0..N {
                write!(f, "{}", self.0[row][column])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const N: usize> FromStr for HikingMap<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = [[0; N]; N];
        for (i, c) in s.chars().enumerate() {
            if i % (N + 1) == N {
                continue;
            }
            let height = c
                .to_digit(10)
                .ok_or_else(|| error!("Invalid height: {c}"))?;
            map[i / (N + 1)][i % (N + 1)] = height as u8;
        }
        Ok(HikingMap(map))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_and_display_small_example() {
        let input = "\
0123
1234
8765
9876
";
        let hiking_map: HikingMap<4> = input.parse().unwrap();

        assert_eq!(hiking_map.to_string(), input);
    }

    #[test]
    fn parse_and_display_big_example() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        let hiking_map: HikingMap<8> = input.parse().unwrap();

        assert_eq!(hiking_map.to_string(), input);
    }

    #[test]
    fn find_reachable_trail_ends_from_small_example() {
        let input = "\
0123
1234
8765
9876
";
        let hiking_map: HikingMap<4> = input.parse().unwrap();

        let result = hiking_map.find_reachable_trail_ends_from(Position { column: 0, row: 0 });

        assert_eq!(result, HashSet::from([Position { column: 0, row: 3 }]));
    }

    #[test]
    fn find_reachable_trail_ends_from_big_example() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        let hiking_map: HikingMap<8> = input.parse().unwrap();

        let result = hiking_map.find_reachable_trail_ends_from(Position { column: 2, row: 0 });

        assert_eq!(
            result,
            HashSet::from([
                Position { column: 1, row: 0 },
                Position { column: 0, row: 3 },
                Position { column: 4, row: 3 },
                Position { column: 5, row: 4 },
                Position { column: 4, row: 5 },
            ])
        );
    }

    #[test]
    fn find_all_trail_starts_small_example() {
        let input = "\
0123
1234
8765
9876
";
        let hiking_map: HikingMap<4> = input.parse().unwrap();

        assert_eq!(
            hiking_map.find_all_trail_starts(),
            HashSet::from([Position { column: 0, row: 0 }]),
        );
    }

    #[test]
    fn find_all_trail_starts_big_example() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        let hiking_map: HikingMap<8> = input.parse().unwrap();

        assert_eq!(
            hiking_map.find_all_trail_starts(),
            HashSet::from([
                Position { column: 2, row: 0 },
                Position { column: 4, row: 0 },
                Position { column: 4, row: 2 },
                Position { column: 6, row: 4 },
                Position { column: 2, row: 5 },
                Position { column: 5, row: 5 },
                Position { column: 0, row: 6 },
                Position { column: 6, row: 6 },
                Position { column: 1, row: 7 },
            ]),
        );
    }

    #[test]
    fn sum_of_all_trail_starts_score_small_example() {
        let input = "\
0123
1234
8765
9876
";
        let hiking_map: HikingMap<4> = input.parse().unwrap();

        let result = hiking_map.sum_of_all_trail_starts_score();

        assert_eq!(result, 1);
    }

    #[test]
    fn sum_of_all_trail_starts_score_big_example() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        let hiking_map: HikingMap<8> = input.parse().unwrap();

        let result = hiking_map.sum_of_all_trail_starts_score();

        assert_eq!(result, 36);
    }

    #[test]
    fn sum_of_all_trail_starts_rating_big_example() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        let hiking_map: HikingMap<8> = input.parse().unwrap();

        let result = hiking_map.sum_of_all_trail_starts_rating();

        assert_eq!(result, 81);
    }
}
