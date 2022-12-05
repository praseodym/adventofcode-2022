use std::collections::VecDeque;

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"), 9);
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str, num_stacks: usize) -> (String, String) {
    let mut part1_answer = "".to_string();
    let part2_answer = "".to_string();

    let (mut stacks, commands) = parse_input(input, num_stacks);

    for stack in &stacks {
        println!("stack: {:?}", stack);
    }
    println!("===========");

    for m in commands {
        for n in 0..m.amount {
            let item = stacks[m.from - 1].pop_back().unwrap();
            stacks[m.to - 1].push_back(item);
        }

        for stack in &stacks {
            println!("stack: {:?}", stack);
        }
        println!("===========");
    }

    for stack in &stacks {
        part1_answer.push(stack[stack.len() - 1]);
    }

    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str, num_stacks: usize) -> (Vec<VecDeque<char>>, Vec<Move>) {
    //let num_stacks = input.split('\n').next().unwrap().chars().count()/4;
    //println!("num stacks: {}", num_stacks);
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(VecDeque::new());
    }
    let mut lines = input.trim_end().split('\n');
    'outer: loop {
        let mut chars = lines.next().unwrap().chars();
        for i in 0..num_stacks {
            let skip = 1 + (if i > 0 { 2 } else { 0 });
            let c = (&mut chars).skip(skip).take(1).next();
            if c.is_none() {
                break;
            }
            let c = c.unwrap().clone();
            if c.is_digit(10) {
                println!("end of stacks: {}", c);
                break 'outer;
            }
            println!("found {}", c);
            if c != ' ' {
                stacks[i].push_front(c);
            }
        }
        println!("new line");
    }
    let mut commands: Vec<Move> = Vec::new();
    for line in lines.skip(1) {
        let mut s = line.split_whitespace();
        let m = Move {
            amount: (&mut s).skip(1).take(1).next().unwrap().parse::<usize>().unwrap(),
            from: (&mut s).skip(1).take(1).next().unwrap().parse::<usize>().unwrap(),
            to: (&mut s).skip(1).take(1).next().unwrap().parse::<usize>().unwrap(),
        };
        println!("command: {:?}", m);
        commands.push(m);
    }
    (stacks, commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let (stacks, commands) = parse_input(include_str!("../input-example"), 3);
        assert_eq!(stacks.len(), 3);
        assert_eq!(commands.len(), 4);
    }

    #[test]
    fn test_input_parse() {
        let (stacks, commands) = parse_input(include_str!("../input"), 9);
        assert_eq!(stacks.len(), 9);
        for stack in stacks {
            println!("stack: {:?}", stack);
        }
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"), 3);
        assert_eq!(part1_answer, "CMZ");
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"), 9);
        assert_eq!(part1_answer, "CNSZFDVLJ");
    }
}
