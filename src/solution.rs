use std::time::Instant;

use crate::Result;

pub trait Solution {
    fn day(&self) -> u8;
    fn part_one(&self) -> Result<String>;
    fn part_two(&self) -> Result<String>;

    fn execute(&self) -> Result<()> {
        let day = self.day();
        let start = Instant::now();
        println!("{day}:1 — {}", self.part_one()?);
        let part1_duration = start.elapsed();
        println!("Part 1 in {}ms", part1_duration.as_millis());
        println!("{day}:2 — {}", self.part_two()?);
        let part2_duration = start.elapsed() - part1_duration;
        println!("Part 2 in {}ms", part2_duration.as_millis());
        let total_duration = start.elapsed();
        println!("Done in {}ms", total_duration.as_millis());
        Ok(())
    }
}
