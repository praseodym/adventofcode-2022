fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

const N: usize = 25;
const ADJACENCY: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];
type Scan = [[[bool; N]; N]; N];

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer: u32 = 0;
    let part2_answer: u32 = 0;

    let scan = parse_input(input);
    for z in 0..N {
        for y in 0..N {
            for x in 0..N {
                if scan[z][y][x] {
                    for (dx, dy, dz) in ADJACENCY.iter() {
                        let nx = x.saturating_add_signed(*dx);
                        let ny = y.saturating_add_signed(*dy);
                        let nz = z.saturating_add_signed(*dz);
                        if nx >= N || ny >= N || nz >= N || !scan[nz][ny][nx] {
                            part1_answer += 1;
                        }
                    }
                }
            }
        }
    }

    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> Scan {
    let mut scan = Scan::default();
    for line in input.trim_end().split('\n') {
        let mut s = line.split(',').map(|s| s.parse::<usize>().unwrap() + 1);
        let x = s.next().unwrap();
        let y = s.next().unwrap();
        let z = s.next().unwrap();
        scan[z][y][x] = true;
    }
    scan
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1_parse() {
        parse_input(include_str!("../input-example1"));
    }

    #[test]
    fn test_example2_parse() {
        parse_input(include_str!("../input-example1"));
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
