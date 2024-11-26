use crate::util::read_input;

pub fn solve() {
    let (final_floor, first_basement_pos) = analyze_floors(&read_input(1));
    println!("Part 1: {}", final_floor);
    println!("Part 2: {}", first_basement_pos.unwrap());
}

fn analyze_floors(input: &str) -> (i32, Option<usize>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "(())";
        let (final_floor, first_basement_position) = analyze_floors(input);
        assert_eq!(final_floor, 0);
        assert_eq!(first_basement_position, None);
    }

    #[test]
    fn test_input() {
        let input = read_input(1);
        let (final_floor, first_basement_position) = analyze_floors(&input);
        assert_eq!(final_floor, 138);
        assert_eq!(first_basement_position, Some(1771));
    }
}
