use std::collections::HashMap;
use std::env;
use std::time::Instant;

mod day1;
mod input;

pub trait Solution {
    fn day(&self) -> u8;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;

    fn execute(&self) {
        let day = self.day();
        let start = Instant::now();
        println!("{day}:1 — {}", self.part_one());
        let part1_duration = start.elapsed();
        println!("Part 1 in {}ms", part1_duration.as_millis());
        println!("{day}:2 — {}", self.part_two());
        let part2_duration = start.elapsed() - part1_duration;
        println!("Part 2 in {}ms", part2_duration.as_millis());
        let total_duration = start.elapsed();
        println!("Done in {}ms", total_duration.as_millis());
    }
}

pub fn solutions() -> HashMap<u8, Box<dyn Solution>> {
    [Box::new(day1::Day1::default()) as Box<dyn Solution>]
        .into_iter()
        .map(|solution| (solution.day(), solution))
        .collect()
}

fn read_day_from_args() -> Option<u8> {
    env::args().nth(1).and_then(|arg| arg.parse().ok())
}

fn main() {
    let solutions = solutions();
    if let Some(solution) = read_day_from_args().and_then(|day| solutions.get(&day)) {
        solution.execute()
    }
}
