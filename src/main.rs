use std::env;

use advent2024::solutions;

fn read_day_from_args() -> usize {
    env::args()
        .nth(1)
        .map(|arg| arg.parse())
        .expect("Missing day argument")
        .expect("Invalid day")
}

fn main() {
    let solutions = solutions();
    let day = read_day_from_args();
    if let Some(solution) = solutions.get(day - 1) {
        solution.execute(day as u8).unwrap()
    } else {
        println!("Unknown day {day}")
    }
}
