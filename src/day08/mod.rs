use std::fs;

pub fn part1() {
    println!("AOC 2015 day 8 - part 1");
    println!("x: {}", solve_part1(&read_input()));
}

pub fn part2() {
    println!("AOC 2015 day 8 - part 2");
    println!("y : {}", solve_part2());
}

fn solve_part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| l.len() - string_memory_len(l))
        .sum()
}

fn string_memory_len(s: &str) -> usize {
    let mut count = 0;

    let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        let c = chars.next().unwrap();

        if c == '"' {
            continue;
        } else if c == '\\' {
            let next = chars.next().unwrap();
            match next {
                '\\' => {}
                '"' => {}
                'x' => {
                    assert!(chars.next().unwrap().is_ascii_hexdigit());
                    assert!(chars.next().unwrap().is_ascii_hexdigit());
                }
                _ => panic!("invalid escape sequence"),
            }
        }

        count += 1;
    }
    count
}

fn solve_part2() -> u16 {
    17
}

fn read_input() -> String {
    fs::read_to_string("input_08.txt").expect("Failed to read input file.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = r#"
            ""
            "abc"
            "aaa\"aaa"
            "\x27"
        "#;
        assert_eq!(solve_part1(input), 12);
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&read_input()), 1350);
    }
}
