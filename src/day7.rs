use crate::input::{ParseExt, ReadLines};
use crate::{error, Error, Result, Solution};
use once_cell::unsync::OnceCell;
use regex::Regex;
use std::str::FromStr;
use std::sync::OnceLock;

mod input;

#[derive(Default)]
pub struct Day7(OnceCell<Vec<Equation>>);

impl Day7 {
    fn equations(&self) -> Result<&Vec<Equation>> {
        self.0
            .get_or_try_init(|| input::INPUT.read_lines().parse().collect())
    }
}

impl Solution for Day7 {
    fn part_one(&self) -> Result<String> {
        let sum_of_test_values: u64 = self
            .equations()?
            .iter()
            .filter(|equation| equation.can_evaluate_to_true_v1())
            .map(|equation| equation.test_value)
            .sum();
        Ok(format!(
            "Sum of equations that can evaluate to true: {sum_of_test_values}"
        ))
    }

    fn part_two(&self) -> Result<String> {
        let sum_of_test_values: u64 = self
            .equations()?
            .iter()
            .filter(|equation| equation.can_evaluate_to_true_v2())
            .map(|equation| equation.test_value)
            .sum();
        Ok(format!(
            "Sum of equations that can evaluate to true with concatenate: {sum_of_test_values}"
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            Operator::Concatenate => (left.to_string() + right.to_string().as_str())
                .parse()
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn evaluate(&self, operators: &[Operator]) -> bool {
        let value = operators
            .iter()
            .enumerate()
            .fold(self.operands[0], |result, (i, operator)| {
                operator.apply(result, self.operands[i + 1])
            });
        value == self.test_value
    }

    fn can_evaluate_to_true_v1(&self) -> bool {
        OperatorsCombination::<2>::new(self.operands.len() - 1)
            .any(|operators| self.evaluate(&operators))
    }

    fn can_evaluate_to_true_v2(&self) -> bool {
        OperatorsCombination::<3>::new(self.operands.len() - 1)
            .any(|operators| self.evaluate(&operators))
    }
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        static EQUATION_REGEX: OnceLock<Regex> = OnceLock::new();
        match EQUATION_REGEX
            .get_or_init(|| Regex::new(r"(\d+): (\d+(?: \d+)+)").unwrap())
            .captures(s)
        {
            Some(captures) => {
                let test_value = captures.get(1).unwrap().as_str().parse()?;
                let operands = captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(' ')
                    .map(|s| s.parse().unwrap())
                    .collect();
                Ok(Equation {
                    test_value,
                    operands,
                })
            }
            None => Err(error!("Invalid equation: {s}")),
        }
    }
}

struct OperatorsCombination<const N: usize> {
    size: usize,
    iterations: usize,
}

impl<const N: usize> OperatorsCombination<N> {
    fn new(size: usize) -> Self {
        Self {
            size,
            iterations: 0,
        }
    }
}

impl<const N: usize> Iterator for OperatorsCombination<N> {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations < N.pow(self.size as u32) {
            let operators = (0..self.size)
                .map(|i| self.iterations / N.pow(i as u32) % N)
                .map(|i| match i {
                    0 => Operator::Add,
                    1 => Operator::Multiply,
                    _ => Operator::Concatenate,
                })
                .collect();
            self.iterations += 1;
            Some(operators)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn operator_concatenate_should_return_156_for_15_and_6() {
        assert_eq!(Operator::Concatenate.apply(15, 6), 156);
    }

    #[test]
    fn operators_combination_v1_size_1() {
        let iterator = OperatorsCombination::<2>::new(1);

        let result = iterator.collect::<Vec<_>>();

        assert_eq!(result, vec![vec![Operator::Add], vec![Operator::Multiply]]);
    }

    #[test]
    fn operators_combination_v1_size_2() {
        let iterator = OperatorsCombination::<2>::new(2);

        let result = iterator.collect::<Vec<_>>();

        assert_eq!(
            result,
            vec![
                vec![Operator::Add, Operator::Add],
                vec![Operator::Multiply, Operator::Add],
                vec![Operator::Add, Operator::Multiply],
                vec![Operator::Multiply, Operator::Multiply],
            ]
        );
    }

    #[test]
    fn operators_combination_v1_size_3() {
        let iterator = OperatorsCombination::<2>::new(3);

        let result = iterator.collect::<Vec<_>>();

        assert_eq!(
            result,
            vec![
                vec![Operator::Add, Operator::Add, Operator::Add],
                vec![Operator::Multiply, Operator::Add, Operator::Add],
                vec![Operator::Add, Operator::Multiply, Operator::Add],
                vec![Operator::Multiply, Operator::Multiply, Operator::Add],
                vec![Operator::Add, Operator::Add, Operator::Multiply],
                vec![Operator::Multiply, Operator::Add, Operator::Multiply],
                vec![Operator::Add, Operator::Multiply, Operator::Multiply],
                vec![Operator::Multiply, Operator::Multiply, Operator::Multiply],
            ]
        );
    }

    #[test]
    fn operators_combination_v2_size_1() {
        let iterator = OperatorsCombination::<3>::new(1);

        let result = iterator.collect::<Vec<_>>();

        assert_eq!(
            result,
            vec![
                vec![Operator::Add],
                vec![Operator::Multiply],
                vec![Operator::Concatenate],
            ]
        );
    }

    #[test]
    fn operators_combination_v2_size_2() {
        let iterator = OperatorsCombination::<3>::new(2);

        let result = iterator.collect::<Vec<_>>();

        assert_eq!(
            result,
            vec![
                vec![Operator::Add, Operator::Add],
                vec![Operator::Multiply, Operator::Add],
                vec![Operator::Concatenate, Operator::Add],
                vec![Operator::Add, Operator::Multiply],
                vec![Operator::Multiply, Operator::Multiply],
                vec![Operator::Concatenate, Operator::Multiply],
                vec![Operator::Add, Operator::Concatenate],
                vec![Operator::Multiply, Operator::Concatenate],
                vec![Operator::Concatenate, Operator::Concatenate],
            ]
        );
    }

    #[test]
    fn operators_combination_v2_size_3() {
        let iterator = OperatorsCombination::<3>::new(3);

        let result = iterator.collect::<Vec<_>>();

        assert_eq!(
            result,
            vec![
                vec![Operator::Add, Operator::Add, Operator::Add],
                vec![Operator::Multiply, Operator::Add, Operator::Add],
                vec![Operator::Concatenate, Operator::Add, Operator::Add],
                vec![Operator::Add, Operator::Multiply, Operator::Add],
                vec![Operator::Multiply, Operator::Multiply, Operator::Add],
                vec![Operator::Concatenate, Operator::Multiply, Operator::Add],
                vec![Operator::Add, Operator::Concatenate, Operator::Add],
                vec![Operator::Multiply, Operator::Concatenate, Operator::Add],
                vec![Operator::Concatenate, Operator::Concatenate, Operator::Add],
                vec![Operator::Add, Operator::Add, Operator::Multiply],
                vec![Operator::Multiply, Operator::Add, Operator::Multiply],
                vec![Operator::Concatenate, Operator::Add, Operator::Multiply],
                vec![Operator::Add, Operator::Multiply, Operator::Multiply],
                vec![Operator::Multiply, Operator::Multiply, Operator::Multiply],
                vec![
                    Operator::Concatenate,
                    Operator::Multiply,
                    Operator::Multiply
                ],
                vec![Operator::Add, Operator::Concatenate, Operator::Multiply],
                vec![
                    Operator::Multiply,
                    Operator::Concatenate,
                    Operator::Multiply
                ],
                vec![
                    Operator::Concatenate,
                    Operator::Concatenate,
                    Operator::Multiply
                ],
                vec![Operator::Add, Operator::Add, Operator::Concatenate],
                vec![Operator::Multiply, Operator::Add, Operator::Concatenate],
                vec![Operator::Concatenate, Operator::Add, Operator::Concatenate],
                vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
                vec![
                    Operator::Multiply,
                    Operator::Multiply,
                    Operator::Concatenate
                ],
                vec![
                    Operator::Concatenate,
                    Operator::Multiply,
                    Operator::Concatenate
                ],
                vec![Operator::Add, Operator::Concatenate, Operator::Concatenate],
                vec![
                    Operator::Multiply,
                    Operator::Concatenate,
                    Operator::Concatenate
                ],
                vec![
                    Operator::Concatenate,
                    Operator::Concatenate,
                    Operator::Concatenate
                ],
            ]
        );
    }

    #[test]
    fn can_evaluate_true_example_v1_1() {
        let equation: Equation = "190: 10 19".parse().unwrap();

        assert!(equation.can_evaluate_to_true_v1());
    }

    #[test]
    fn can_evaluate_true_example_v1_2() {
        let equation: Equation = "3267: 81 40 27".parse().unwrap();

        assert!(equation.can_evaluate_to_true_v1());
    }

    #[test]
    fn can_evaluate_true_example_v1_3() {
        let equation: Equation = "83: 17 5".parse().unwrap();

        assert!(!equation.can_evaluate_to_true_v1());
    }

    #[test]
    fn can_evaluate_true_example_v2_4() {
        let equation: Equation = "156: 15 6".parse().unwrap();

        assert!(equation.can_evaluate_to_true_v2());
    }
}
