use std::collections::HashMap;
use std::fs;

const SAMPLE_INPUT: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

fn get_dir_sizes(input: &str) -> HashMap<String, u64> {
    let mut cd = vec![""];
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    for line in input.lines() {
        // eprintln!("{}", line);
        if line.starts_with("$ cd") {
            // change current directory
            if let Some((_cmd, dir_name)) = line.rsplit_once(" ") {
                match dir_name {
                    "/" => {
                        cd = vec![""];
                    }
                    ".." => {
                        cd.pop();
                    }
                    _ => {
                        cd.push(dir_name);
                    }
                };
            }
        } else if line.starts_with("$ ls") {
            // TODO: sanity check?
        } else if let Some((size, _filename)) = line.split_once(" ") {
            if size == "dir" {
                continue;
            }
            // Add entries for this file in current dir and all parent dirs
            for end in 1..=cd.len() {
                *dir_sizes.entry(cd[0..end].join("/")).or_insert(0) += size.parse::<u64>().unwrap();
            }
        }
    }
    dir_sizes
}

fn part1(input: &str) -> u64 {
    let dir_sizes = get_dir_sizes(input);

    let mut res = 0;
    for (_dir_name, size) in dir_sizes {
        if size <= 100000 {
            res += size;
        }
    }
    res
}

fn part2(input: &str) -> u64 {
    let dir_sizes = get_dir_sizes(input);

    const ALLOWED_USED_SPACE: u64 = 40_000_000;
    let used_space = dir_sizes[""];
    let min_removed_dirsize = used_space - ALLOWED_USED_SPACE;

    let mut dir_sizes: Vec<_> = dir_sizes.into_values().collect();
    dir_sizes.sort_unstable();
    for size in dir_sizes {
        if size >= min_removed_dirsize {
            return size;
        }
    }
    panic!("No directory suitable for deletion found!");
}

fn main() {
    let full_input = fs::read_to_string("input/day7").unwrap();

    println!("Part 1 (sample input): {}", part1(SAMPLE_INPUT.trim()));
    println!("Part 1 (full input): {}", part1(full_input.trim()));

    println!("Part 2 (sample input): {}", part2(SAMPLE_INPUT.trim()));
    println!("Part 2 (full input): {}", part2(full_input.trim()));
}
