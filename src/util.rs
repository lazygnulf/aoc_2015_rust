use std::fs;

pub fn read_input(day: u32) -> String {
    fs::read_to_string(format!("input_{:0>2}.txt", day)).expect("Failed to read input file.")
}
