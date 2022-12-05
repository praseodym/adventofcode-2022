use std::cmp::Reverse;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let elfs = parse_input(input);

    let mut cals: Vec<u32> = elfs.iter().map(|elf| elf.iter().sum()).collect();

    let top_elf = *cals.iter().max().unwrap();

    cals.sort_by_key(|w| Reverse(*w));
    let mut top3_elfs: u32 = 0;
    for n in 0..3 {
        top3_elfs += cals[n];
    }

    (top_elf, top3_elfs)
}

fn parse_input(input: &'static str) -> Vec<Vec<u32>> {
    let mut ret = Vec::new();
    for elf in input.trim_end().split("\n\n") {
        let cal = elf
            .lines()
            .map(|line_str| line_str.parse::<u32>().unwrap())
            .collect();
        ret.push(cal);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let elfs = parse_input(include_str!("../input-example"));
        assert_eq!(elfs.len(), 5);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 24000);
        assert_eq!(part2_answer, 45000);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 67016);
        assert_eq!(part2_answer, 200116);
    }
}
