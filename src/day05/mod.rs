use std::{char, fs};

pub fn part1() {
    println!("AOC 2015 day 5 - part 1");
    println!("Nice strings: {}", count_nice_strings_1(&read_input()));
}

pub fn part2() {
    println!("AOC 2015 day 5 - part 2");
    println!("Nice strings: {}", count_nice_strings_2(&read_input()));
}

fn read_input() -> String {
    fs::read_to_string("input_05.txt").expect("Failed to read input file.")
}

fn count_nice_strings_1(input: &str) -> usize {
    input.lines().filter(|line| is_nice_1(line)).count()
}

fn count_nice_strings_2(input: &str) -> usize {
    input.lines().filter(|line| is_nice_2(line)).count()
}

fn is_nice_1(line: &str) -> bool {
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

    if num_vowels < 3 || !has_double_letter {
        return false;
    }

    let forbidden = ["ab", "cd", "pq", "xy"];
    for s in forbidden {
        if line.contains(s) {
            return false;
        }
    }

    true
}

fn is_nice_2(line: &str) -> bool {
    let mut has_pair = false;
    let chars: Vec<char> = line.chars().collect();

    for i in 0..chars.len() - 1 {
        let pair = &chars[i..i + 2];
        for j in i + 2..chars.len() - 1 {
            if pair == &chars[j..j + 2] {
                has_pair = true;
                break;
            }
        }
        if has_pair {
            break;
        }
    }

    if !has_pair {
        return false;
    }

    let mut has_repeat = false;
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            has_repeat = true;
            break;
        }
    }

    has_repeat
}

fn is_vowel(ch: char) -> bool {
    matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_1_examples() {
        assert!(is_nice_1("ugknbfddgicrmopn"));
        assert!(is_nice_1("aaa"));
        assert!(!is_nice_1("jchzalrnumimnmhp"));
        assert!(!is_nice_1("haegwjzuvuyypxyu"));
        assert!(!is_nice_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(count_nice_strings_1(&read_input()), 255);
    }

    #[test]
    fn test_is_nice_2_examples() {
        assert!(is_nice_2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_2("xxyxx"));
        assert!(!is_nice_2("uurcxstgmygtbstg"));
        assert!(!is_nice_2("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(count_nice_strings_2(&read_input()), 55);
    }
}
