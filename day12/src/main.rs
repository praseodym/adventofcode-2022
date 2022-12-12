use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let em = ElevationMap::parse_input(input);
    let part1_answer = em.find_path(em.start);

    let mut distances: Vec<usize> = Vec::new();
    for y in 0..=em.max_y {
        for x in 0..=em.max_x {
            if em.elevations[y][x] == 0 {
                let distance = em.find_path((x, y));
                distances.push(distance);
            }
        }
    }
    distances.sort();
    let part2_answer = distances[0];

    (part1_answer, part2_answer)
}

fn get_elevation(elevation: char) -> Elevation {
    if elevation.is_lowercase() {
        (elevation as Elevation) - 97
    } else {
        match elevation {
            'S' => 0 as Elevation,
            'E' => 25 as Elevation,
            _ => panic!("unknown elevation: {}", elevation),
        }
    }
}

const N: usize = 80;
type Elevation = i8;

#[derive(Debug)]
struct ElevationMap {
    elevations: [[Elevation; N]; N],
    max_x: usize,
    max_y: usize,
    start: (usize, usize),
    end: (usize, usize),
}
#[derive(Default, Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
    elevation: Elevation,
}

#[derive(Default, Debug, Copy, Clone)]
struct Visit<V> {
    pos: V,
    distance: usize,
}

impl Eq for Position {}
impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl ElevationMap {
    fn parse_input(input: &str) -> ElevationMap {
        let mut em: ElevationMap = ElevationMap {
            elevations: [[0; N]; N],
            max_x: 0,
            max_y: 0,
            start: (0, 0),
            end: (0, 0),
        };
        let input = input.trim_end().split('\n');
        for (y, line) in input.enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        em.start = (x, y);
                    }
                    'E' => {
                        em.end = (x, y);
                    }
                    _ => (),
                }
                em.max_x = cmp::max(x, em.max_x);
                em.max_y = cmp::max(y, em.max_y);
                em.elevations[y][x] = get_elevation(c);
            }
        }
        em
    }

    // adventofcode2021 day15
    fn find_path(&self, start: (usize, usize)) -> usize {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit: BinaryHeap<Visit<Position>> = BinaryHeap::new();

        // start
        to_visit.push(Visit {
            pos: Position {
                x: start.0,
                y: start.1,
                elevation: self.elevations[start.1][start.0],
            },
            distance: 0,
        });

        while let Some(Visit { pos, distance }) = to_visit.pop() {
            // TODO: terminate early if we are at the end position

            if !visited.insert(pos) {
                continue;
            }

            let adj = self.get_adjacent(pos);
            for neighbour in adj {
                let new_distance = distance + 1;

                let is_shorter = distances
                    .get(&neighbour)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(neighbour, new_distance);
                    to_visit.push(Visit {
                        pos: neighbour,
                        distance: new_distance,
                    })
                }
            }
        }

        let end_distance = distances.get(&Position {
            x: self.end.0,
            y: self.end.1,
            elevation: 0,
        });
        end_distance.map_or(usize::MAX, |d| *d)
    }

    // "you can move exactly one square up, down, left, or right" / "at most one higher"
    fn get_adjacent(&self, pos: Position) -> Vec<Position> {
        let mut ret = Vec::new();
        let x = pos.x;
        let y = pos.y;
        if x != 0 {
            // left
            let x = x - 1;
            let y = y;
            let elevation = self.elevations[y][x] as Elevation;
            if elevation - pos.elevation <= 1 {
                ret.push(Position { x, y, elevation })
            }
        }
        if x != self.max_x {
            // right
            let x = x + 1;
            let y = y;
            let elevation = self.elevations[y][x] as Elevation;
            if elevation - pos.elevation <= 1 {
                ret.push(Position { x, y, elevation })
            }
        }
        if y != 0 {
            // up
            let x = x;
            let y = y - 1;
            let elevation = self.elevations[y][x] as Elevation;
            if elevation - pos.elevation <= 1 {
                ret.push(Position { x, y, elevation })
            }
        }
        if y != self.max_y {
            // down
            let x = x;
            let y = y + 1;
            let elevation = self.elevations[y][x] as Elevation;
            if elevation - pos.elevation <= 1 {
                ret.push(Position { x, y, elevation })
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elevation() {
        assert_eq!(get_elevation('b'), 1);
        assert_eq!(get_elevation('d'), 3);
        assert_eq!(get_elevation('S'), 0);
        assert_eq!(get_elevation('E'), 25);
    }

    #[test]
    fn test_example_parse() {
        let em = ElevationMap::parse_input(include_str!("../input-example"));
        assert_eq!(em.max_x, 7);
        assert_eq!(em.max_y, 4);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 31);
        assert_eq!(part2_answer, 29);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 391);
        assert_eq!(part2_answer, 386);
    }
}
