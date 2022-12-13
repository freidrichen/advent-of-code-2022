use std::fs;

const SAMPLE_INPUT: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

const INTERESTING_CYCLE_COUNTS: [u32; 6] = [20, 60, 100, 140, 180, 220];

struct Cpu {
    x: i32,
    cycle: u32,
    signal_strengths: Vec<i32>,
    screen_output: String,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            x: 1,
            cycle: 0,
            signal_strengths: Vec::new(),
            screen_output: String::new(),
        }
    }

    fn step(&mut self) {
        if INTERESTING_CYCLE_COUNTS.contains(&self.cycle) {
            self.signal_strengths.push(self.x * self.cycle as i32);
        }
        let screen_pos = self.cycle as i32 % 40;
        self.screen_output
            .push_str(if (screen_pos - self.x).abs() <= 1 {
                "#"
            } else {
                "."
            });
        if screen_pos == 39 {
            self.screen_output.push_str("\n");
        }
        self.cycle += 1;
    }
}

fn part1(input: &str) -> i32 {
    let mut cpu = Cpu::new();
    for line in input.lines() {
        if line == "noop" {
            cpu.step();
        } else {
            let (_op, v) = line.split_once(" ").unwrap();
            let v: i32 = v.parse().unwrap();
            cpu.step();
            cpu.step();
            cpu.x += v;
        }
    }
    cpu.signal_strengths.into_iter().sum::<i32>()
}

fn part2(input: &str) -> String {
    let mut cpu = Cpu::new();
    for line in input.lines() {
        if line == "noop" {
            cpu.step();
        } else {
            let (_op, v) = line.split_once(" ").unwrap();
            let v: i32 = v.parse().unwrap();
            cpu.step();
            cpu.step();
            cpu.x += v;
        }
    }
    cpu.screen_output
}

fn main() {
    let full_input = fs::read_to_string("input/day10").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input):\n{}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input)\n{}", part2(full_input.trim()));
}
