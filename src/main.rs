use std::time::{Instant};

mod day01;
mod helpers;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let time_for_part1 = Instant::now();
    let part1 = day06::part_1();
    let time1 = time_for_part1.elapsed().as_millis();
    println!("{}", part1);
    println!("Done in {}ms", time1);
    let time_for_part2 = Instant::now();
    let part2 = day06::part_2();
    let time2 = time_for_part2.elapsed().as_millis();
    println!("{}", part2);
    println!("Done in {}ms", time2);
}