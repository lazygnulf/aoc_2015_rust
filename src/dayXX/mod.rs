use crate::util::read_input;

static DAY: u32 = XX;

pub fn solve() {
    let input = read_input(DAY);
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input))
}

fn solve_part1(_input: &str) -> usize {
    17
}

fn solve_part2(_input: &str) -> usize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&read_input(DAY)), 17);
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&read_input(DAY)), 42);
    }
}
