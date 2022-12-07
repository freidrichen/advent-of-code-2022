use nom::{
    bytes::complete::tag, character::complete, character::complete::newline,
    multi::separated_list1, sequence::separated_pair, IResult,
};
use std::fs;

const SAMPLE_INPUT: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

struct Range {
    start: u32,
    end: u32,
}

fn full_overlap(a: &Range, b: &Range) -> bool {
    a.start <= b.start && a.end >= b.end || a.start >= b.start && a.end <= b.end
}
fn overlap(a: &Range, b: &Range) -> bool {
    a.start <= b.start && a.end >= b.start || a.start >= b.start && a.start <= b.end
}

fn range(input: &str) -> IResult<&str, Range> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
    Ok((input, Range { start, end }))
}
fn range_pair(input: &str) -> IResult<&str, (Range, Range)> {
    let (input, (range_a, range_b)) = separated_pair(range, tag(","), range)(input)?;
    Ok((input, (range_a, range_b)))
}
fn range_pairs(input: &str) -> IResult<&str, Vec<(Range, Range)>> {
    let (input, res) = separated_list1(newline, range_pair)(input)?;
    Ok((input, res))
}

fn part1(input: &str) -> usize {
    let (_input, assignments) = range_pairs(input).unwrap();
    assignments
        .into_iter()
        .filter(|(a, b)| full_overlap(a, b))
        .count()
}

fn part2(input: &str) -> usize {
    let (_input, assignments) = range_pairs(input).unwrap();
    assignments
        .into_iter()
        .filter(|(a, b)| overlap(a, b))
        .count()
}

fn main() {
    let full_input = fs::read_to_string("input/day4").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
