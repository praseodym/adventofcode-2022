fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer: u32 = 0;
    let part2_answer: u32 = 0;

    let scan = parse_input(input);
    for x in 0..N {
        for y in 0..N {
            for z in 0..N {
                if scan[x][y][z] {
                    if z == 0 || !scan[x][y][z.saturating_sub(1)] {
                        part1_answer += 1;
                    }
                    if !scan[x][y][z + 1] {
                        part1_answer += 1;
                    }
                    if y == 0 || !scan[x][y.saturating_sub(1)][z] {
                        part1_answer += 1;
                    }
                    if !scan[x][y + 1][z] {
                        part1_answer += 1;
                    }
                    if x == 0 || !scan[x.saturating_sub(1)][y][z] {
                        part1_answer += 1;
                    }
                    if !scan[x + 1][y][z] {
                        part1_answer += 1;
                    }
                }
            }
        }
    }

    (part1_answer, part2_answer)
}

const N: usize = 25;

fn parse_input(input: &'static str) -> [[[bool; N]; N]; N] {
    let mut scan = [[[false; N]; N]; N];
    for line in input.trim_end().split('\n') {
        let mut s = line.split(',');
        let x = s.next().unwrap().parse::<usize>().unwrap() + 1;
        let y = s.next().unwrap().parse::<usize>().unwrap() + 1;
        let z = s.next().unwrap().parse::<usize>().unwrap() + 1;
        scan[x][y][z] = true;
    }
    scan
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1_parse() {
        let lines = parse_input(include_str!("../input-example1"));
    }

    #[test]
    fn test_example2_parse() {
        let lines = parse_input(include_str!("../input-example1"));
    }

    #[test]
    fn test_example1_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 10);
        assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_example2_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 64);
        assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 4512);
        // assert_eq!(part2_answer, 0);
    }
}
