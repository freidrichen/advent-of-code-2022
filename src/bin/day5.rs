use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

const SAMPLE_INPUT: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let count: usize = input
        .lines()
        .last()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..count {
        stacks.push(Vec::new());
    }

    for line in input.lines() {
        let chars = line.chars();
        let blocks = chars.skip(1).step_by(4);
        for (i, block) in blocks.enumerate() {
            if block == '1' {
                break;
            }
            if block == ' ' {
                continue;
            }
            stacks[i].push(block)
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}

fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^move (\d+) from (\d+) to (\d+)$").unwrap();
    }
    let mut instructions = Vec::new();
    for cap in RE.captures_iter(input) {
        let count: usize = (&cap[1]).parse().unwrap();
        let from: usize = (&cap[2]).parse::<usize>().unwrap() - 1;
        let to: usize = (&cap[3]).parse::<usize>().unwrap() - 1;
        instructions.push((count, from, to))
    }
    instructions
}

fn trim(input: &str) -> &str {
    input.trim_start_matches("\n").trim_end_matches("\n")
}

fn eprint_stacks(stacks: &Vec<Vec<char>>) {
    for (i, stack) in stacks.iter().enumerate() {
        eprint!("{i} ");
        for block in stack {
            eprint!("{block}");
        }
        eprintln!("");
    }
}

fn part1(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions);

    for (count, from, to) in instructions {
        // eprint_stacks(&stacks);
        // eprintln!("Move {count} from {from} to {to}");
        for _ in 0..count {
            let v = stacks[from].pop().unwrap();
            stacks[to].push(v);
        }
    }
    // eprint_stacks(&stacks);

    let mut res = Vec::new();
    for mut stack in stacks {
        res.push(stack.pop().unwrap().to_string())
    }
    res.join("")
}

fn part2(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions);

    for (count, from, to) in instructions {
        // eprint_stacks(&stacks);
        // eprintln!("Move {count} from {from} to {to}");
        let mut cache = Vec::new();
        for _ in 0..count {
            cache.push(stacks[from].pop().unwrap());
        }
        cache.reverse();
        for v in cache {
            stacks[to].push(v);
        }
    }
    // eprint_stacks(&stacks);

    let mut res = Vec::new();
    for mut stack in stacks {
        res.push(stack.pop().unwrap().to_string())
    }
    res.join("")
}

fn main() {
    let full_input = fs::read_to_string("input/day5").unwrap();

    println!("Part 1 (sample input): {}", part1(trim(SAMPLE_INPUT)));
    println!("Part 1 (full input): {}", part1(trim(&full_input)));

    println!("Part 2 (sample input): {}", part2(trim(SAMPLE_INPUT)));
    println!("Part 2 (full input): {}", part2(trim(&full_input)));
}
