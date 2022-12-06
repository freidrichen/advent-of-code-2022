use std::collections::HashSet;
use std::fs;

const SAMPLE_INPUT: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

fn parse_sections(assignment: &str) -> HashSet<u64> {
    let (start, end) = assignment.split_once("-").unwrap();
    let start: u64 = start.parse().unwrap();
    let end: u64 = end.parse().unwrap();
    (start..=end).collect()
}

fn full_overlap(line: &str) -> bool {
    let (assign1, assign2) = line.split_once(",").unwrap();
    let sections1 = parse_sections(assign1);
    let sections2 = parse_sections(assign2);
    sections1.is_subset(&sections2) || sections2.is_subset(&sections1)
}

fn part1(input: &str) -> usize {
    input.lines().filter(|&l| full_overlap(l)).count()
}

fn overlap(line: &str) -> bool {
    let (assign1, assign2) = line.split_once(",").unwrap();
    let sections1 = parse_sections(assign1);
    let sections2 = parse_sections(assign2);
    !sections1.is_disjoint(&sections2)
}

fn part2(input: &str) -> usize {
    input.lines().filter(|&l| overlap(l)).count()
}

fn main() {
    let full_input = fs::read_to_string("input/day4").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
