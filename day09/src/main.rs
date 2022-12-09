fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut grid = Grid::new();

    let motions = parse_input(input);
    for motion in motions {
        grid.simulate_step(motion.0, motion.1);
    }

    let part1_answer: u32 = 0;
    let part2_answer = grid.count_visited();

    (part1_answer, part2_answer)
}

const N: isize = 1000;
#[derive(Debug)]
struct Grid {
    visited: Box<[[bool; N as usize]; N as usize]>,
    kx: [isize; 10],
    ky: [isize; 10],
}

impl Grid {
    fn new() -> Self {
        let mut grid = Grid {
            visited: Box::new([[false; N as usize]; N as usize]),
            kx: [N / 2; 10],
            ky: [N / 2; 10],
        };
        grid.update_visited();
        grid
    }

    fn simulate_step(&mut self, direction: &'static str, steps: u8) {
        // println!("== {} {} ==\n", direction, steps);
        match direction {
            "R" => {
                for _ in 0..steps {
                    self.kx[0] += 1;
                    self.update_knots();
                }
            }
            "L" => {
                for _ in 0..steps {
                    self.kx[0] -= 1;
                    self.update_knots();
                }
            }
            "U" => {
                for _ in 0..steps {
                    self.ky[0] -= 1;
                    self.update_knots();
                }
            }
            "D" => {
                for _ in 0..steps {
                    self.ky[0] += 1;
                    self.update_knots();
                }
            }
            _ => {
                panic!("unknown: {}", direction)
            }
        }
    }

    fn count_visited(&self) -> u32 {
        let mut count = 0;
        for y in 0..N as usize {
            for x in 0..N as usize {
                if self.visited[y][x] {
                    // print!("#");
                    count += 1;
                } else {
                    // print!(".");
                }
            }
            // println!();
        }
        count
    }

    fn update_knots(&mut self) {
        // if self.first {
        //     self.first = false;
        //     return;
        // }
        // match self.tx.cmp(&self.tx) {
        //     Ordering::Greater => a(),
        //     Ordering::Less => b(),
        //     Ordering::Equal => c()
        // }
        for i in 1..10 {
            let touching = (self.kx[i - 1] == self.kx[i]
                || self.kx[i - 1] - 1 == self.kx[i]
                || self.kx[i - 1] + 1 == self.kx[i])
                && (self.ky[i - 1] == self.ky[i]
                    || self.ky[i - 1] - 1 == self.ky[i]
                    || self.ky[i - 1] + 1 == self.ky[i]);

            if touching {
                continue;
            }

            if self.kx[i - 1] > self.kx[i] {
                self.kx[i] += 1;
            } else if self.kx[i - 1] < self.kx[i] {
                self.kx[i] -= 1;
            }
            if self.ky[i - 1] > self.ky[i] {
                self.ky[i] += 1;
            } else if self.ky[i - 1] < self.ky[i] {
                self.ky[i] -= 1;
            }

            if self.kx[i] < 0 {
                self.kx[i] = 0;
            }
            if self.ky[i] < 0 {
                self.ky[i] = 0;
            }
        }

        self.update_visited();
    }

    fn update_visited(&mut self) {
        // println!("tail: {} {}", self.tx, self.ty);
        self.visited[self.ky[9] as usize][self.kx[9] as usize] = true;
    }
}

fn parse_input(input: &'static str) -> Vec<(&'static str, u8)> {
    let mut ret = Vec::new();
    for line in input.trim_end().split('\n') {
        let mut s = line.split_whitespace();
        ret.push((s.next().unwrap(), s.next().unwrap().parse::<u8>().unwrap()))
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let lines = parse_input(include_str!("../input-example"));
        assert_eq!(lines.len(), 8);
    }

    #[test]
    fn test_example1_answer() {
        let (part1_answer, _) = run(include_str!("../input-example"));
        //assert_eq!(part1_answer, 13);
    }

    #[test]
    fn test_example2_answer() {
        let (_, part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part2_answer, 36);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        // assert_eq!(part1_answer, 6494);
        assert_eq!(part2_answer, 2691);
    }
}
