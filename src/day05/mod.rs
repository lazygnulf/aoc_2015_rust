use std::{char, fs};

pub fn part1() {
    println!("AOC 2015 day 5 - part 1");
    println!("Nice strings: {}", count_nice_strings(&read_input()));

    read_input();
}

fn read_input() -> String {
    fs::read_to_string("input_05.txt").expect("Failed to read input file.")
}

fn count_nice_strings(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        if is_nice(line) {
            count += 1;
        }
    }
    count
}

fn is_nice(line: &str) -> bool {
    let mut num_vowels = 0;

    let mut has_double_letter = false;
    let mut last_char: char = ' ';

    for (i, ch) in line.chars().enumerate() {
        if is_vowel(ch) {
            num_vowels += 1;
        }
        if i > 0 && last_char == ch {
            has_double_letter = true;
        }

        last_char = ch;
    }

    let contains_forbidden =
        line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy");

    num_vowels >= 3 && has_double_letter && !contains_forbidden
}

fn is_vowel(ch: char) -> bool {
    matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_examples() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(count_nice_strings(&read_input()), 255);
    }
}
