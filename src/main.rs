use std::fs;

fn main() {
    println!("AOC 2015 day 1");

    let input = fs::read_to_string("input.txt").expect("Failed to read input file.");

    let mut floor: i32 = 0;
    for ch in input.chars() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }

    println!("Final floor: {}", floor);

}
