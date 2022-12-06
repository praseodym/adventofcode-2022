use std::collections::HashSet;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let input = input.trim();

    for i in 4..input.len() {
        let mut set = HashSet::new();
        let mut chars = input.chars().skip(i - 4);
        for _ in 0..4 {
            let c = chars.next().unwrap();
            set.insert(c);
        }
        if set.len() == 4 {
            return (i as u32, 0);
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 7);
        assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1093);
        // assert_eq!(part2_answer, 0);
    }
}
