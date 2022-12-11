use std::collections::VecDeque;
use std::mem;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u64, u64) {
    let monkeys = parse_input(input);
    let part1_answer = shenanigans(monkeys.clone(), 20, |wl| wl / 3);
    let product: u64 = monkeys.iter().map(|m| m.test).product();
    let part2_answer = shenanigans(monkeys, 10000, |wl| wl % product);
    (part1_answer, part2_answer)
}

fn shenanigans(mut monkeys: Vec<Monkey>, rounds: usize, simplify_wl: impl Fn(u64) -> u64) -> u64 {
    let mut inspected = vec![0u64; monkeys.len()];

    for _round in 1..=rounds {
        #[allow(clippy::needless_range_loop)]
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
                wl = simplify_wl(wl);
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
            }
        }
    }

    inspected.sort();
    inspected.reverse();
    inspected[0] * inspected[1]
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    test_true: usize,
    test_false: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Squared,
    Times(u64),
    Plus(u64),
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
        let starting_items: VecDeque<u64> = s
            .split(", ")
            .map(|line_str| line_str.parse::<u64>().unwrap())
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
                _ => Operation::Times(scalar.parse::<u64>().unwrap()),
            },
            "+" => Operation::Plus(scalar.parse::<u64>().unwrap()),
            _ => panic!("unknown op {}", op),
        };
        let test = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<u64>()
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
        assert_eq!(part2_answer, 2713310158);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 62491);
        assert_eq!(part2_answer, 17408399184);
    }
}
