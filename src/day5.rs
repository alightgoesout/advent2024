use itertools::Itertools;
use once_cell::unsync::OnceCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;

use crate::input::{ParseExt, ReadLines};
use crate::{error, Error, Result, Solution};

mod input;

#[derive(Default)]
pub struct Day5 {
    page_orderings: OnceCell<PageOrderings>,
    updates: OnceCell<Vec<Update>>,
}

impl Day5 {
    fn page_orderings(&self) -> Result<&PageOrderings> {
        self.page_orderings.get_or_try_init(|| {
            PageOrderings::from_page_orderings(input::PAGE_ORDERINGS.read_lines().parse())
        })
    }

    fn updates(&self) -> Result<&[Update]> {
        self.updates
            .get_or_try_init(|| input::UPDATES.read_lines().parse().collect())
            .map(Vec::as_slice)
    }
}

impl Solution for Day5 {
    fn part_one(&self) -> Result<String> {
        let sum_of_middle_page_of_valid_updates =
            sum_of_middle_page_of_valid_updates(self.updates()?, self.page_orderings()?);
        Ok(format!(
            "Sum of middle pages of valid updates: {sum_of_middle_page_of_valid_updates}"
        ))
    }

    fn part_two(&self) -> Result<String> {
        let sum_of_middle_page_of_fixed_updates =
            sum_of_middle_page_of_fixed_updates(self.updates()?, self.page_orderings()?);
        Ok(format!(
            "Sum of middle pages of fixed updates: {sum_of_middle_page_of_fixed_updates}"
        ))
    }
}

fn sum_of_middle_page_of_valid_updates(updates: &[Update], page_orderings: &PageOrderings) -> u32 {
    updates
        .iter()
        .filter(|update| update.is_valid(page_orderings))
        .map(Update::middle_page)
        .sum()
}

fn sum_of_middle_page_of_fixed_updates(updates: &[Update], page_orderings: &PageOrderings) -> u32 {
    updates
        .iter()
        .filter(|update| !update.is_valid(page_orderings))
        .map(|update| update.fix_order(page_orderings))
        .map(|update| update.middle_page())
        .sum()
}

struct PageOrderings(HashMap<u32, Vec<u32>>);

impl PageOrderings {
    fn from_page_orderings<I>(page_orderings: I) -> Result<PageOrderings>
    where
        I: IntoIterator<Item = Result<PageOrdering>>,
    {
        Ok(PageOrderings(
            page_orderings
                .into_iter()
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|page_ordering| (page_ordering.0, page_ordering.1))
                .into_grouping_map()
                .collect(),
        ))
    }

    fn compare(&self, a: &u32, b: &u32) -> Ordering {
        if let Some(pages_after) = self.0.get(a) {
            if pages_after.contains(b) {
                return Ordering::Less;
            }
        }
        if let Some(pages_after) = self.0.get(b) {
            if pages_after.contains(a) {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl Index<u32> for PageOrderings {
    type Output = [u32];

    fn index(&self, index: u32) -> &Self::Output {
        self.0.get(&index).map(Vec::as_slice).unwrap_or(&[])
    }
}

struct PageOrdering(u32, u32);

impl FromStr for PageOrdering {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split_once('|')
            .ok_or_else(|| error!("Invalid page ordering: {s}"))
            .and_then(|(a, b)| Ok(PageOrdering(a.parse()?, b.parse()?)))
    }
}

struct Update(Vec<u32>);

impl FromStr for Update {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let pages = s
            .split(',')
            .map(|p| Ok(p.parse()?))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(pages))
    }
}

impl Update {
    fn is_valid(&self, page_orderings: &PageOrderings) -> bool {
        let length = self.0.len();
        (0..length).all(|i| {
            let page = self.0[i];
            let pages_after = &page_orderings[page];
            self.0[0..i]
                .iter()
                .all(|page_before| !pages_after.contains(page_before))
        })
    }

    fn middle_page(&self) -> u32 {
        self.0[self.0.len() / 2]
    }

    fn fix_order(&self, page_orderings: &PageOrderings) -> Update {
        let mut pages = self.0.clone();
        pages.sort_by(|a, b| page_orderings.compare(a, b));
        Update(pages)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_page_orderings() -> PageOrderings {
        PageOrderings::from_page_orderings(
            b"\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"
                .as_slice()
                .read_lines()
                .parse(),
        )
        .unwrap()
    }

    fn example_updates() -> Vec<Update> {
        b"\
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .as_slice()
            .read_lines()
            .parse()
            .collect::<Result<_>>()
            .unwrap()
    }

    #[test]
    fn is_valid_should_return_true_when_update_order_is_correct() {
        let update = Update(vec![75, 47, 61, 53, 29]);

        assert!(update.is_valid(&example_page_orderings()));
    }

    #[test]
    fn is_valid_should_return_false_when_pages_appear_in_the_wrong_order() {
        let update = Update(vec![75, 97, 47, 61, 53]);

        assert!(!update.is_valid(&example_page_orderings()));
    }

    #[test]
    fn sum_of_middle_page_of_valid_updates_example() {
        let result =
            sum_of_middle_page_of_valid_updates(&example_updates(), &example_page_orderings());

        assert_eq!(result, 143);
    }

    #[test]
    fn fix_order_example() {
        let update = Update(vec![75, 97, 47, 61, 53]);

        let result = update.fix_order(&example_page_orderings());

        assert_eq!(result.0, vec![97, 75, 47, 61, 53])
    }

    #[test]
    fn sum_of_middle_page_of_fixed_updates_example() {
        let result =
            sum_of_middle_page_of_fixed_updates(&example_updates(), &example_page_orderings());

        assert_eq!(result, 123);
    }
}
