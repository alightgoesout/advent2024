use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::{Add, Index};

use crate::{Result, Solution};

mod input;

pub struct Day16;

impl Solution for Day16 {
    fn part_one(&self) -> Result<String> {
        let map = Map::<141>::parse(input::INPUT);
        let lowest_score = map.find_paths_with_lowest_score();
        Ok(format!(
            "Lowest score possible: {}\nNumber of tiles part of lowest score paths: {}",
            lowest_score.0,
            lowest_score.1.len()
        ))
    }

    fn part_two(&self) -> Result<String> {
        Ok(String::new())
    }
}

struct Map<const N: usize>([[char; N]; N]);

impl<const N: usize> Map<N> {
    fn parse(s: &str) -> Self {
        let mut chars = [['#'; N]; N];

        for (i, c) in s.chars().enumerate() {
            if i % (N + 1) == N {
                continue;
            }
            chars[i / (N + 1)][i % (N + 1)] = c;
        }

        Self(chars)
    }

    fn find_paths_with_lowest_score(&self) -> (usize, HashSet<Tile>) {
        let start = Position {
            tile: Tile {
                column: 1,
                row: N - 2,
            },
            direction: Direction::East,
        };

        let mut scores = HashMap::new();
        let mut to_visit = BinaryHeap::from([State {
            position: start,
            path: HashSet::new(),
            score: 0,
        }]);
        let mut tiles_on_paths_with_lowest_score = HashSet::new();

        let mut lowest_score = usize::MAX;

        while let Some(State {
            position,
            mut path,
            score,
        }) = to_visit.pop()
        {
            if score > lowest_score {
                break;
            } else if position.tile.is_at_end::<N>() {
                lowest_score = score;
                tiles_on_paths_with_lowest_score.extend(path);
            } else if score <= *scores.get(&position).unwrap_or(&usize::MAX) {
                scores.insert(position, score);
                let next = position.next();
                path.insert(position.tile);
                if self[next.tile] != '#' {
                    to_visit.push(State {
                        position: next,
                        path: path.clone(),
                        score: score + 1,
                    });
                }
                to_visit.push(State {
                    position: position.turn_left(),
                    path: path.clone(),
                    score: score + 1000,
                });
                to_visit.push(State {
                    position: position.turn_right(),
                    path,
                    score: score + 1000,
                });
            }
        }

        (lowest_score, tiles_on_paths_with_lowest_score)
    }
}

impl<const N: usize> Index<Tile> for Map<N> {
    type Output = char;

    fn index(&self, Tile { column, row }: Tile) -> &Self::Output {
        &self.0[row][column]
    }
}

impl<const N: usize> Index<usize> for Map<N> {
    type Output = [char; N];

    fn index(&self, row: usize) -> &Self::Output {
        &self.0[row]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Tile {
    column: usize,
    row: usize,
}

impl Tile {
    fn is_at_end<const N: usize>(&self) -> bool {
        self.column == N - 2 && self.row == 1
    }
}

impl Add<Direction> for Tile {
    type Output = Tile;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => Tile {
                column: self.column,
                row: self.row - 1,
            },
            Direction::East => Tile {
                column: self.column + 1,
                row: self.row,
            },
            Direction::South => Tile {
                column: self.column,
                row: self.row + 1,
            },
            Direction::West => Tile {
                column: self.column - 1,
                row: self.row,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position {
    tile: Tile,
    direction: Direction,
}

impl Position {
    fn next(&self) -> Self {
        Self {
            tile: self.tile + self.direction,
            direction: self.direction,
        }
    }

    fn turn_left(&self) -> Self {
        Self {
            tile: self.tile,
            direction: self.direction.turn_left(),
        }
    }

    fn turn_right(&self) -> Self {
        Self {
            tile: self.tile,
            direction: self.direction.turn_right(),
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct State {
    position: Position,
    path: HashSet<Tile>,
    score: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.position == other.position
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_paths_with_lowest_score_one_path() {
        let map = Map::<6>::parse(
            "\
######
#...E#
#.####
#.####
#S####
######
",
        );

        let result = map.find_paths_with_lowest_score();

        assert_eq!(
            result,
            (
                2006,
                HashSet::from([
                    Tile { column: 1, row: 4 },
                    Tile { column: 1, row: 3 },
                    Tile { column: 1, row: 2 },
                    Tile { column: 1, row: 1 },
                    Tile { column: 2, row: 1 },
                    Tile { column: 3, row: 1 },
                ])
            )
        );
    }

    #[test]
    fn find_paths_with_lowest_score_diverging_paths() {
        let map = Map::<6>::parse(
            "\
######
#...E#
#.##.#
#....#
#S####
######
",
        );

        let result = map.find_paths_with_lowest_score();

        assert_eq!(
            result,
            (
                2006,
                HashSet::from([
                    Tile { column: 1, row: 4 },
                    Tile { column: 1, row: 3 },
                    Tile { column: 1, row: 2 },
                    Tile { column: 1, row: 1 },
                    Tile { column: 2, row: 1 },
                    Tile { column: 3, row: 1 },
                ])
            )
        );
    }

    #[test]
    fn find_paths_with_lowest_score_loop() {
        let map = Map::<7>::parse(
            "\
#######
####.E#
#....##
#.##.##
#.##.##
#S...##
#######
",
        );

        let result = map.find_paths_with_lowest_score();

        assert_eq!(result.0, 2008);
        assert_eq!(result.1.len(), 8);
    }

    #[test]
    fn find_paths_with_lowest_score_two_paths() {
        let map = Map::<7>::parse(
            "\
#######
####.E#
#....##
#.##.##
#....##
#S#####
#######
",
        );

        let result = map.find_paths_with_lowest_score();

        assert_eq!(result.0, 4008);
        assert_eq!(result.1.len(), 12);
    }

    #[test]
    fn find_paths_with_lowest_score_example() {
        let map = Map::<15>::parse(
            "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
",
        );

        let result = map.find_paths_with_lowest_score();

        assert_eq!(result.0, 7036);
        assert_eq!(result.1.len(), 44);
    }
    #[test]
    fn find_paths_with_lowest_score_large_example() {
        let map = Map::<17>::parse(
            "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
",
        );

        let result = map.find_paths_with_lowest_score();

        assert_eq!(result.0, 11048);
        assert_eq!(result.1.len(), 63);
    }
}
