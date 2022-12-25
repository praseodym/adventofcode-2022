fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let scan = parse_input(input);
    let mut part1_answer: u32 = 0;

    for x in 0..N {
        for y in 0..N {
            for z in 0..N {
                if scan[x][y][z] {
                    println!("{},{},{}", x, y, z);
                    part1_answer += exposed_surface(scan, x, y, z);
                }
            }
        }
    }

    let mut part2_answer: u32 = 0;
    //
    for x in 0..N {
        for y in 0..N {
            for z in 0..N {
                if scan[x][y][z] {
                    println!("{},{},{}", x, y, z);
                    part2_answer += exposed_surface(scan, x, y, z);
                }
            }
        }
    }

    (part1_answer, part2_answer)
}

fn exposed_surface(scan: Scan, x: usize, y: usize, z: usize) -> u32 {
    let mut count: u32 = 0;
    if z == 0 || !scan[x][y][z - 1] {
        count += 1;
    }
    if !scan[x][y][z + 1] {
        count += 1;
    }
    if y == 0 || !scan[x][y - 1][z] {
        count += 1;
    }
    if !scan[x][y + 1][z] {
        count += 1;
    }
    if x == 0 || !scan[x - 1][y][z] {
        count += 1;
    }
    if !scan[x + 1][y][z] {
        count += 1;
    }
    count
}

const N: usize = 30;
type Scan = [[[bool; N]; N]; N];
fn parse_input(input: &'static str) -> Scan {
    let mut scan = [[[false; N]; N]; N];
    for line in input.trim_end().split('\n') {
        let mut s = line.split(',');
        let x = s.next().unwrap().parse::<usize>().unwrap()+5;
        let y = s.next().unwrap().parse::<usize>().unwrap()+5;
        let z = s.next().unwrap().parse::<usize>().unwrap()+5;
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
        let (part1_answer, _part2_answer) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 10);
    }

    #[test]
    fn test_example2_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example2"));

        assert_eq!(part1_answer, 64);
        assert_eq!(part2_answer, 58);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 4512);
        // assert_eq!(part2_answer, 0);
    }
}
