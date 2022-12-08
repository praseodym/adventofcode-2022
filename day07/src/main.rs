use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let fs = parse_input(input);

    let part1_answer = fs.values().filter(|&&size| size <= 100000).sum();

    let total = 70000000usize;
    let free = total - fs.get(&PathBuf::from("/")).unwrap();
    let req = 30000000 - free;

    let mut sizes: Vec<usize> = fs.values().copied().collect();
    sizes.sort();
    let part2_answer = *sizes.iter().find(|&&size| size >= req).unwrap();

    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> HashMap<PathBuf, usize> {
    let mut dirs: HashMap<PathBuf, usize> = HashMap::new();
    let mut cwd = PathBuf::from("/");

    for line in input.trim_end().split('\n') {
        let mut s = line.split_whitespace();
        let first = s.next().unwrap();
        match first {
            "$" => {
                let second = s.next().unwrap();
                match second {
                    "cd" => {
                        let third = s.next().unwrap();
                        if third == ".." {
                            cwd = cwd.parent().unwrap().to_path_buf();
                        } else {
                            cwd = cwd.join(third);
                        }
                    }
                    "ls" => (),
                    _ => {
                        panic!("unknown command {} {}", first, second);
                    }
                }
            }
            "dir" => (),
            _ => {
                let _name = s.next().unwrap();
                let size = first.parse::<usize>().unwrap();
                let mut d = cwd.clone();
                loop {
                    dirs.entry(d.clone())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                    let parent = d.parent();
                    if parent.is_none() {
                        break;
                    }
                    d = parent.unwrap().to_path_buf();
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
        let fs = parse_input(include_str!("../input-example"));
        assert_eq!(fs.len(), 4);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 95437);
        assert_eq!(part2_answer, 24933642);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1443806);
        assert_eq!(part2_answer, 942298);
    }
}
