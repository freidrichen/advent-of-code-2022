use std::collections::HashSet;
use std::fs;

const SAMPLE_INPUT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

static PRIORITIES: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn get_doublet(contents: &str) -> char {
    let contents: Vec<_> = contents.chars().collect();
    let len = contents.len();
    assert_eq!(len % 2, 0);
    let compartment1: HashSet<_> = contents[..len / 2].iter().cloned().collect();
    let compartment2: HashSet<_> = contents[len / 2..].iter().cloned().collect();
    let intersection = compartment1.intersection(&compartment2);
    *intersection.last().unwrap()
}

fn priority(c: char) -> u64 {
    1 + PRIORITIES.iter().position(|o| &c == o).unwrap() as u64
}

fn part1(input: &str) -> u64 {
    input.lines().map(get_doublet).map(priority).sum()
}

fn part2(input: &str) -> u64 {
    let mut total_priority = 0;
    let mut lines = input.lines();
    while let Some(rucksack1) = lines.next() {
        let rucksack1: HashSet<_> = rucksack1.chars().collect();
        let rucksack2: HashSet<_> = lines.next().unwrap().chars().collect();
        let rucksack3: HashSet<_> = lines.next().unwrap().chars().collect();
        let badge_item: char = rucksack1
            .intersection(&rucksack2)
            .cloned()
            .collect::<HashSet<_>>()
            .intersection(&rucksack3)
            .next()
            .unwrap()
            .clone();
        total_priority += priority(badge_item);
    }
    total_priority
}

fn main() {
    let full_input = fs::read_to_string("input/day3").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
