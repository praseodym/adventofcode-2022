use std::collections::HashSet;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer: u32 = 0;
    let part2_answer: u32 = 0;

    let rucksacks = parse_input(input);
    for r in rucksacks {
        let err = r.0.intersection(&r.1).next().unwrap();
        part1_answer += get_priority(*err);
    }

    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut rucksacks = Vec::new();
    for rucksack in input.trim_end().split_whitespace() {
        let r = rucksack.to_string();
        let n = r.len()/2;
        let mut c = r.chars();
        let r1 = c.by_ref().take(n).collect();
        let r2 = c.by_ref().take(n).collect();
        rucksacks.push((r1, r2));
    }
    rucksacks
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96
    } else {
        (c as u32) - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let rucksacks = parse_input(include_str!("../input-example"));
        assert_eq!(rucksacks.len(), 6);
    }

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('L'), 38);
        assert_eq!(get_priority('P'), 42);
        assert_eq!(get_priority('v'), 22);
        assert_eq!(get_priority('t'), 20);
        assert_eq!(get_priority('s'), 19);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 157);
        assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 7997);
        // assert_eq!(part2_answer, 0);
    }
}
