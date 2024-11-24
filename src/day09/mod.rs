use std::{collections::HashMap, fs, ops::Index, usize};

use itertools::Itertools;

pub fn part1() {
    println!("AOC 2015 day 9 - part 1");
    println!("{}", solve_part1(&read_input()));
}

pub fn part2() {
    println!("AOC 2015 day 9 - part 2");
    println!("{}", solve_part2(&read_input()));
}

fn solve_part1(input: &str) -> usize {
    let (locations, distances) = parse_input(input);
    println!("{:?}\n{:?}", locations, distances);

    let mut location_vec = vec![0usize; locations.len()];
    for i in 0..locations.len() {
        location_vec[i] = i;
    }

    let mut shortest_tour_len = usize::MAX;
    for tour in location_vec.iter().permutations(location_vec.len()) {
        println!("{:?}", tour);

        let mut tour_len: usize = 0;
        for loc in tour.windows(2) {
            tour_len += distances[*loc[0]][*loc[1]];
        }
        if shortest_tour_len > tour_len {
            shortest_tour_len = tour_len;
        }
    }

    shortest_tour_len
}

fn solve_part2(input: &str) -> usize {
    42
}

fn parse_input(input: &str) -> (HashMap<&str, usize>, Vec<Vec<usize>>) {
    let mut locations = HashMap::new();

    let mut idx: usize = 0;

    for line in input.lines().map(|l| l.trim()) {
        let distance_parts: Vec<&str> = line.split(" = ").collect();
        let location_parts: Vec<&str> = distance_parts[0].split(" to ").collect();
        let from = location_parts[0];
        let to = location_parts[1];
        if !locations.contains_key(from) {
            locations.insert(from, idx);
            idx += 1;
        }
        if !locations.contains_key(to) {
            locations.insert(to, idx);
            idx += 1;
        }
    }

    let size = locations.len();

    let mut distances = vec![vec![0; size]; size];

    for line in input.lines().map(|l| l.trim()) {
        let distance_parts: Vec<&str> = line.split(" = ").collect();
        let distance: usize = distance_parts[1].parse().unwrap();
        let location_parts: Vec<&str> = distance_parts[0].split(" to ").collect();
        let from = location_parts[0];
        let to = location_parts[1];
        let from_idx = locations.get(from).unwrap();
        let to_idx = locations.get(to).unwrap();
        distances[*from_idx][*to_idx] = distance;
        distances[*to_idx][*from_idx] = distance;
    }

    (locations, distances)
}

fn read_input() -> String {
    fs::read_to_string("input_09.txt").expect("Failed to read input file.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = r#"London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141"#;

        assert_eq!(solve_part1(input), 605);
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&read_input()), 207);
    }

    #[test]
    fn test_part2_with_input() {}
}
