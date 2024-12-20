use std::collections::HashMap;

use crate::util::read_input;

pub fn solve() {
    let input = read_input(3);
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> usize {
    let mut house_visited = HashMap::new();
    let mut pos = (0, 0);
    house_visited.insert(pos, 1);

    for ch in input.chars() {
        pos = move_pos(pos, ch);
        match house_visited.get(&pos) {
            Some(x) => house_visited.insert(pos, x + 1),
            None => house_visited.insert(pos, 1),
        };
    }

    house_visited.len()
}

fn solve_part_2(input: &str) -> usize {
    let mut house_visited = HashMap::new();
    let mut pos = (0, 0);
    let mut robo = (0, 0);
    let mut robo_turn = false;
    house_visited.insert(pos, 2);

    for ch in input.chars() {
        if robo_turn {
            robo = move_pos(robo, ch);
            match house_visited.get(&robo) {
                Some(x) => house_visited.insert(robo, x + 1),
                None => house_visited.insert(robo, 1),
            };
        } else {
            pos = move_pos(pos, ch);
            match house_visited.get(&pos) {
                Some(x) => house_visited.insert(pos, x + 1),
                None => house_visited.insert(pos, 1),
            };
        }
        robo_turn = !robo_turn;
    }

    house_visited.len()
}

fn move_pos(pos: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        '>' => (pos.0 + 1, pos.1),
        '<' => (pos.0 - 1, pos.1),
        '^' => (pos.0, pos.1 - 1),
        'v' => (pos.0, pos.1 + 1),
        _ => pos,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_input() {
        assert_eq!(solve_part_1(&read_input(3)), 2081);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(solve_part_2(&read_input(3)), 2341);
    }
}
