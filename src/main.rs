use std::fs;

fn day01_analyze_floors(input: &str) -> (i32, Option<usize>) {
    let mut floor = 0;
    let mut first_basement_pos = None;

    for (i, ch) in input.chars().enumerate() {
        floor += match ch {
          '(' => 1,
          ')' => -1,
          _ => 0  
        };

        if floor == -1 && first_basement_pos == None {
            first_basement_pos = Some(i+1);
        }
    }

    (floor, first_basement_pos)
}

fn main() {
    println!("AOC 2015 day 1");

    let input = fs::read_to_string("input.txt").expect("Failed to read input file.");
    let (final_floor, first_basement_pos) = day01_analyze_floors(&input);
    println!("Final floor: {}", final_floor);
    println!("First basement position: {}", first_basement_pos.unwrap());

}
