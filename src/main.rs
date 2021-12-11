#![feature(drain_filter)]

mod day1;
mod day2;
mod util;
mod day3;
mod day4;


fn main() {
    println!("Day 1 Part 1: {:?}", day1::part_one());
    println!("Day 1 Part 2: {:?}", day1::part_two());
    println!("Day 2 Part 1: {:?}", day2::part_one());
    println!("Day 2 Part 2: {:?}", day2::part_two());
    println!("Day 3 Part 1: {:?}", day3::part_one());
    println!("Day 3 Part 2: {:?}", day3::part_two());
    println!("Day 4 Part 1: {:?}", day4::part_one());
    println!("Day 4 Part 2: {:?}", day4::part_two());
}
