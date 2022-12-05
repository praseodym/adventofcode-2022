use std::collections::VecDeque;

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (String, String) {
    let (mut stacks, commands) = parse_input(input);
    let stacks2 = stacks.clone();

    // part one
    for m in &commands {
        for _ in 0..m.amount {
            let item = stacks[m.from - 1].pop_back().unwrap();
            stacks[m.to - 1].push_back(item);
        }
    }
    let part1_answer = answer(&mut stacks);

    // part two
    stacks = stacks2;
    for m in &commands {
        let n = stacks[m.from - 1].len();
        let items: Vec<char> = stacks[m.from - 1].drain(n - m.amount..n).collect();
        items
            .iter()
            .for_each(|item| stacks[m.to - 1].push_back(*item));
    }
    let part2_answer = answer(&mut stacks);

    (part1_answer, part2_answer)
}

fn answer(stacks: &Vec<VecDeque<char>>) -> String {
    let mut answer = "".to_string();
    for stack in stacks {
        answer.push(stack[stack.len() - 1]);
    }
    answer
}

fn parse_input(input: &'static str) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut lines = input.trim_end().split('\n');
    'outer: loop {
        let mut chars = lines.next().unwrap().chars();
        let mut i = 0;
        loop {
            let skip = 1 + (if i > 0 { 2 } else { 0 });
            let c = (&mut chars).skip(skip).take(1).next();
            if c.is_none() {
                break;
            }
            let c = c.unwrap().clone();
            if c.is_digit(10) {
                break 'outer;
            }
            if stacks.len() == i {
                stacks.push(VecDeque::new());
            }
            if c != ' ' {
                stacks[i].push_front(c);
            }
            i += 1;
        }
    }
    let mut commands: Vec<Move> = Vec::new();
    for line in lines.skip(1) {
        let mut s = line.split_whitespace();
        let m = Move {
            amount: (&mut s)
                .skip(1)
                .take(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            from: (&mut s)
                .skip(1)
                .take(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            to: (&mut s)
                .skip(1)
                .take(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        };
        commands.push(m);
    }
    (stacks, commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let (stacks, commands) = parse_input(include_str!("../input-example"));
        assert_eq!(stacks.len(), 3);
        assert_eq!(commands.len(), 4);
    }

    #[test]
    fn test_input_parse() {
        let (stacks, _) = parse_input(include_str!("../input"));
        assert_eq!(stacks.len(), 9);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, "CMZ");
        assert_eq!(part2_answer, "MCD");
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, "CNSZFDVLJ");
    }
}
