fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer: u32 = 0;
    let mut part2_answer: u32 = 0;

    let lines = parse_input(input);
    for line in lines {
        let mut pair = line.split(',');
        let p1 = pair.next().unwrap();
        let p2 = pair.next().unwrap();
        let mut p1s = p1.split('-');
        let p1a = p1s.next().unwrap().parse::<u32>().unwrap();
        let p1b = p1s.next().unwrap().parse::<u32>().unwrap();
        let mut p2s = p2.split('-');
        let p2a = p2s.next().unwrap().parse::<u32>().unwrap();
        let p2b = p2s.next().unwrap().parse::<u32>().unwrap();

        if p1a >= p2a && p1b <= p2b || p2a >= p1a && p2b <= p1b {
            part1_answer += 1;
        }

        if (p1a >= p2a && p1a <= p2b)
            || (p1b >= p2a && p1b <= p2b)
            || (p2a >= p1a && p2a <= p1b)
            || (p2b >= p1a && p2b <= p1b)
        {
            part2_answer += 1;
        }
    }

    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> Vec<&'static str> {
    input.trim_end().split('\n').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let lines = parse_input(include_str!("../input-example"));
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 2);
        assert_eq!(part2_answer, 4);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 556);
        assert_eq!(part2_answer, 876);
    }
}
