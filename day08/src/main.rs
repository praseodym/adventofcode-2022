use std::cmp;

const N: usize = 100;
type Tree = u8;
#[derive(Debug)]
struct Grid {
    trees: [[Tree; N]; N],
    width: usize,
    height: usize,
}

impl Grid {
    fn parse_input(input: &str) -> Grid {
        let mut grid: Grid = Grid {
            trees: [[0; N]; N],
            width: 0,
            height: 0,
        };
        let input = input.trim_end().split('\n');
        for (y, line) in input.enumerate() {
            for (x, d) in line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .enumerate()
            {
                grid.width = cmp::max(x, grid.width);
                grid.height = cmp::max(y, grid.height);
                grid.trees[y][x] = d;
            }
        }
        grid
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.width || y == self.height {
            true
        } else {
            let h = self.trees[y][x];
            let mut visible_left = true;
            for a in 0..x {
                if self.trees[y][a] >= h {
                    visible_left = false;
                    break;
                }
            }
            let mut visible_right = true;
            for a in x + 1..=self.width {
                if self.trees[y][a] >= h {
                    visible_right = false;
                    break;
                }
            }
            let mut visible_top = true;
            for b in 0..y {
                if self.trees[b][x] >= h {
                    visible_top = false;
                    break;
                }
            }
            let mut visible_bottom = true;
            for b in y + 1..=self.height {
                if self.trees[b][x] >= h {
                    visible_bottom = false;
                    break;
                }
            }
            visible_left || visible_right || visible_top || visible_bottom
        }
    }

    fn count_visible(&self) -> usize {
        let mut n: usize = 0;
        for x in 0..=self.width {
            for y in 0..=self.height {
                if self.is_visible(x, y) {
                    n += 1;
                    print!("x")
                } else {
                    print!(".")
                }
            }
            println!();
        }
        n
    }
}

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let grid = Grid::parse_input(input);

    let part1_answer = grid.count_visible() as u32;
    let part2_answer = 0;

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let grid = Grid::parse_input(include_str!("../input-example"));
        // off by one
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 4);
    }

    #[test]
    fn test_example_visibility() {
        let grid = Grid::parse_input(include_str!("../input-example"));
        assert!(grid.is_visible(0, 0));
        assert!(grid.is_visible(4, 0));
        assert!(!grid.is_visible(1, 3));
        assert!(grid.is_visible(2, 3));
        assert!(!grid.is_visible(3, 3));
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 21);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (_part1_answer, _part2_answer) = run(include_str!("../input"));
        // assert_eq!(part1_answer, 0);
        // assert_eq!(part2_answer, 0);
    }
}
