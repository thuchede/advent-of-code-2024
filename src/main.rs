use std::time::{Instant};

mod day01;
mod helpers;
mod day02;

fn main() {
    let time_for_part1 = Instant::now();
    let part1 = day01::part_1();
    println!("{}", part1);
    println!("Done in {}ms", time_for_part1.elapsed().as_millis());
    let time_for_part2 = Instant::now();
    let part2 = day01::part_2();
    println!("{}", part2);
    println!("Done in {}ms", time_for_part2.elapsed().as_millis());
}