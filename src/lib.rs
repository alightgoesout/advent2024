use std::collections::HashMap;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod error;
mod input;
mod solution;

pub use error::Error;
pub use solution::Solution;
pub type Result<T> = std::result::Result<T, Error>;

pub fn solutions() -> HashMap<u8, Box<dyn Solution>> {
    [
        Box::new(day1::Day1::default()) as Box<dyn Solution>,
        Box::new(day2::Day2::default()),
        Box::new(day3::Day3::default()),
        Box::new(day4::Day4),
        Box::new(day5::Day5::default()),
        Box::new(day6::Day6::default()),
        Box::new(day7::Day7::default()),
        Box::new(day8::Day8::default()),
    ]
    .into_iter()
    .map(|solution| (solution.day(), solution))
    .collect()
}

#[macro_export]
macro_rules! error {
    ($($x:expr),+$(,)?) => { $crate::Error(format!($($x),+)) };
}
