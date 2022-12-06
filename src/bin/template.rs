use std::fs;

const SAMPLE_INPUT: &str = "
";

fn part1(input: &str) -> u64 {
    0
}

fn part2(input: &str) -> u64 {
    0
}

fn main() {
    let full_input = fs::read_to_string("input/day?").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
