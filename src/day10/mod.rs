use crate::util::read_input;

static DAY: u32 = 10;

pub fn solve() {
    let input = read_input(DAY);
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input))
}

fn solve_part1(input: &str) -> usize {
    iterate_look_and_say(&input, 40).len()
}

fn solve_part2(input: &str) -> usize {
    iterate_look_and_say(&input, 50).len()
}

fn iterate_look_and_say(input: &str, iterations: usize) -> String {
    let mut result = input.to_string();
    for _ in 0..iterations {
        let next = look_an_say(&result);
        result = next;
    }
    result
}

fn look_an_say(s: &str) -> String {
    let mut result = String::new();

    let mut chars = s.chars();
    let mut current_seq_ch = chars.next().expect("Empty String");
    let mut current_seq_len: usize = 1;

    while let Some(next_ch) = chars.next() {
        if next_ch == current_seq_ch {
            current_seq_len += 1;
        } else {
            result.push_str(&format!("{}{}", current_seq_len, current_seq_ch));
            current_seq_ch = next_ch;
            current_seq_len = 1;
        }
    }
    result.push_str(&format!("{}{}", current_seq_len, current_seq_ch));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(look_an_say("1"), "11");
        assert_eq!(look_an_say("11"), "21");
        assert_eq!(look_an_say("21"), "1211");
        assert_eq!(look_an_say("1211"), "111221");
        assert_eq!(look_an_say("111221"), "312211");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&read_input(DAY)), 329356);
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&read_input(DAY)), 4666278);
    }
}
