use std::fs;

const SAMPLE_INPUT: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

fn part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|n| n.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let mut calories: Vec<_> = input
        .split("\n\n")
        .map(|s| s.lines().map(|n| n.parse::<u64>().unwrap()).sum())
        .collect();
    calories.sort_by_key(|&e| -(e as i64));
    calories.iter().take(3).sum()
}

fn main() {
    let full_input = fs::read_to_string("input/day1").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
