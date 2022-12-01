fn main() {
    let part1_answer = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
}

fn run(input: &'static str) -> u32 {
    let elfs = parse_input(input);

    elfs.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn parse_input(input: &'static str) -> Vec<Vec<u32>> {
    let mut ret = Vec::new();
    for elf in input.trim_end().split("\n\n") {
        let cal = elf.lines()
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
    fn test_parse_example() {
        let elfs = parse_input(include_str!("../input-example"));
        assert_eq!(elfs.len(), 5);
    }

    #[test]
    fn test_input_example1() {
        let part1_answer = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 24000);
    }
}
