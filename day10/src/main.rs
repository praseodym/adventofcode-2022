fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (i32, u32) {
    let mut part1_answer: i32 = 0;
    let mut part2_answer: u32 = 0;

    let lines = parse_input(input);
    let mut cycles: i32 = 0;
    let mut x: i32 = 1;

    for line in lines {
        let mut s = line.split_whitespace();
        let cmd = s.next().unwrap();
        match cmd {
            "noop" => {
                cycles += 1;
                if cycles == 20 || (cycles + 20) % 40 == 0 {
                    part1_answer += cycles * x;
                    println!("[*] {} * {} = {}", cycles, x, cycles * x);
                }
                println!("{}", cmd);
            }
            "addx" => {
                let dx = s.next().unwrap().parse::<i32>().unwrap();
                cycles += 1;
                if cycles == 20 || (cycles + 20) % 40 == 0 {
                    part1_answer += cycles * x;
                    println!("[*] {} * {} = {}", cycles, x, cycles * x);
                }

                cycles += 1;
                if cycles == 20 || (cycles + 20) % 40 == 0 {
                    part1_answer += cycles * x;
                    println!("[*] {} * {} = {}", cycles, x, cycles * x);
                }

                x += dx;
                println!("{}", line);
                println!("{} {} -- x={}, cycle={}", cmd, dx, x, cycles);
            }
            _ => {
                panic!("cmd: {}", cmd);
            }
        }
    }

    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> Vec<&str> {
    input.trim_end().split('\n').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let lines = parse_input(include_str!("../input-example2"));
        // assert_eq!(lines.len(), 5);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 13140);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        // assert_eq!(part1_answer, 0);
        // assert_eq!(part2_answer, 0);
    }
}
