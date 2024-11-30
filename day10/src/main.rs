fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer:\n{}", part2_answer);
}

fn run(input: &'static str) -> (i32, String) {
    let mut part1_answer: i32 = 0;
    let mut part2_answer = String::new();

    let lines = parse_input(input);
    let mut cycles: i32 = 0;
    let mut x: i32 = 1;

    for line in lines {
        let mut s = line.split_whitespace();
        let cmd = s.next().unwrap();
        match cmd {
            "noop" => {
                cycles += 1;
                update_answers(&mut part1_answer, &mut part2_answer, &mut cycles, &mut x);
            }
            "addx" => {
                let dx = s.next().unwrap().parse::<i32>().unwrap();
                cycles += 1;
                update_answers(&mut part1_answer, &mut part2_answer, &mut cycles, &mut x);
                cycles += 1;
                update_answers(&mut part1_answer, &mut part2_answer, &mut cycles, &mut x);
                x += dx;
            }
            _ => {
                panic!("cmd: {}", cmd);
            }
        }
    }

    (part1_answer, part2_answer)
}

fn update_answers(
    part1_answer: &mut i32,
    part2_answer: &mut String,
    cycles: &mut i32,
    x: &mut i32,
) {
    if *cycles == 20 || (*cycles + 20) % 40 == 0 {
        *part1_answer += *cycles * *x;
    }

    let pos = (*cycles - 1) % 40;
    if pos >= *x - 1 && pos <= *x + 1 {
        *part2_answer += "#";
    } else {
        *part2_answer += ".";
    }
    if *cycles % 40 == 0 {
        *part2_answer += "\n";
    }
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
        assert_eq!(lines.len(), 146);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 13140);
        assert_eq!(part2_answer, include_str!("../input-example-answer-part2"));
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 16480);
        assert_eq!(part2_answer, include_str!("../input-answer-part2"));
    }
}
