use std::{collections::HashMap, fs};

pub fn part1() {
    println!("AOC 2015 day 7 - part 1");
    println!("Wire a : {}", solve_part1());
}

pub fn part2() {
    println!("AOC 2015 day 7 - part 2");
    println!("Wire a : {}", solve_part2());
}

fn solve_part1() -> u16 {
    let mut circuit = parse_input(&read_input());
    let mut cache = HashMap::new();
    eval("a", &mut circuit, &mut cache)
}

fn solve_part2() -> u16 {
    let mut circuit = parse_input(&read_input());
    let mut cache = HashMap::new();
    circuit.insert("b".to_owned(), Instruction::Signal(solve_part1()));
    eval("a", &mut circuit, &mut cache)
}

fn read_input() -> String {
    fs::read_to_string("input_07.txt").expect("Failed to read input file.")
}

type Circuit = HashMap<String, Instruction>;

#[derive(Debug, Clone)]
enum Instruction {
    Signal(u16),
    Wire(String),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(String, u16),
    RShift(String, u16),
    Not(Operand),
}

#[derive(Debug, Clone)]
enum Operand {
    Value(u16),
    Wire(String),
}

fn parse_operand(s: &str) -> Operand {
    if let Ok(value) = s.parse() {
        Operand::Value(value)
    } else {
        Operand::Wire(s.to_string())
    }
}

fn parse_input(input: &str) -> HashMap<String, Instruction> {
    let mut circuit: Circuit = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let target_wire = parts[1].to_string();

        let instr = if let Some((left, right)) = parts[0].split_once(" AND ") {
            Instruction::And(parse_operand(left), parse_operand(right))
        } else if let Some((left, right)) = parts[0].split_once(" OR ") {
            Instruction::Or(parse_operand(left), parse_operand(right))
        } else if let Some((left, right)) = parts[0].split_once(" LSHIFT ") {
            Instruction::LShift(left.to_string(), right.parse().unwrap())
        } else if let Some((left, right)) = parts[0].split_once(" RSHIFT ") {
            Instruction::RShift(left.to_string(), right.parse().unwrap())
        } else if let Some(right) = parts[0].strip_prefix("NOT ") {
            Instruction::Not(parse_operand(right))
        } else if let Ok(value) = parts[0].parse() {
            Instruction::Signal(value)
        } else {
            Instruction::Wire(parts[0].to_string())
        };

        circuit.insert(target_wire, instr);
    }

    circuit
}

fn eval_operand(op: &Operand, circuit: &Circuit, cache: &mut HashMap<String, u16>) -> u16 {
    match op {
        Operand::Value(value) => *value,
        Operand::Wire(wire) => eval(wire, circuit, cache),
    }
}

fn eval(wire: &str, circuit: &Circuit, cache: &mut HashMap<String, u16>) -> u16 {
    if let Some(&value) = cache.get(wire) {
        return value;
    }

    let value = match circuit.get(wire).expect("No wire") {
        Instruction::Signal(value) => *value,
        Instruction::Wire(input_wire) => eval(input_wire, circuit, cache),
        Instruction::And(left, right) => {
            let left_value = eval_operand(left, circuit, cache);
            let right_value = eval_operand(right, circuit, cache);
            left_value & right_value
        }
        Instruction::Or(left, right) => {
            let left_value = eval_operand(left, circuit, cache);
            let right_value = eval_operand(right, circuit, cache);
            left_value | right_value
        }
        Instruction::LShift(left, shift) => {
            let left_value = eval(left, circuit, cache);
            left_value << shift
        }
        Instruction::RShift(left, shift) => {
            let left_value = eval(left, circuit, cache);
            left_value >> shift
        }
        Instruction::Not(input_wire) => {
            let input_value = eval_operand(input_wire, circuit, cache);
            !input_value
        }
    };

    cache.insert(wire.to_string(), value);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(), 3176);
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(), 14710);
    }
}
