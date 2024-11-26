use std::{env, fs};

mod util;

mod day01;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    println!("AOC 2015 day 2");

    let input = fs::read_to_string("input_02.txt").expect("Failed to read input file.");
    let mut wrapping_paper = 0;
    let mut ribbon = 0;
    for line in input.lines() {
        let present: Vec<u32> = line.split('x').map(|v| v.parse::<u32>().unwrap()).collect();

        let l = present[0];
        let w = present[1];
        let h = present[2];

        let sides = [l * w, l * h, w * h];
        wrapping_paper += 2 * sides[0] + 2 * sides[1] + 2 * sides[2] + sides.iter().min().unwrap();

        let perimeters = [2 * (l + w), 2 * (l + h), 2 * (w + h)];
        ribbon += perimeters.iter().min().unwrap() + l * w * h;
    }
    println!("Wrapping paper needed: {}", wrapping_paper);
    println!("Ribbon needed: {}", ribbon);

    // NEW

    let mut day: u32 = 1;

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        day = args[1].parse::<u32>().expect("Invalid day format.");
    } else {
        println!("Usage: {} <day>", args[0])
    }

    println!("Advent of Code day {}", day);

    match day {
        1 => day01::solve(),
        3 => {
            day03::part1();
            day03::part2();
        }
        _ => println!("No solution for day {}.", day),
    }
}
