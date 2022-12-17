use std::cmp;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, u32) {
    let jets = parse_input(input);
    // rocks are upside down
    let rocks = vec![
        vec![vec![true, true, true, true]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ],
        vec![vec![true], vec![true], vec![true], vec![true]],
        vec![vec![true, true], vec![true, true]],
    ];

    let mut chamber: Chamber = Vec::new();
    let mut jet_idx: usize = 0;

    'spawn_rocks: for rock_idx in 0..2022 {
        // SPAWN ROCK
        println!("== ROCK {} ==", rock_idx + 1);
        let rock = &rocks[rock_idx % rocks.len()];
        let mut movement = Movement::Jet;
        // Each rock appears so that its left edge is two units away from the left wall
        let mut x: isize = 2;
        // and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
        let mut y: isize = (chamber.len() + 3) as isize;

        loop {
            println!("rock position: x={}, y={}", x, y);
            println!("  movement: {:?}", movement);

            // print_chamber(&chamber, rock, x, y);

            match &movement {
                Movement::Jet => {
                    let jet = &jets[jet_idx % jets.len()];
                    println!("  jet: {:?}", jet);
                    match jet {
                        Jet::Left => {
                            if is_clear(&chamber, rock, x - 1, y) {
                                x -= 1;
                            }
                        }
                        Jet::Right => {
                            if is_clear(&chamber, rock, x + 1, y) {
                                x += 1;
                            }
                        }
                    }
                    jet_idx += 1;
                    movement = Movement::Down;
                }
                Movement::Down => {
                    if is_clear(&chamber, rock, x, y - 1) {
                        y -= 1;
                    } else {
                        add_rock(&mut chamber, rock, x, y);
                        continue 'spawn_rocks;
                    }
                    movement = Movement::Jet;
                }
            }
        }
    }

    let part1_answer: usize = chamber.len();
    let part2_answer: u32 = 0;

    (part1_answer, part2_answer)
}

fn is_clear(chamber: &Chamber, rock: &[Vec<bool>], x: isize, y: isize) -> bool {
    // check horizontal clearance
    if x < 0 || x + rock[0].len() as isize > 7 {
        // println!("    no horizontal clearance");
        return false;
    }

    // check vertical clearance
    let dy = y - chamber.len() as isize;
    let h = rock.len() as isize;
    // println!("    distance from chamber: dy={}", dy);
    return if dy >= 0 {
        println!("    don't have to check chamber");
        true
    } else if -dy > chamber.len() as isize {
        println!("    chamber too shallow, rock hits floor");
        false
    } else {
        let mdy = cmp::min(dy + h, 0);
        println!(
            "    checking for collisions with chamber from i in {}..{}",
            dy, mdy
        );
        for i in dy..mdy {
            let ry = (i - dy) as usize;
            println!(
                "      checking i={} with mdy={}, dy={}, h={} => ry {}",
                i, mdy, dy, h, ry
            );
            let rock_line = &rock[ry];
            for (j, v) in rock_line.iter().enumerate() {
                let a = (chamber.len() as isize + i) as usize;
                let b = x as usize + j;
                if *v && chamber[a][b] {
                    // println!("    collision!");
                    return false;
                }
            }
        }
        true
    };
}

fn add_rock(chamber: &mut Chamber, rock: &[Vec<bool>], x: isize, y: isize) {
    println!("rock comes to rest in chamber");
    let dy = rock.len() as isize + y - chamber.len() as isize;
    println!("  adding dy={} rows", dy);
    for _ in 0..dy {
        chamber.push([false; 7]);
    }
    for ry in 0..rock.len() {
        for rx in 0..rock[ry].len() {
            chamber[y as usize + ry][x as usize + rx] |= rock[ry][rx];
        }
    }
}

fn print_chamber(chamber: &Chamber, rock: &[Vec<bool>], x: isize, y: isize) {
    for ry in (0..y + rock.len() as isize).rev() {
        print!("|");
        for rx in 0..7 {
            let w = rock[0].len() as isize;
            let h = rock.len() as isize;
            if (x..x + w).contains(&rx)
                && (y..y + h).contains(&ry)
                && rock[(ry - y) as usize][(rx - x) as usize]
            {
                print!("@");
            } else if ry < chamber.len() as isize && chamber[ry as usize][rx as usize] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+\n");
}

type Chamber = Vec<[bool; 7]>;

#[derive(Debug)]
enum Movement {
    Jet,
    Down,
}

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

fn parse_input(input: &'static str) -> Vec<Jet> {
    let mut jets = Vec::new();
    for c in input.trim_end().chars() {
        let j = match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("unknown jet: {}", c),
        };
        jets.push(j);
    }
    jets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let jets = parse_input(include_str!("../input-example"));
        assert_eq!(jets.len(), 40);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 3068);
        assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 3193);
        // assert_eq!(part2_answer, 0);
    }
}
