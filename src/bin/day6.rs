use std::collections::{HashSet, VecDeque};
use std::fs;

const SAMPLE_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

fn find_marker(input: &str, marker_size: usize) -> Option<usize> {
    let mut buffer = VecDeque::with_capacity(marker_size);
    let mut chars = input.chars();
    for _ in 0..marker_size {
        buffer.push_back(chars.next().unwrap());
    }
    for (i, c) in chars.enumerate() {
        let unique_chars: HashSet<_> = buffer.iter().cloned().collect();
        if unique_chars.len() == marker_size {
            return Some(i + marker_size);
        }
        buffer.pop_front();
        buffer.push_back(c);
    }
    None
}

fn part1(input: &str) -> usize {
    find_marker(input, 4).unwrap_or_else(|| panic!("No start-of-packet marker found!"))
}

fn part2(input: &str) -> usize {
    find_marker(input, 14).unwrap_or_else(|| panic!("No start-of-message marker found!"))
}

fn main() {
    let full_input = fs::read_to_string("input/day6").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
