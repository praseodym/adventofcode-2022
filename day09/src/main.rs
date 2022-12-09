use std::cmp::Ordering;
use std::slice::Iter;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let motions = parse_input(input);

    let part1_answer = Rope::simulate(motions.iter(), 2);
    let part2_answer = Rope::simulate(motions.iter(), 10);

    (part1_answer, part2_answer)
}

const N: usize = 700;
#[derive(Debug)]
struct Rope {
    visited: Box<[[bool; N]; N]>,
    kx: Vec<usize>,
    ky: Vec<usize>,
}

impl Rope {
    pub fn simulate(motions: Iter<(char, u8)>, knots: usize) -> usize {
        let mut rope = Rope::new(knots);
        for motion in motions {
            rope.simulate_step(motion.0, motion.1);
        }
        rope.count_visited()
    }

    fn new(knots: usize) -> Self {
        Rope {
            visited: Box::new([[false; N as usize]; N as usize]),
            kx: vec![N / 2; knots],
            ky: vec![N / 2; knots],
        }
    }

    fn simulate_step(&mut self, direction: char, steps: u8) {
        for _ in 0..steps {
            match direction {
                'L' => self.kx[0] -= 1,
                'R' => self.kx[0] += 1,
                'U' => self.ky[0] -= 1,
                'D' => self.ky[0] += 1,
                _ => panic!("unknown: {}", direction),
            };
            self.update_knots();
        }
    }

    fn update_knots(&mut self) {
        for i in 1..self.kx.len() {
            let touching = (self.ky[i - 1] + 1 == self.ky[i]
                || self.ky[i - 1] - 1 == self.ky[i]
                || self.ky[i - 1] == self.ky[i])
                && (self.kx[i - 1] + 1 == self.kx[i]
                    || self.kx[i - 1] - 1 == self.kx[i]
                    || self.kx[i - 1] == self.kx[i]);

            if touching {
                continue;
            }

            match self.kx[i - 1].cmp(&self.kx[i]) {
                Ordering::Greater => self.kx[i] += 1,
                Ordering::Less => self.kx[i] -= 1,
                Ordering::Equal => (),
            }

            match self.ky[i - 1].cmp(&self.ky[i]) {
                Ordering::Greater => self.ky[i] += 1,
                Ordering::Less => self.ky[i] -= 1,
                Ordering::Equal => (),
            }
        }

        self.update_visited();
    }

    fn update_visited(&mut self) {
        self.visited[self.ky[self.ky.len() - 1]][self.kx[self.kx.len() - 1]] = true;
    }

    fn count_visited(&self) -> usize {
        self.visited
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&v| v)
            .count()
    }
}

fn parse_input(input: &'static str) -> Vec<(char, u8)> {
    let mut ret = Vec::new();
    for line in input.trim_end().split('\n') {
        let mut s = line.split_whitespace();
        ret.push((
            s.next().unwrap().parse::<char>().unwrap(),
            s.next().unwrap().parse::<u8>().unwrap(),
        ))
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1_parse() {
        let lines = parse_input(include_str!("../input-example1"));
        assert_eq!(lines.len(), 8);
    }

    #[test]
    fn test_example2_parse() {
        let lines = parse_input(include_str!("../input-example2"));
        assert_eq!(lines.len(), 8);
    }

    #[test]
    fn test_example1_answer() {
        let (part1_answer, _) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 13);
    }

    #[test]
    fn test_example2_answer() {
        let (_, part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part2_answer, 36);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 6494);
        assert_eq!(part2_answer, 2691);
    }
}
