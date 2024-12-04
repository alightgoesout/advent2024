use itertools::Itertools;

use crate::Solution;

mod input;

pub struct Day4;

impl Solution for Day4 {
    fn day(&self) -> u8 {
        4
    }

    fn part_one(&self) -> String {
        let word_search = WordSearch(input::INPUT);
        let number_of_xmas = word_search.count_word(b"XMAS");
        format!("Number of XMAS: {number_of_xmas}")
    }

    fn part_two(&self) -> String {
        let word_search = WordSearch(input::INPUT);
        let number_of_x_mas = word_search.count_x_mas();
        format!("Number of X-MAS: {number_of_x_mas}")
    }
}

struct WordSearch<const N: usize>([[u8; N]; N]);

impl<const N: usize> From<&[u8]> for WordSearch<N> {
    fn from(bytes: &[u8]) -> Self {
        let mut grid = [[0; N]; N];
        (0..N).for_each(|i| {
            let start = i * N;
            let end = start + N;
            grid[i].copy_from_slice(&bytes[start..end])
        });
        Self(grid)
    }
}

impl<const N: usize> WordSearch<N> {
    fn char_at(&self, Position { column, row }: Position<N>) -> u8 {
        self.0[row][column]
    }

    fn count_word(&self, word: &[u8]) -> usize {
        (0..N)
            .cartesian_product(0..N)
            .map(|(row, column)| Position { row, column })
            .map(|position| self.count_word_from_position(word, position))
            .sum()
    }

    fn count_word_from_position(&self, word: &[u8], position: Position<N>) -> usize {
        [
            Direction::LeftToRight,
            Direction::RightToLeft,
            Direction::TopToBottom,
            Direction::BottomToTop,
            Direction::TopLeftToBottomRight,
            Direction::TopRightToBottomLeft,
            Direction::BottomLeftToTopRight,
            Direction::BottomRightToTopLeft,
        ]
        .iter()
        .filter(|direction| self.is_word(word, position, **direction))
        .count()
    }

    fn is_word(&self, word: &[u8], position: Position<N>, direction: Direction) -> bool {
        if !word.is_empty() && self.char_at(position) == word[0] {
            if let Some(next_position) = position.next(direction) {
                self.is_word(&word[1..], next_position, direction)
            } else {
                word.len() == 1
            }
        } else {
            word.is_empty()
        }
    }

    fn is_x_mas(&self, Position { column, row }: Position<N>) -> bool {
        self.char_at(Position { column, row }) == b'A'
            && matches!(
                (
                    self.char_at(Position {
                        column: column - 1,
                        row: row - 1,
                    }),
                    self.char_at(Position {
                        column: column + 1,
                        row: row + 1,
                    })
                ),
                (b'M', b'S') | (b'S', b'M')
            )
            && matches!(
                (
                    self.char_at(Position {
                        column: column - 1,
                        row: row + 1,
                    }),
                    self.char_at(Position {
                        column: column + 1,
                        row: row - 1,
                    })
                ),
                (b'M', b'S') | (b'S', b'M')
            )
    }

    fn count_x_mas(&self) -> usize {
        (1..(N - 1))
            .cartesian_product(1..(N - 1))
            .map(|(row, column)| Position { row, column })
            .filter(|position| self.is_x_mas(*position))
            .count()
    }
}

#[derive(Debug, Copy, Clone)]
struct Position<const N: usize> {
    column: usize,
    row: usize,
}

impl<const N: usize> Position<N> {
    fn next(&self, direction: Direction) -> Option<Position<N>> {
        match direction {
            Direction::LeftToRight => {
                if self.column < N - 1 {
                    Some(Position {
                        column: self.column + 1,
                        row: self.row,
                    })
                } else {
                    None
                }
            }
            Direction::RightToLeft => {
                if self.column > 0 {
                    Some(Position {
                        column: self.column - 1,
                        row: self.row,
                    })
                } else {
                    None
                }
            }
            Direction::TopToBottom => {
                if self.row < N - 1 {
                    Some(Position {
                        column: self.column,
                        row: self.row + 1,
                    })
                } else {
                    None
                }
            }
            Direction::BottomToTop => {
                if self.row > 0 {
                    Some(Position {
                        column: self.column,
                        row: self.row - 1,
                    })
                } else {
                    None
                }
            }
            Direction::TopLeftToBottomRight => {
                if self.row < N - 1 && self.column < N - 1 {
                    Some(Position {
                        column: self.column + 1,
                        row: self.row + 1,
                    })
                } else {
                    None
                }
            }
            Direction::TopRightToBottomLeft => {
                if self.row < N - 1 && self.column > 0 {
                    Some(Position {
                        column: self.column - 1,
                        row: self.row + 1,
                    })
                } else {
                    None
                }
            }
            Direction::BottomLeftToTopRight => {
                if self.row > 0 && self.column < N - 1 {
                    Some(Position {
                        column: self.column + 1,
                        row: self.row - 1,
                    })
                } else {
                    None
                }
            }
            Direction::BottomRightToTopLeft => {
                if self.row > 0 && self.column > 0 {
                    Some(Position {
                        column: self.column - 1,
                        row: self.row - 1,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    TopLeftToBottomRight,
    TopRightToBottomLeft,
    BottomLeftToTopRight,
    BottomRightToTopLeft,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: WordSearch<10> = WordSearch([
        *b"MMMSXXMASM",
        *b"MSAMXMSMSA",
        *b"AMXSXMAAMM",
        *b"MSAMASMSMX",
        *b"XMASAMXAMM",
        *b"XXAMMXXAMA",
        *b"SMSMSASXSS",
        *b"SAXAMASAAA",
        *b"MAMMMXMMMM",
        *b"MXMXAXMASX",
    ]);

    const XMASES: [(Position<10>, Direction); 18] = [
        (
            Position { row: 0, column: 4 },
            Direction::TopLeftToBottomRight,
        ),
        (Position { row: 0, column: 5 }, Direction::LeftToRight),
        (Position { row: 1, column: 4 }, Direction::RightToLeft),
        (Position { row: 3, column: 9 }, Direction::TopToBottom),
        (
            Position { row: 3, column: 9 },
            Direction::TopRightToBottomLeft,
        ),
        (Position { row: 4, column: 0 }, Direction::LeftToRight),
        (Position { row: 4, column: 6 }, Direction::BottomToTop),
        (Position { row: 4, column: 6 }, Direction::RightToLeft),
        (
            Position { row: 5, column: 0 },
            Direction::BottomLeftToTopRight,
        ),
        (
            Position { row: 5, column: 6 },
            Direction::BottomRightToTopLeft,
        ),
        (
            Position { row: 9, column: 1 },
            Direction::BottomLeftToTopRight,
        ),
        (
            Position { row: 9, column: 3 },
            Direction::BottomLeftToTopRight,
        ),
        (
            Position { row: 9, column: 3 },
            Direction::BottomRightToTopLeft,
        ),
        (
            Position { row: 9, column: 5 },
            Direction::BottomLeftToTopRight,
        ),
        (
            Position { row: 9, column: 5 },
            Direction::BottomRightToTopLeft,
        ),
        (Position { row: 9, column: 5 }, Direction::LeftToRight),
        (Position { row: 9, column: 9 }, Direction::BottomToTop),
        (
            Position { row: 9, column: 9 },
            Direction::BottomRightToTopLeft,
        ),
    ];

    #[test]
    fn count_word_xmas_should_return_18_for_example() {
        assert_eq!(EXAMPLE.count_word(b"XMAS"), 18);
    }

    #[test]
    fn is_word_horizontal_left_to_right() {
        assert!(EXAMPLE.is_word(
            b"XMAS",
            Position { column: 5, row: 0 },
            Direction::LeftToRight
        ));
    }

    #[test]
    fn is_word_horizontal_left_to_right_too_short() {
        assert!(!EXAMPLE.is_word(
            b"XMAS",
            Position { column: 9, row: 3 },
            Direction::LeftToRight
        ));
    }

    #[test]
    fn is_word_for_all_xmases_in_example() {
        for (position, direction) in XMASES {
            assert!(EXAMPLE.is_word(b"XMAS", position, direction))
        }
    }

    #[test]
    fn is_xmas_3_by_3() {
        assert!(WordSearch::<3>::from(
            b"\
MAS\
AAA\
MAS\
"
            .as_slice()
        )
        .is_x_mas(Position { row: 1, column: 1 }));
        assert!(WordSearch::<3>::from(
            b"\
SAS\
MAS\
MAM\
"
            .as_slice()
        )
        .is_x_mas(Position { row: 1, column: 1 }));
        assert!(WordSearch::<3>::from(
            b"\
MAM\
AAA\
SAS\
"
            .as_slice()
        )
        .is_x_mas(Position { row: 1, column: 1 }));
        assert!(WordSearch::<3>::from(
            b"\
SAM\
MAS\
SAM\
"
            .as_slice()
        )
        .is_x_mas(Position { row: 1, column: 1 }));
        assert!(!WordSearch::<3>::from(
            b"\
SAM\
MAS\
MAM\
"
            .as_slice()
        )
        .is_x_mas(Position { row: 1, column: 1 }));
        assert!(!WordSearch::<3>::from(
            b"\
SAS\
AAA\
MAS\
"
            .as_slice()
        )
        .is_x_mas(Position { row: 1, column: 1 }));
    }

    #[test]
    fn count_x_mas_example() {
        assert_eq!(EXAMPLE.count_x_mas(), 9);
    }
}
