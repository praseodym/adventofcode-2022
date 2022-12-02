fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    //println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let lines = parse_input(input);

    let mut part1_answer: u32 = 0;
    for line in lines {
        part1_answer += match line {
            "A X" => 1 + 3,
            "B X" => 1 + 0,
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Y" => 2 + 3,
            "C Y" => 2 + 0,
            "A Z" => 3 + 0,
            "B Z" => 3 + 6,
            "C Z" => 3 + 3,
            _ => panic!("unknown input: {}", line)
        }
    }

    (part1_answer, 0)
}

fn parse_input(input: &'static str) -> Vec<&str> {
    input.trim_end().split("\n").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let lines = parse_input(include_str!("../input-example"));
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, _) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 15);
    }

    // #[test]
    // fn test_input_answer() {
    //     let (part1_answer, _) = run(include_str!("../input"));
    //     assert_eq!(part1_answer, 67016);
    // }
}
