use std::env;

mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
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
        2 => day02::solve(),
        3 => {
            day03::part1();
            day03::part2();
        }
        4 => {
            day04::part1();
            day04::part2();
        }
        5 => {
            day05::part1();
            day05::part2();
        }
        6 => {
            day06::part1();
            day06::part2();
        }
        7 => {
            day07::part1();
            day07::part2();
        }
        8 => {
            day08::part1();
            day08::part2();
        }
        9 => {
            day09::part1();
            day09::part2();
        }
        10 => {
            day10::part1();
            day10::part2();
        }
        _ => println!("No solution for day {}.", day),
    }
}
