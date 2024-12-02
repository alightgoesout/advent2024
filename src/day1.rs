use itertools::Itertools;
use std::cell::OnceCell;
use std::collections::HashMap;

use crate::input::read_lines;
use crate::Solution;

mod input;

#[derive(Default)]
pub struct Day1(OnceCell<(Vec<u32>, Vec<u32>)>);

impl Day1 {
    fn items(&self) -> (&[u32], &[u32]) {
        let (first_items, second_items) = self.0.get_or_init(|| parse_lists(input::INPUT));
        (first_items, second_items)
    }
}

impl Solution for Day1 {
    fn day(&self) -> u8 {
        1
    }

    fn part_one(&self) -> String {
        let (first_items, second_items) = self.items();
        let sum_distances: u32 = distances(first_items, second_items).sum();
        format!("Sum of all distances: {sum_distances}")
    }

    fn part_two(&self) -> String {
        let (first_items, second_items) = self.items();
        let similarity_score: usize = similarity_scores(first_items, second_items).iter().sum();
        format!("Similarity score: {similarity_score}")
    }
}

fn parse_lists(input: &[u8]) -> (Vec<u32>, Vec<u32>) {
    read_lines(input).fold(
        (Vec::new(), Vec::new()),
        |(mut first_items, mut second_items), line| {
            let (first, second) = line.split_once("   ").unwrap();
            first_items.push(first.parse().unwrap());
            second_items.push(second.parse().unwrap());
            (first_items, second_items)
        },
    )
}

fn distances(first_items: &[u32], second_items: &[u32]) -> impl Iterator<Item = u32> {
    first_items
        .iter()
        .copied()
        .sorted()
        .zip(second_items.iter().copied().sorted())
        .map(|(first, second)| first.abs_diff(second))
}

fn similarity_scores(first_items: &[u32], second_items: &[u32]) -> Vec<usize> {
    let mut occurrences = HashMap::new();
    first_items
        .iter()
        .map(|&item| {
            (item as usize)
                * *occurrences
                    .entry(item)
                    .or_insert_with(|| second_items.iter().filter(|i| **i == item).count())
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &[u8] = b"\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn parse_lists_example() {
        let result = parse_lists(EXAMPLE_INPUT);

        assert_eq!(result, (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]))
    }

    #[test]
    fn distances_example() {
        let (first_items, second_items) = parse_lists(EXAMPLE_INPUT);

        let result: Vec<_> = distances(&first_items, &second_items).collect();

        assert_eq!(result, vec![2, 1, 0, 1, 2, 5]);
    }

    #[test]
    fn similarity_scores_example() {
        let (first_items, second_items) = parse_lists(EXAMPLE_INPUT);

        let result = similarity_scores(&first_items, &second_items);

        assert_eq!(result, vec![9, 4, 0, 0, 9, 9]);
    }
}
