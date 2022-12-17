use regex::Regex;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let valves = parse_input(input);

    let mut distances: Vec<Vec<usize>> = Vec::new();
    for i in 0..valves.len() {
        distances.push(shortest_path(&valves, i));
    }
    // println!("{:?}", distances);
    // println!("shortest paths done");

    let mut max_released_pressure = 0;

    let start = valves.iter().position(|v| v.name == "AA").unwrap();
    let mut states: Vec<State> = Vec::new();
    states.push(State {
        pos1: start,
        pos2: start,
        done1: 0,
        done2: 0,
        minute: 0,
        released_pressure: 0,
        open_valves: Vec::new(),
        // potential_released_pressure: 0,
    });
    let useful_valves: Vec<usize> = (0..valves.len())
        .filter(|i| valves[*i].flow_rate > 0)
        .collect();
    let total_possible_release: u32 = valves.iter().map(|v| v.flow_rate).sum();
    let max_minutes = 26;

    loop {
        let state = states.pop();
        if state.is_none() {
            break;
        }
        let mut state = state.unwrap();

        // println!("state: {:?}", state);

        if state.minute >= max_minutes {
            if state.released_pressure > max_released_pressure {
                println!(
                    "done - minute: {}, cur max: {}, prev max: {}, left: {}",
                    state.minute,
                    state.released_pressure,
                    max_released_pressure,
                    states.len()
                );
            }
            max_released_pressure = cmp::max(max_released_pressure, state.released_pressure);
            continue;
        }

        // state.visited.push(state.valve);
        let pressure: u32 = state.open_valves.iter().map(|v| valves[*v].flow_rate).sum();
        state.minute += 1;
        state.released_pressure += pressure;
        // states.push(state.clone());

        let minutes_remaining = max_minutes - state.minute;

        // check if we have valves left to open
        let cur_remaining = minutes_remaining * total_possible_release;
        if (state.released_pressure + cur_remaining) <= max_released_pressure {
            continue;
        }
        // state.potential_released_pressure = state.released_pressure + cur_remaining;

        // move to other valves or stay at current place
        for next in &useful_valves {
            let distance = cmp::max(1, distances[state.pos1][*next] as u32);
            if distance > minutes_remaining {
                continue;
            }

            let mut new_state = state.clone();
            if state.done1 <= state.minute {
                new_state.pos1 = *next;
                new_state.done1 = state.minute + distance;
                states.push(new_state.clone());
            }

            for next in &useful_valves {
                let distance = cmp::max(1, distances[state.pos1][*next] as u32);
                if distance > minutes_remaining {
                    continue;
                }
                let mut new_state2 = new_state.clone();
                if state.done2 <= state.minute {
                    new_state2.pos2 = *next;
                    new_state2.done2 = state.minute + distance;
                    states.push(new_state2);
                }
            }
        }

        // check if we can open the current valve
        if state.done1 <= state.minute
            && valves[state.pos1].flow_rate > 0
            && !state.open_valves.contains(&state.pos1)
        {
            let mut new_state = state.clone();
            new_state.open_valves.push(state.pos1);
            states.push(new_state.clone());

            if state.done2 <= state.minute {
                for next in &useful_valves {
                    let distance = cmp::max(1, distances[state.pos1][*next] as u32);
                    if distance > minutes_remaining {
                        continue;
                    }
                    let mut new_state2 = new_state.clone();
                    new_state2.pos2 = *next;
                    new_state2.done2 = state.minute + distance;
                    states.push(new_state2.clone());
                }
            }
        }
        if state.done2 <= state.minute
            && valves[state.pos2].flow_rate > 0
            && !state.open_valves.contains(&state.pos2)
        {
            let mut new_state = state.clone();
            new_state.open_valves.push(state.pos2);
            states.push(new_state.clone());

            if state.done1 <= state.minute {
                for next in &useful_valves {
                    let distance = cmp::max(1, distances[state.pos1][*next] as u32);
                    if distance > minutes_remaining {
                        continue;
                    }
                    let mut new_state2 = new_state.clone();
                    new_state2.pos1 = *next;
                    new_state2.done1 = state.minute + distance;
                    states.push(new_state2.clone());
                }
            }
        }
    }

    (0, max_released_pressure)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct DijkstraState {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.
// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(valves: &Vec<Valve>, start: usize) -> Vec<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..valves.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(DijkstraState {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(DijkstraState { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        // if position == goal {
        //     return Some(cost);
        // }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &valves[position].tunnels {
            let next = DijkstraState {
                cost: cost + 1,
                position: *edge,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    dist
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels_names: Vec<String>,
    tunnels: Vec<usize>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct State {
    pos1: usize,
    pos2: usize,
    done1: u32,
    done2: u32,
    minute: u32,
    released_pressure: u32,
    open_valves: Vec<usize>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.released_pressure.cmp(&other.released_pressure)
        // .then_with(|| self.released_pressure.cmp(&other.released_pressure))
        // .then_with(|| {
        //     self.potential_released_pressure
        //         .cmp(&other.potential_released_pressure)
        // })
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &'static str) -> Vec<Valve> {
    let mut valves = Vec::new();
    let mut valve_names = Vec::new();
    let re =
        Regex::new(r"^Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap();
    for line in input.trim_end().split('\n') {
        // println!("line: {:?}", line);
        let cap = re.captures(line).unwrap();
        let name = cap[1].to_string();
        let flow_rate = cap[2].parse().unwrap();
        let tunnels_names = cap[3].split(", ").map(|s| s.to_string()).collect();
        let valve = Valve {
            name: name.clone(),
            flow_rate,
            tunnels: Vec::new(),
            tunnels_names,
        };
        // println!("valve: {:?}", valve);
        valve_names.push(valve.name.clone());
        valves.push(valve);
    }
    for valve in valves.iter_mut() {
        valve.tunnels = valve
            .tunnels_names
            .iter()
            .map(|n| valve_names.iter().position(|v| v == n).unwrap())
            .collect();
    }
    valves
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
        let (_part1_answer, part2_answer) = run(include_str!("../input-example"));
        // assert_eq!(part1_answer, 1651);
        assert_eq!(part2_answer, 1707);
    }

    #[test]
    fn test_input_answer() {
        let (_part1_answer, _part2_answer) = run(include_str!("../input"));
        // assert_eq!(part1_answer, 1792);
        // assert_eq!(part2_answer, 0);
    }
}
