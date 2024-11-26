use crate::util::read_input;

pub fn solve() {
    let (paper, ribbon) = compute_wrapping_needs(&read_input(2));
    println!("Part 1: {}", paper);
    println!("Part 2: {}", ribbon);
}

fn compute_wrapping_needs(input: &str) -> (u32, u32) {
    let mut wrapping_paper = 0;
    let mut ribbon = 0;
    for line in input.lines() {
        let present: Vec<u32> = line.split('x').map(|v| v.parse::<u32>().unwrap()).collect();

        let l = present[0];
        let w = present[1];
        let h = present[2];

        let sides = [l * w, l * h, w * h];
        wrapping_paper += 2 * sides[0] + 2 * sides[1] + 2 * sides[2] + sides.iter().min().unwrap();

        let perimeters = [2 * (l + w), 2 * (l + h), 2 * (w + h)];
        ribbon += perimeters.iter().min().unwrap() + l * w * h;
    }

    (wrapping_paper, ribbon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = read_input(2);
        let (paper, ribbon) = compute_wrapping_needs(&input);
        assert_eq!(paper, 1586300);
        assert_eq!(ribbon, 3737498);
    }
}
