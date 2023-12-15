use regex::Regex;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let valves = parse_input(input);
    let shortest_paths = shortest_paths(&valves);
    let part1_answer = max_released_pressure(&valves, &shortest_paths);
    (part1_answer, 0)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Valve {
    name: String,
    index: usize,
    flow_rate: usize,
    neighbours: Vec<usize>,
    neighbour_names: Vec<String>,
}

fn parse_input(input: &'static str) -> Vec<Valve> {
    let mut valves = Vec::new();
    let mut valve_indices = HashMap::new();
    let re =
        Regex::new(r"^Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap();
    for (index, line) in input.trim_end().split('\n').enumerate() {
        let cap = re.captures(line).unwrap();
        let name = cap[1].to_string();
        valve_indices.insert(name.clone(), index);
        let flow_rate = cap[2].parse().unwrap();
        let neighbour_names = cap[3].split(", ").map(|s| s.to_string()).collect();
        let valve = Valve {
            name,
            index,
            flow_rate,
            neighbours: Vec::new(),
            neighbour_names,
        };
        valves.push(valve);
    }

    for valve in &mut valves {
        for neighbour_name in &valve.neighbour_names {
            let neighbour_index = valve_indices.get(neighbour_name).unwrap();
            valve.neighbours.push(*neighbour_index);
        }
    }

    valves
}

fn shortest_paths(valves: &Vec<Valve>) -> Vec<Vec<usize>> {
    // all-pairs shortest paths for valves, using the Floyd-Warshall algorithm
    // https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
    let mut distances: Vec<Vec<usize>> = Vec::new();

    for _ in 0..valves.len() {
        let row = vec![usize::MAX; valves.len()];
        distances.push(row);
    }

    for (i, d) in distances.iter_mut().enumerate() {
        d[i] = 0;
    }

    for valve in valves {
        for neighbour in &valve.neighbours {
            distances[valve.index][*neighbour] = 1;
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if i == j {
                    continue;
                }
                distances[i][j] = cmp::min(
                    distances[i][j],
                    distances[i][k].saturating_add(distances[k][j]),
                );
            }
        }
    }

    distances
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    opened: Vec<usize>,
    pos: usize,
    elapsed: usize,
    relieved: usize,
}

fn max_released_pressure(valves: &[Valve], distances: &[Vec<usize>]) -> usize {
    let usable_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_i, v)| v.flow_rate > 0)
        .map(|(i, _v)| i)
        .collect();

    let start = valves.iter().position(|v| v.name == "AA").unwrap();
    let state = State {
        opened: Vec::new(),
        pos: start,
        elapsed: 0,
        relieved: 0,
    };
    let mut queue: Vec<State> = Vec::new();
    queue.push(state);

    let mut max_released_pressure = 0;
    while let Some(state) = queue.pop() {
        let possible = usable_valves
            .iter()
            .filter(|v| **v != state.pos && !state.opened.contains(*v))
            .collect::<Vec<_>>();
        for &next in possible {
            let distance = distances[state.pos][next];
            if distance == usize::MAX {
                continue;
            }
            let mut opened = state.opened.clone();
            opened.push(next);
            let elapsed = state.elapsed + distance + 1;
            if elapsed > 30 {
                continue;
            }
            let relieved = state.relieved + (30 - elapsed) * valves[next].flow_rate;
            let state = State {
                opened,
                pos: next,
                elapsed,
                relieved,
            };
            max_released_pressure = cmp::max(max_released_pressure, state.relieved);
            queue.push(state);
        }
    }

    max_released_pressure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let valves = parse_input(include_str!("../input-example"));
        assert_eq!(valves.len(), 10);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 1651);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1792);
        // assert_eq!(part2_answer, 0);
    }
}
