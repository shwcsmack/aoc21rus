#![feature(drain_filter)]

mod day1;
mod day2;
mod util;
mod day3;


fn main() {
    println!("{:?}", day1::part_one());
    println!("{:?}", day1::part_two());
    println!("{:?}", day2::part_one());
    println!("{:?}", day2::part_two());
    println!("{:?}", day3::part_one());
    println!("{:?}", day3::part_two());
}
