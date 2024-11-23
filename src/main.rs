use std::fs;

mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn day01_analyze_floors(input: &str) -> (i32, Option<usize>) {
    let mut floor = 0;
    let mut first_basement_pos = None;

    for (i, ch) in input.chars().enumerate() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if floor == -1 && first_basement_pos == None {
            first_basement_pos = Some(i + 1);
        }
    }

    (floor, first_basement_pos)
}

fn main() {
    println!("AOC 2015 day 1");

    let mut input = fs::read_to_string("input_01.txt").expect("Failed to read input file.");
    let (final_floor, first_basement_pos) = day01_analyze_floors(&input);
    println!("Final floor: {}", final_floor);
    println!("First basement position: {}", first_basement_pos.unwrap());

    println!("AOC 2015 day 2");

    input = fs::read_to_string("input_02.txt").expect("Failed to read input file.");
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

    /*
    day03::part1();
    day03::part2();

    day04::part1();
    day04::part2();

    day05::part1();
    day05::part2();

    day06::part1();
    day06::part2();

    day07::part1();
    day07::part2();
    */
    day08::part1();
    day08::part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "(())";
        let (final_floor, first_basement_position) = day01_analyze_floors(input);
        assert_eq!(final_floor, 0);
        assert_eq!(first_basement_position, None);
    }

    #[test]
    fn test_day1_input() {
        let input = fs::read_to_string("input_01.txt").expect("Failed to read input file.");
        let (final_floor, first_basement_position) = day01_analyze_floors(&input);
        assert_eq!(final_floor, 138);
        assert_eq!(first_basement_position, Some(1771));
    }
}
