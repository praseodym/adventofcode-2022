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

    let part1_answer = grid.count_visited();
    let part2_answer: u32 = 0;

    (part1_answer, part2_answer)
}

const N: isize = 1000;
#[derive(Debug)]
struct Grid {
    visited: Box<[[bool; N as usize]; N as usize]>,
    hx: isize,
    hy: isize,
    tx: isize,
    ty: isize,
}

impl Grid {
    fn new() -> Self {
        let mut grid = Grid {
            visited: Box::new([[false; N as usize]; N as usize]),
            hx: N / 2,
            hy: N / 2,
            tx: N / 2,
            ty: N / 2,
        };
        grid.update_visited();
        grid
    }

    fn simulate_step(&mut self, direction: &'static str, steps: u8) {
        // println!("== {} {} ==\n", direction, steps);
        match direction {
            "R" => {
                for _ in 0..steps {
                    self.hx += 1;
                    self.update_tail();
                }
            }
            "L" => {
                for _ in 0..steps {
                    self.hx -= 1;
                    self.update_tail();
                }
            }
            "U" => {
                for _ in 0..steps {
                    self.hy -= 1;
                    self.update_tail();
                }
            }
            "D" => {
                for _ in 0..steps {
                    self.hy += 1;
                    self.update_tail();
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

    fn update_tail(&mut self) {
        // if self.first {
        //     self.first = false;
        //     return;
        // }
        // match self.tx.cmp(&self.tx) {
        //     Ordering::Greater => a(),
        //     Ordering::Less => b(),
        //     Ordering::Equal => c()
        // }

        let touching = (self.hx == self.tx || self.hx - 1 == self.tx || self.hx + 1 == self.tx)
            && (self.hy == self.ty || self.hy - 1 == self.ty || self.hy + 1 == self.ty);

        if touching {
            // self.print_state();
            return;
        }

        if self.hx > self.tx {
            self.tx += 1;
        } else if self.hx < self.tx {
            self.tx -= 1;
        }
        if self.hy > self.ty {
            self.ty += 1;
        } else if self.hy < self.ty {
            self.ty -= 1;
        }

        if self.tx < 0 {
            self.tx = 0;
        }
        if self.ty < 0 {
            self.ty = 0;
        }

        // self.print_state();
        self.update_visited();
    }

    fn print_state(&self) {
        for y in 0..N {
            for x in 0..N {
                if x == self.hx && y == self.hy {
                    print!("H");
                } else if x == self.tx && y == self.ty {
                    print!("T");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn update_visited(&mut self) {
        // println!("tail: {} {}", self.tx, self.ty);
        self.visited[self.ty as usize][self.tx as usize] = true;
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
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 13);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        // assert_eq!(part1_answer, 0);
        // assert_eq!(part2_answer, 0);
    }
}
