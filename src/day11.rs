use crate::{Result, Solution};
use itertools::Itertools;
use std::collections::HashMap;

const STONES: [Stone; 8] = [
    Stone(30),
    Stone(71441),
    Stone(3784),
    Stone(580926),
    Stone(2),
    Stone(8122942),
    Stone(0),
    Stone(291),
];

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self) -> Result<String> {
        let stones_after_25_blinks = blink_stones(&STONES, 25);
        Ok(format!(
            "Number of stones after 25 blinks: {}",
            stones_after_25_blinks.len(),
        ))
    }

    fn part_two(&self) -> Result<String> {
        let nb_stones_after_75_blinks = count_stones_after_blinks(&STONES, 75);
        Ok(format!(
            "Number of stones after 75 blinks: {}",
            nb_stones_after_75_blinks
        ))
    }
}

fn blink_stones(stones: &[Stone], n: usize) -> Vec<Stone> {
    let mut stones = stones.to_vec();

    for _ in 0..n {
        stones = stones.iter().flat_map(Stone::blink).collect();
    }

    stones
}

fn count_stones_after_blinks(stones: &[Stone], n: usize) -> usize {
    let mut stones: Stones = stones.into();

    for _ in 0..n {
        stones = stones.blink();
    }

    stones.0.values().sum()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Stone(u64);

impl Stone {
    fn blink(&self) -> Vec<Stone> {
        if self.0 == 0 {
            vec![Stone(1)]
        } else {
            let s = self.0.to_string();
            if s.len() % 2 == 0 {
                let (first, second) = s.split_at(s.len() / 2);
                vec![
                    Stone(first.parse().unwrap()),
                    Stone(second.parse().unwrap()),
                ]
            } else {
                vec![Stone(self.0 * 2024)]
            }
        }
    }
}

struct Stones(HashMap<Stone, usize>);

impl Stones {
    fn blink(&self) -> Self {
        let mut stones = HashMap::new();

        for (stone, count) in &self.0 {
            let new_stones = stone.blink();
            for stone in new_stones {
                *stones.entry(stone).or_default() += count;
            }
        }

        Self(stones)
    }
}

impl From<&[Stone]> for Stones {
    fn from(value: &[Stone]) -> Self {
        Stones(value.iter().copied().counts())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blink_0() {
        let result = Stone(0).blink();

        assert_eq!(result, vec![Stone(1)]);
    }

    #[test]
    fn blink_1() {
        let result = Stone(1).blink();

        assert_eq!(result, vec![Stone(2024)]);
    }

    #[test]
    fn blink_11() {
        let result = Stone(11).blink();

        assert_eq!(result, vec![Stone(1), Stone(1)]);
    }

    #[test]
    fn blink_1000() {
        let result = Stone(1000).blink();

        assert_eq!(result, vec![Stone(10), Stone(0)]);
    }

    #[test]
    fn blink_stones_example_6_times() {
        let stones = [Stone(125), Stone(17)];

        let result = blink_stones(&stones, 6);

        assert_eq!(
            result,
            vec![
                Stone(2097446912),
                Stone(14168),
                Stone(4048),
                Stone(2),
                Stone(0),
                Stone(2),
                Stone(4),
                Stone(40),
                Stone(48),
                Stone(2024),
                Stone(40),
                Stone(48),
                Stone(80),
                Stone(96),
                Stone(2),
                Stone(8),
                Stone(6),
                Stone(7),
                Stone(6),
                Stone(0),
                Stone(3),
                Stone(2),
            ]
        );
    }

    #[test]
    fn count_stones_after_blinks_example_6_times() {
        let stones = [Stone(125), Stone(17)];

        let result = count_stones_after_blinks(&stones, 6);

        assert_eq!(result, 22);
    }
}
