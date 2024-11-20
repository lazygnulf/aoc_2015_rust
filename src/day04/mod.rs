use md5;

pub fn part1() {
    println!("AOC 2015 day 4 - part 1");
    println!(
        "AdventCoin number: {}",
        find_first_with_leading_zeros(&read_input(), 5)
    );
}

pub fn part2() {
    println!("AOC 2015 day 4 - part 2");
    println!(
        "AdventCoin number: {}",
        find_first_with_leading_zeros(&read_input(), 6)
    );
}

fn read_input() -> String {
    "ckczppom".to_owned()
}

fn find_first_with_leading_zeros(input: &str, leading_zeros: usize) -> u32 {
    let zeros = "0".repeat(leading_zeros);
    let mut number = 0;

    loop {
        let data = format!("{}{}", input, number);
        if format!("{:x}", md5::compute(data)).starts_with(&zeros) {
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
        assert_eq!(find_first_with_leading_zeros("abcdef", 5), 609043);
        assert_eq!(find_first_with_leading_zeros("pqrstuv", 5), 1048970);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(find_first_with_leading_zeros(&read_input(), 5), 117946);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(find_first_with_leading_zeros(&read_input(), 6), 3938038);
    }
}
