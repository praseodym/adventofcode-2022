use crate::Content::{Air, Rock, Sand};
use std::cmp;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer: u32 = 0;
    let part2_answer: u32 = 0;

    let mut cm = CaveMap::parse_input(input);
    // for line in lines {}
    for i in 0..100 {
        println!("drop sand {}", i);
        if !cm.drop_sand() {
            part1_answer = i;
            break;
        }
    }

    (part1_answer, part2_answer)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Content {
    Air,
    Sand,
    Rock,
}

const N: usize = 600;
#[derive(Debug)]
struct CaveMap {
    blocked: [[Content; N]; N],
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

impl CaveMap {
    fn parse_input(input: &str) -> CaveMap {
        let mut points: Vec<Vec<(usize, usize)>> = Vec::new();
        for line in input.trim_end().split('\n') {
            points.push(
                line.split(" -> ")
                    .map(|p| p.split(','))
                    .map(|mut s| {
                        (
                            s.next().unwrap().parse::<usize>().unwrap(),
                            s.next().unwrap().parse::<usize>().unwrap(),
                        )
                    })
                    .collect(),
            );
        }

        let mut cm: CaveMap = CaveMap {
            blocked: [[Air; N]; N],
            min_x: N,
            min_y: N,
            max_x: 0,
            max_y: 0,
        };

        for path in points {
            println!("- {:?}", path);
            for i in 1..path.len() {
                println!("  path {:?} -> {:?}", path[i - 1], path[i]);

                for x in cmp::min(path[i - 1].0, path[i].0)..=cmp::max(path[i - 1].0, path[i].0) {
                    for y in cmp::min(path[i - 1].1, path[i].1)..=cmp::max(path[i - 1].1, path[i].1)
                    {
                        println!("     ({},{})", x, y);
                        cm.block_point(Rock, x, y);
                    }
                }
            }
        }

        cm.print(0, 0);

        cm
    }

    fn block_point(&mut self, content: Content, x: usize, y: usize) {
        self.blocked[y][x] = content;
        self.min_x = cmp::min(x, self.min_x);
        self.min_y = cmp::min(y, self.min_y);
        self.max_x = cmp::max(x, self.max_x);
        self.max_y = cmp::max(y, self.max_y);
    }

    fn print(&self, sx: usize, sy: usize) {
        for y in 0..=self.max_y + 2 {
            print!("| {y:>4}: ", y = y);
            for x in self.min_x - 2..=self.max_x + 2 {
                if x == 0 && y == 500 {
                    print!("+");
                } else if x == sx && y == sy {
                    print!("x");
                } else if self.blocked[y][x] == Sand {
                    print!("o");
                } else if self.blocked[y][x] == Rock {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn drop_sand(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;

        loop {
            if self.blocked[y + 1][x] == Air {
                y += 1;
            } else if self.blocked[y + 1][x - 1] == Air {
                y += 1;
                x -= 1;
            } else if self.blocked[y + 1][x + 1] == Air {
                y += 1;
                x += 1;
            } else {
                break;
            }
            if y == N - 1 {
                println!("sand fell into abyss");
                return false;
            }
        }

        println!("sand lands on {}, {}", x, y);
        self.print(x, y);
        self.block_point(Sand, x, y);
        println!();
        println!();
        true
    }
}
// fn parse_input(input: &'static str) -> Vec<Vec<(usize, usize)>> {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let cm = CaveMap::parse_input(include_str!("../input-example"));
        // println!("lines: {:?}", lines);
        // assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 24);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 901);
        // assert_eq!(part2_answer, 0);
    }
}
