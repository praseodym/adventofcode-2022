use std::collections::HashMap;
use std::path::Path;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer: usize = 0;
    let mut part2_answer: usize = 0;

    let fs = parse_input(input);

    // println!("fs: {:?}", fs);
    part1_answer = fs.values().filter(|size| **size <= 100000).sum();

    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> HashMap<String, usize> {
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut cwd = "/".to_string();

    for line in input.trim_end().split('\n') {
        // println!("line: {}", line);
        let mut s = line.split_whitespace();
        let first = s.next().unwrap();
        match first {
            "$" => {
                let second = s.next().unwrap();
                match second {
                    "cd" => {
                        let third = s.next().unwrap();
                        if third == ".." {
                            cwd = Path::new(&cwd)
                                .parent()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();
                        } else {
                            cwd = Path::new(&cwd).join(third).to_str().unwrap().to_string();
                        }
                    }
                    "ls" => {
                        // ignore
                    }
                    _ => {
                        panic!("unknown command {} {}", first, second);
                    }
                }
            }
            "dir" => {
                // ignore
            }
            _ => {
                let name = s.next().unwrap();
                let size = first.parse::<usize>().unwrap();
                let mut d = Path::new(&cwd);
                loop {
                    dirs.entry(d.to_str().unwrap().to_string())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                    let parent = d.parent();
                    if parent.is_none() {
                        break;
                    }
                    d = parent.unwrap();
                }
            }
        }
    }

    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let lines = parse_input(include_str!("../input-example"));
        // assert_eq!(lines.len(), 23);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 95437);
        assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1443806);
        // assert_eq!(part2_answer, 0);
    }
}
