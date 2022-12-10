use nom::{
    bytes::complete::tag, character::complete, character::complete::newline,
    multi::separated_list1, IResult,
};
use std::collections::HashSet;
use std::fs;

const SAMPLE_INPUT: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_steps(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
    separated_list1(newline, step)(input)
}
fn step(input: &str) -> IResult<&str, (Direction, u32)> {
    let (input, dir) = complete::anychar(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, count) = complete::u32(input)?;
    let dir = match dir {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction code: {dir}"),
    };
    Ok((input, (dir, count)))
}

fn part1(input: &str) -> usize {
    let (_, steps) = parse_steps(input).unwrap();
    // TODO: Rewrite part1 using the more general solution for part2
    let mut hx: i32 = 0;
    let mut hy: i32 = 0;
    let mut tx: i32 = 0;
    let mut ty: i32 = 0;
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    for (dir, count) in steps {
        for _ in 0..count {
            match dir {
                Direction::Up => hy -= 1,
                Direction::Down => hy += 1,
                Direction::Left => hx -= 1,
                Direction::Right => hx += 1,
            };
            let dx = hx - tx;
            let dy = hy - ty;
            if dx.abs() > 1 || dy.abs() > 1 {
                tx += dx.signum();
                ty += dy.signum();
            }
            tail_positions.insert((tx, ty));
            // eprintln!("({tx}, {ty})");
        }
    }
    tail_positions.len()
}

fn part2(input: &str) -> usize {
    let (_, steps) = parse_steps(input).unwrap();
    let rope_length = 10;
    let mut x_positions: Vec<i32> = vec![0; rope_length];
    let mut y_positions: Vec<i32> = vec![0; rope_length];
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    for (dir, count) in steps {
        for _ in 0..count {
            match dir {
                Direction::Up => y_positions[0] -= 1,
                Direction::Down => y_positions[0] += 1,
                Direction::Left => x_positions[0] -= 1,
                Direction::Right => x_positions[0] += 1,
            };
            for i in 1..x_positions.len() {
                let dx = x_positions[i - 1] - x_positions[i];
                let dy = y_positions[i - 1] - y_positions[i];
                if dx.abs() > 1 || dy.abs() > 1 {
                    x_positions[i] += dx.signum();
                    y_positions[i] += dy.signum();
                }
            }
            tail_positions.insert((x_positions[rope_length - 1], y_positions[rope_length - 1]));
            // eprintln!("({tx}, {ty})");
        }
    }
    tail_positions.len()
}

fn main() {
    let full_input = fs::read_to_string("input/day9").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
