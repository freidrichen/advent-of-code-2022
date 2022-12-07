use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{alpha1, newline, space1},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
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

fn krate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((tag("   "), delimited(tag("["), alpha1, tag("]"))))(input)?;
    let c = match c {
        "   " => None,
        c => Some(c),
    };
    Ok((input, c))
}
fn crates_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), krate)(input)
}
fn crates_lines(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>> {
    separated_list1(newline, crates_line)(input)
}
fn stacks(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, stacks_lines) = crates_lines(input)?;

    let mut stacks: Vec<Vec<&str>> = Vec::new();
    for stacks_line in stacks_lines {
        if stacks.is_empty() {
            for _ in 0..stacks_line.len() {
                stacks.push(Vec::new());
            }
        }
        for (i, krate) in stacks_line
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.map(|c| (i, c)))
        {
            stacks[i].push(krate)
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }
    Ok((input, stacks))
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag(" ")(input)?;
    let (input, res) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = tag(" ")(input)?;
    Ok((input, res))
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;
    Ok((
        input,
        Instruction {
            count: count as usize,
            from: from as usize - 1,
            to: to as usize - 1,
        },
    ))
}
fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, instruction)(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Instruction>)> {
    let (input, stacks) = stacks(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = numbers(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, instructions) = instructions(input)?;
    Ok((input, (stacks, instructions)))
}

fn trim(input: &str) -> &str {
    input.trim_start_matches("\n").trim_end_matches("\n")
}

// fn eprint_stacks(stacks: &Vec<Vec<&str>>) {
//     for (i, stack) in stacks.iter().enumerate() {
//         eprint!("{i} ");
//         for block in stack {
//             eprint!("{block}");
//         }
//         eprintln!("");
//     }
// }

fn part1(input: &str) -> String {
    let (_input, (mut stacks, instructions)) = parse(input).unwrap();

    for Instruction { count, from, to } in instructions {
        // eprint_stacks(&stacks);
        // eprintln!("Move {count} from {from} to {to}");
        for _ in 0..count {
            let v = stacks[from].pop().unwrap();
            stacks[to].push(v);
        }
    }

    let mut res = Vec::new();
    for mut stack in stacks {
        res.push(stack.pop().unwrap().to_string())
    }
    res.join("")
}

fn part2(input: &str) -> String {
    let (_input, (mut stacks, instructions)) = parse(input).unwrap();

    for Instruction { count, from, to } in instructions {
        // eprint_stacks(&stacks);
        // eprintln!("Move {count} from {from} to {to}");
        let len = stacks[from].len();
        let cache: Vec<_> = stacks[from].drain(len - count..).collect();
        stacks[to].extend(cache);
    }

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
