use std::collections::VecDeque;
use std::mem;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer: u32 = 0;
    let mut part2_answer: u32 = 0;

    println!("=====");

    let mut monkeys = parse_input(input);
    let mut inspected = vec![0; monkeys.len()];

    for round in 1..=20 {
        for i in 0..monkeys.len() {
            let mut items = {
                let monkey = monkeys.get_mut(i).unwrap();
                mem::take(&mut monkey.items)
            };
            let monkey = monkeys.get(i).unwrap().clone();
            while let Some(item) = items.pop_front() {
                inspected[i] += 1;
                let mut wl = match monkey.operation {
                    Operation::Squared => item * item,
                    Operation::Times(x) => item * x,
                    Operation::Plus(x) => item + x,
                };
                wl /= 3;
                if wl % monkey.test == 0 {
                    monkeys
                        .get_mut(monkey.test_true)
                        .unwrap()
                        .items
                        .push_back(wl);
                } else {
                    monkeys
                        .get_mut(monkey.test_false)
                        .unwrap()
                        .items
                        .push_back(wl);
                }
                // println!("wl: {}", wl);
            }
            // println!("monkey: {:?}", &monkey);
        }
        println!("=====");

        for monkey in &monkeys {
            println!("monkey: {:?}", &monkey);
        }
    }

    println!("inspected: {:?}", &inspected);
    inspected.sort();
    inspected.reverse();
    part1_answer = inspected[0] * inspected[1];

    (part1_answer, part2_answer)
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    test: u32,
    test_true: usize,
    test_false: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Squared,
    Times(u32),
    Plus(u32),
}

fn parse_input(input: &'static str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut lines = input.trim_end().split('\n');
    loop {
        lines.next().unwrap();
        let s = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap();
        let starting_items: VecDeque<u32> = s
            .split(", ")
            .map(|line_str| line_str.parse::<u32>().unwrap())
            .collect();
        let s = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Operation: new = old ")
            .unwrap();
        let mut p = s.split_whitespace();
        let op = p.next().unwrap();
        let scalar = p.next().unwrap();
        let operation = match op {
            "" => Operation::Squared,
            "*" => match scalar {
                "old" => Operation::Squared,
                _ => Operation::Times(scalar.parse::<u32>().unwrap()),
            },
            "+" => Operation::Plus(scalar.parse::<u32>().unwrap()),
            _ => panic!("unknown op {}", op),
        };
        let test = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let test_true = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let test_false = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let monkey = Monkey {
            items: starting_items,
            operation,
            test,
            test_true,
            test_false,
        };
        println!("monkey: {:?}", &monkey);
        monkeys.push(monkey);
        if lines.next().is_none() {
            break;
        }
    }
    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let lines = parse_input(include_str!("../input-example"));
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 10605);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 62491);
        // assert_eq!(part2_answer, 0);
    }
}
