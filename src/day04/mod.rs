use md5;

pub fn part1() {
    println!("AOC 2015 day 4 - part 1");
    println!("AdventCoin number: {}", solve_part_1(&read_input()));
}

fn read_input() -> String {
    "ckczppom".to_owned()
}

fn solve_part_1(input: &str) -> u32 {
    let mut number = 0;

    loop {
        let data = format!("{}{}", input, number);
        if format!("{:x}", md5::compute(data)).starts_with("00000") {
            break;
        }
        number += 1;
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(solve_part_1("abcdef"), 609043)
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(solve_part_1(&read_input()), 117946);
    }
}
