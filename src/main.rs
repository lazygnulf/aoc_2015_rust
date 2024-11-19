use std::collections::HashMap;
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

fn move_pos(pos: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        '>' => (pos.0+1, pos.1),
        '<' => (pos.0-1, pos.1),
        '^' => (pos.0, pos.1-1),
        'v' => (pos.0, pos.1+1),
        _ => pos 
     }
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
        let h  = present[2];

        let sides = [l*w, l*h, w*h];
        wrapping_paper += 2*sides[0] + 2*sides[1] + 2*sides[2] + sides.iter().min().unwrap();

        let perimeters = [2*(l+w), 2*(l+h), 2*(w+h)];
        ribbon += perimeters.iter().min().unwrap() + l*w*h;


    }
    println!("Wrapping paper needed: {}", wrapping_paper);
    println!("Ribbon needed: {}", ribbon);


    println!("AOC 2015 day 3");

    input = fs::read_to_string("input_03.txt").expect("Failed to read input file.");
    let mut house_visited = HashMap::new();
    let mut pos = (0, 0);
    let mut robo = (0, 0);
    let mut robo_turn = false;
    house_visited.insert(pos, 2);

    for ch in input.chars() {
        if robo_turn {
            robo = move_pos(robo, ch);
            match house_visited.get(&robo) {
                Some(x) => house_visited.insert(robo, x+1),
                None => house_visited.insert(robo, 1)
            };
            
        }
        else {
            pos = move_pos(pos, ch);
            match house_visited.get(&pos) {
                Some(x) => house_visited.insert(pos, x+1),
                None => house_visited.insert(pos, 1)
            };
        }
        robo_turn = !robo_turn;
    }

    println!("Houses delivered: {}", house_visited.len());


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
