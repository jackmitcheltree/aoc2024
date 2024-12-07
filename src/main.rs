use std::fs;
//mod template::{one, two}
// mod day1;
// use day1::{one, two};
// mod day2;
// use day2::{one, two};
// mod day3;
// use day3::{one, two}; 
// mod day4;
// use day4::{one, two};
// mod day5;
// use day5::{one, two};
mod day6;
use day6::{one, two};

fn main() {
    let input : String = fs::read_to_string("src/input/day6.txt").unwrap();
    println!("{:?}", one(input));
    // println!("{:?}", two(input))
}
