mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod error;
mod input;
mod solution;

pub use error::Error;
pub use solution::Solution;
pub type Result<T> = std::result::Result<T, Error>;

pub fn solutions() -> Vec<Box<dyn Solution>> {
    vec![
        Box::new(day1::Day1::default()) as Box<dyn Solution>,
        Box::new(day2::Day2::default()),
        Box::new(day3::Day3::default()),
        Box::new(day4::Day4),
        Box::new(day5::Day5::default()),
        Box::new(day6::Day6::default()),
        Box::new(day7::Day7::default()),
        Box::new(day8::Day8::default()),
        Box::new(day9::Day9),
        Box::new(day10::Day10),
        Box::new(day11::Day11),
        Box::new(day12::Day12::default()),
        Box::new(day13::Day13::default()),
        Box::new(day14::Day14::default()),
        Box::new(day15::Day15),
        Box::new(day16::Day16),
    ]
}

#[macro_export]
macro_rules! error {
    ($($x:expr),+$(,)?) => { $crate::Error(format!($($x),+)) };
}
