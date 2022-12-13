use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::newline,
    combinator::opt,
    multi::{count, separated_list1},
    IResult,
};
use std::fs;

const SAMPLE_INPUT: &str = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn eval(&self, old: u64) -> u64 {
        match self {
            &Operation::Square => old * old,
            &Operation::Multiply(n) => old * n,
            &Operation::Add(n) => old + n,
        }
    }
}

struct Monkey {
    inspections: u64,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    true_target: usize,
    false_target: usize,
}

fn items(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(", "), complete::u64)(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("new = old ")(input)?;
    if let (input, Some(_)) = opt(tag("* old"))(input)? {
        Ok((input, Operation::Square))
    } else {
        let (input, op) = alt((tag("+ "), tag("* ")))(input)?;
        let (input, n) = complete::u64(input)?;
        match op {
            "+ " => Ok((input, Operation::Add(n))),
            "* " => Ok((input, Operation::Multiply(n))),
            _ => panic!("incorrect operator: {op}"),
        }
    }
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _monkey_id) = complete::u32(input)?;
    let (input, _) = tag(":\n  Starting items: ")(input)?;
    let (input, items) = items(input)?;
    let (input, _) = tag("\n  Operation: ")(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = tag("\n  Test: divisible by ")(input)?;
    let (input, test) = complete::u64(input)?;
    let (input, _) = tag("\n    If true: throw to monkey ")(input)?;
    let (input, true_target) = complete::u32(input)?;
    let (input, _) = tag("\n    If false: throw to monkey ")(input)?;
    let (input, false_target) = complete::u32(input)?;
    Ok((
        input,
        Monkey {
            inspections: 0,
            items,
            operation,
            test,
            true_target: true_target as usize,
            false_target: false_target as usize,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(count(newline, 2), monkey)(input)
}

fn part1(input: &str) -> u64 {
    let (_input, mut monkeys) = parse_monkeys(input).unwrap();
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let items: Vec<_> = monkeys[i].items.drain(..).collect();
            for mut item in items {
                monkeys[i].inspections += 1;
                item = monkeys[i].operation.eval(item) / 3;
                let target = if item % monkeys[i].test == 0 {
                    monkeys[i].true_target
                } else {
                    monkeys[i].false_target
                };
                monkeys[target].items.push(item);
            }
        }
    }
    let mut inspections = monkeys
        .into_iter()
        .map(|m| m.inspections)
        .collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn part2(input: &str) -> u64 {
    let (_input, mut monkeys) = parse_monkeys(input).unwrap();
    let tests_product: u64 = monkeys.iter().map(|m| m.test).product();
    eprintln!("tests_product: {tests_product}");
    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            let items: Vec<_> = monkeys[i].items.drain(..).collect();
            for mut item in items {
                monkeys[i].inspections += 1;
                item = monkeys[i].operation.eval(item) % tests_product;
                // dbg!(item);
                let target = if item % monkeys[i].test == 0 {
                    monkeys[i].true_target
                } else {
                    monkeys[i].false_target
                };
                monkeys[target].items.push(item);
            }
        }
    }
    let mut inspections = monkeys
        .into_iter()
        .map(|m| m.inspections)
        .collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn main() {
    let full_input = fs::read_to_string("input/day11").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
