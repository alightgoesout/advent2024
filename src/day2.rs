use crate::input::{FilterNotEmpty, ParseExt, ReadLines};
use crate::{Error, Result, Solution};
use itertools::Itertools;
use once_cell::unsync::OnceCell;
use std::cmp::Ordering;
use std::str::FromStr;

mod input;

#[derive(Default)]
pub struct Day2(OnceCell<Vec<Report>>);

impl Day2 {
    fn reports(&self) -> Result<&[Report]> {
        Ok(self.0.get_or_try_init(|| parse_reports(input::INPUT))?)
    }
}

impl Solution for Day2 {
    fn part_one(&self) -> Result<String> {
        let nb_safe_reports = self
            .reports()?
            .iter()
            .filter(|report| report.is_safe())
            .count();
        Ok(format!("Number of safe reports: {nb_safe_reports}"))
    }

    fn part_two(&self) -> Result<String> {
        let nb_safe_reports_with_problem_dampener = self
            .reports()?
            .iter()
            .filter(|report| report.is_safe_with_problem_dampener())
            .count();
        Ok(format!(
            "Number of safe reports with problem dampener: {nb_safe_reports_with_problem_dampener}"
        ))
    }
}

struct Report {
    levels: Vec<u32>,
}

impl FromStr for Report {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let levels = s.split(' ').map(|s| s.parse().unwrap()).collect();
        Ok(Report { levels })
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        if self.levels.len() <= 1 {
            true
        } else {
            is_safe(&self.levels)
        }
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        let nb_reports = self.levels.len();
        if nb_reports <= 1 {
            true
        } else {
            ProblemDampenerIterator::new(self.levels.clone()).any(|sequence| is_safe(&sequence))
        }
    }
}

fn parse_reports(input: &[u8]) -> Result<Vec<Report>> {
    input.read_lines().filter_not_empty().parse().collect()
}

fn is_safe(levels: &[u32]) -> bool {
    let ordering = match levels[0].cmp(&levels[1]) {
        Ordering::Less => Ordering::Less,
        _ => Ordering::Greater,
    };
    levels
        .iter()
        .tuple_windows()
        .all(|(i, ii)| i.cmp(ii) == ordering && (1..=3).contains(&i.abs_diff(*ii)))
}

struct ProblemDampenerIterator {
    levels: Vec<u32>,
    next_to_skip: Option<usize>,
}

impl ProblemDampenerIterator {
    fn new(levels: Vec<u32>) -> Self {
        Self {
            levels,
            next_to_skip: None,
        }
    }
}

impl Iterator for ProblemDampenerIterator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_to_skip {
            None => {
                self.next_to_skip = Some(0);
                Some(self.levels.clone())
            }
            Some(i) if i < self.levels.len() => {
                self.next_to_skip = Some(i + 1);
                Some([&self.levels[0..i], &self.levels[i + 1..]].concat())
            }
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = b"\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn example_first_report_is_safe() {
        let report = Report {
            levels: vec![7, 6, 4, 2, 1],
        };

        assert!(report.is_safe());
    }

    #[test]
    fn example_second_report_is_safe() {
        let report = Report {
            levels: vec![1, 2, 7, 8, 9],
        };

        assert!(!report.is_safe());
    }

    #[test]
    fn full_example_is_safe() {
        let reports = parse_reports(EXAMPLE).unwrap();

        let result: Vec<_> = reports.iter().map(Report::is_safe).collect();

        assert_eq!(result, vec![true, false, false, false, false, true]);
    }

    #[test]
    fn full_example_is_safe_with_problem_dampener() {
        let reports = parse_reports(EXAMPLE).unwrap();

        let result: Vec<_> = reports
            .iter()
            .map(Report::is_safe_with_problem_dampener)
            .collect();

        assert_eq!(result, vec![true, false, false, true, true, true]);
    }

    #[test]
    fn is_safe_with_problem_dampener_last_level_bad() {
        let report: Report = "20 21 24 25 27 29 27".parse().unwrap();

        assert!(!report.is_safe());
        assert!(report.is_safe_with_problem_dampener());
    }

    #[test]
    fn is_safe_with_problem_dampener_last_two_levels_bad() {
        let report: Report = "20 21 24 26 29 27 25".parse().unwrap();

        assert!(!report.is_safe());
        assert!(!report.is_safe_with_problem_dampener());
    }

    #[test]
    fn is_safe_with_problem_dampener_skip_first() {
        let report: Report = "1 5 6 7".parse().unwrap();

        assert!(!report.is_safe());
        assert!(report.is_safe_with_problem_dampener());
    }

    #[test]
    fn is_safe_with_problem_dampener_first_skipped_ordering_changed() {
        let report: Report = "5 1 6 7".parse().unwrap();

        assert!(report.is_safe_with_problem_dampener());
    }

    #[test]
    fn problem_dampener_iterator() {
        let iterator = ProblemDampenerIterator::new(vec![20, 21, 24, 29, 27]);

        let result: Vec<_> = iterator.collect();

        assert_eq!(
            result,
            vec![
                vec![20, 21, 24, 29, 27],
                vec![21, 24, 29, 27],
                vec![20, 24, 29, 27],
                vec![20, 21, 29, 27],
                vec![20, 21, 24, 27],
                vec![20, 21, 24, 29],
            ]
        );
    }
}
