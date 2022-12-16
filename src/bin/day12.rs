use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::ops::{Index, IndexMut};

const SAMPLE_INPUT: &str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Pos(usize, usize);

struct Map<V> {
    values: Vec<V>,
    shape: (usize, usize),
}

impl<V> Map<V>
where
    V: Clone,
{
    fn filled(value: V, shape: (usize, usize)) -> Map<V> {
        Map::<V> {
            values: vec![value; shape.0 * shape.1],
            shape,
        }
    }

    fn get_adjacent_indices(&self, pos: Pos) -> Vec<Pos> {
        let mut adjacent = Vec::new();
        if pos.0 > 0 {
            adjacent.push(Pos(pos.0 - 1, pos.1))
        }
        if pos.0 < self.shape.0 - 1 {
            adjacent.push(Pos(pos.0 + 1, pos.1))
        }
        if pos.1 > 0 {
            adjacent.push(Pos(pos.0, pos.1 - 1))
        }
        if pos.1 < self.shape.1 - 1 {
            adjacent.push(Pos(pos.0, pos.1 + 1))
        }
        adjacent
    }

    fn dbg_compact<F>(&self, format_value: F) -> String
    where
        F: Fn(&V) -> char,
    {
        let mut res = String::new();
        for row in 0..self.shape.1 {
            for col in 0..self.shape.0 {
                res.push(format_value(&self[Pos(col, row)]));
            }
            res.push('\n');
        }
        res
    }
}

impl<V> Index<Pos> for Map<V> {
    type Output = V;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.values[pos.0 + self.shape.0 * pos.1]
    }
}

impl<V> IndexMut<Pos> for Map<V> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.values[pos.0 + self.shape.0 * pos.1]
    }
}

#[derive(Debug)]
struct NoPathError;

fn heuristic(start: Pos, goal: Pos, elevations: &Map<usize>) -> usize {
    (goal.0.saturating_sub(start.0) + goal.1.saturating_sub(start.1))
        .max(elevations[goal].saturating_sub(elevations[start]))
}

/// Return length of shortest path through elevation map
fn shortest_path(start: Pos, goal: Pos, elevations: &Map<usize>) -> Result<usize, NoPathError> {
    // A* algorithm adapted from Wikipedia:
    let mut fringe = BinaryHeap::from([Reverse((start, heuristic(start, goal, &elevations)))]);
    let mut g_score = Map::filled(usize::MAX, elevations.shape);
    g_score[start] = 0;

    while let Some(Reverse((current_pos, current_f))) = fringe.pop() {
        // eprintln!("{current_pos:?}");
        if current_pos == goal {
            return Ok(current_f);
        }

        let neighbors = elevations.get_adjacent_indices(current_pos);
        for neighbor in neighbors {
            if elevations[neighbor] > elevations[current_pos] + 1 {
                continue;
            }
            // tentative_g_score is the distance from start to the neighbor through current
            let tentative_g_score = g_score[current_pos] + 1;
            if tentative_g_score < g_score[neighbor] {
                // This path to neighbor is better than any previous one. Record it!
                g_score[neighbor] = tentative_g_score;
                fringe.push(Reverse((
                    neighbor,
                    tentative_g_score + heuristic(neighbor, goal, &elevations),
                )))
            }
        }
        // eprintln!("  next: {fringe:?}");
    }

    // Fringe is empty but goal was never reached
    return Err(NoPathError);
}

fn shortest_paths(goal: Pos, elevations: &Map<usize>) -> Map<usize> {
    // Dijstras algorithm adapted from the A* algorithm above:
    let mut fringe = BinaryHeap::from([Reverse((0, goal))]);
    let mut g_score = Map::filled(usize::MAX, elevations.shape);
    let mut visited = Map::filled(false, elevations.shape);
    g_score[goal] = 0;

    while let Some(Reverse((_current_f, current_pos))) = fringe.pop() {
        visited[current_pos] = true;
        // eprintln!("{current_pos:?}");

        // Uncomment this debug output for a pretty cool "animation" of the search
        // eprintln!("{}", visited.dbg_compact(|&b| if b { '#' } else { '.' }));

        let neighbors = elevations.get_adjacent_indices(current_pos);
        for neighbor in neighbors {
            if visited[neighbor] || elevations[neighbor] + 1 < elevations[current_pos] {
                continue;
            }
            // tentative_g_score is the distance from start to the neighbor through current
            let tentative_g_score = g_score[current_pos] + 1;
            if tentative_g_score < g_score[neighbor] {
                // This path to neighbor is better than any previous one. Record it!
                g_score[neighbor] = tentative_g_score;
                fringe.push(Reverse((tentative_g_score, neighbor)))
            }
        }
        // eprintln!("  next: {fringe:?}");
    }

    return g_score;
}

struct AllTheData {
    elevations: Map<usize>,
    start: Pos,
    goal: Pos,
}

fn parse_elevations(input: &str) -> AllTheData {
    let mut elevations = Vec::new();
    let mut start = None;
    let mut goal = None;
    let mut shape = None;
    for (row, line) in input.lines().enumerate() {
        shape = Some((line.len(), input.len() / line.len()));
        for (col, c) in line.chars().enumerate() {
            elevations.push(if c == 'S' {
                start = Some(Pos(col, row));
                0
            } else if c == 'E' {
                goal = Some(Pos(col, row));
                (b'z' - b'a') as usize
            } else {
                c as usize - b'a' as usize
            })
        }
    }
    AllTheData {
        elevations: Map {
            values: elevations,
            shape: shape.unwrap(),
        },
        start: start.unwrap(),
        goal: goal.unwrap(),
    }
}

fn part1(input: &str) -> usize {
    let AllTheData {
        elevations,
        start,
        goal,
    } = parse_elevations(input);
    shortest_path(start, goal, &elevations).unwrap()
}

fn part2(input: &str) -> usize {
    let AllTheData {
        elevations,
        start: _,
        goal,
    } = parse_elevations(input);
    shortest_paths(goal, &elevations)
        .values
        .into_iter()
        .zip(elevations.values)
        .filter_map(|(p, e)| if e == 0 { Some(p) } else { None })
        .min()
        .unwrap()
}

fn main() {
    let full_input = fs::read_to_string("input/day12").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
