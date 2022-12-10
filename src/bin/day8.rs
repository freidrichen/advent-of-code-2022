use std::fs;

const SAMPLE_INPUT: &str = "
30373
25512
65332
33549
35390
";

struct Forest {
    trees: Vec<u32>,
    width: usize,
    height: usize,
}

impl Forest {
    fn row(&self, y: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        (y * self.width..(y + 1) * self.width).into_iter()
    }

    fn column(&self, x: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        (x..self.width * self.height + x)
            .into_iter()
            .step_by(self.width)
    }
}

fn parse_input(input: &str) -> Forest {
    let mut width = 0;
    let mut trees = Vec::new();
    for line in input.lines() {
        width = line.chars().count();
        trees.extend(line.chars().map(|c| c.to_digit(10).unwrap()));
    }
    let height = trees.len() / width;
    Forest {
        trees,
        width,
        height,
    }
}

fn part1(input: &str) -> usize {
    let forest = parse_input(input);
    let mut seen = vec![false; forest.trees.len()];

    // left to right
    for y in 0..forest.height {
        let mut highest = -1;
        for i in forest.row(y) {
            let tree = forest.trees[i];
            if tree as i32 > highest {
                highest = tree as i32;
                seen[i] = true;
            }
        }
    }
    // right to left
    for y in 0..forest.height {
        let mut highest = -1;
        for i in forest.row(y).rev() {
            let tree = forest.trees[i];
            if tree as i32 > highest {
                highest = tree as i32;
                seen[i] = true;
            }
        }
    }
    // top to bottom
    for x in 0..forest.width {
        let mut highest = -1;
        for i in forest.column(x) {
            let tree = forest.trees[i];
            if tree as i32 > highest {
                highest = tree as i32;
                seen[i] = true;
            }
        }
    }
    // bottom to top
    for x in 0..forest.width {
        let mut highest = -1;
        for i in forest.column(x).rev() {
            let tree = forest.trees[i];
            if tree as i32 > highest {
                highest = tree as i32;
                seen[i] = true;
            }
        }
    }

    seen.into_iter().filter(|&v| v).count()
}

fn add_scenic_factor(
    scenic_scores: &mut Vec<u32>,
    indices: impl DoubleEndedIterator<Item = usize>,
    forest: &Forest,
) {
    let mut viewing_distances = [0; 10];
    for i in indices {
        let tree = forest.trees[i];
        scenic_scores[i] *= viewing_distances[tree as usize];
        for pos in 0..viewing_distances.len() {
            if pos <= tree as usize {
                viewing_distances[pos] = 1;
            } else {
                viewing_distances[pos] += 1;
            }
        }
    }
}

fn part2(input: &str) -> u32 {
    let forest = parse_input(input);
    let mut scenic_scores = vec![1; forest.trees.len()];

    for y in 0..forest.height {
        // left to right
        let indices = forest.row(y);
        add_scenic_factor(&mut scenic_scores, indices, &forest);
    }
    for y in 0..forest.height {
        // right to left
        let indices = forest.row(y).rev();
        add_scenic_factor(&mut scenic_scores, indices, &forest);
    }
    for x in 0..forest.width {
        // top to bottom
        let indices = forest.column(x);
        add_scenic_factor(&mut scenic_scores, indices, &forest);
    }
    for x in 0..forest.width {
        // bottom to top
        let indices = forest.column(x).rev();
        add_scenic_factor(&mut scenic_scores, indices, &forest);
    }

    scenic_scores.into_iter().max().unwrap()
}

fn main() {
    let full_input = fs::read_to_string("input/day8").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
