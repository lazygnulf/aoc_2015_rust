use std::fs;

pub fn part1() {
    println!("AOC 2015 day 6 - part 1");
    let instructions = parse_input(&read_input());
    println!("Lights on: {}", exec_instructions(&instructions));
}

pub fn part2() {
    println!("AOC 2015 day 6 - part 2");
    let instructions = parse_input(&read_input());
    println!("Total brightness: {}", exec_instructions2(&instructions));
}

fn read_input() -> String {
    fs::read_to_string("input_06.txt").expect("Failed to read input file.")
}

fn exec_instructions(instructions: &Vec<Instruction>) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];

    for instr in instructions {
        for x in instr.x1..=instr.x2 {
            for y in instr.y1..=instr.y2 {
                match instr.cmd {
                    Command::TurnOn => grid[x][y] = true,
                    Command::TurnOff => grid[x][y] = false,
                    Command::Toogle => grid[x][y] = !grid[x][y],
                }
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|&&v| v).count())
        .sum()
}

fn exec_instructions2(instructions: &Vec<Instruction>) -> usize {
    let mut grid = vec![vec![0usize; 1000]; 1000];

    for instr in instructions {
        for x in instr.x1..=instr.x2 {
            for y in instr.y1..=instr.y2 {
                match instr.cmd {
                    Command::TurnOn => grid[x][y] += 1,
                    Command::TurnOff => {
                        if grid[x][y] > 0 {
                            grid[x][y] -= 1
                        }
                    }
                    Command::Toogle => grid[x][y] += 2,
                }
            }
        }
    }

    grid.iter().map(|row| row.iter().sum::<usize>()).sum()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut rect_pos = 2;

        let command;
        if line.starts_with("turn on") {
            command = Command::TurnOn
        } else if line.starts_with("turn off") {
            command = Command::TurnOff;
        } else if line.starts_with("toggle") {
            command = Command::Toogle;
            rect_pos = 1;
        } else {
            panic!("Parse error: unknown command in {}.", line);
        }

        let start: Vec<usize> = parts[rect_pos]
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        let end: Vec<usize> = parts[rect_pos + 2]
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        instructions.push(Instruction {
            cmd: command,
            x1: start[0],
            x2: end[0],
            y1: start[1],
            y2: end[1],
        });
    }

    instructions
}

enum Command {
    TurnOn,
    TurnOff,
    Toogle,
}

struct Instruction {
    cmd: Command,
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_input() {
        assert_eq!(exec_instructions(&parse_input(&read_input())), 569999);
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(exec_instructions2(&parse_input(&read_input())), 17836115);
    }
}
