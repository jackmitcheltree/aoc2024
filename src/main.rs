use std::fs;
//mod template::{one, two}
// mod day1;
mod day2;
// use day1::{one, two};
use day2::{one, two}; 

fn main() {
    let input : String = fs::read_to_string("src/input/day2.txt").unwrap();
    println!("{:?}", one(input));
    // println!("{:?}", two(input))
}
