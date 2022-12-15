use regex::Regex;
use std::cmp;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"), 2000000, 4000000);
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str, y: i32, max: i32) -> (u32, i64) {
    let mut part1_answer: u32 = 0;
    let mut part2_answer: i64 = 0;

    let srs = parse_input(input);
    'outer: for x in -5000000..5000000 {
        for sr in &srs {
            if sr.beacon.0 == x && sr.beacon.1 == y {
                continue 'outer;
            }
        }

        for sr in &srs {
            let d = distance(sr.sensor.0, sr.sensor.1, x, y);
            let z = d <= sr.distance;
            if z {
                part1_answer += 1;
                continue 'outer;
            }
        }
    }

    for sr in &srs {
        let a = (sr.sensor.0, sr.sensor.1 - sr.distance);
        let b = (sr.sensor.0 + sr.distance, sr.sensor.1);
        let c = (sr.sensor.0, sr.sensor.1 + sr.distance);
        let d = (sr.sensor.0 - sr.distance, sr.sensor.1);
        if let Some(ans) = is_free(&srs, max, a.0 + 1, a.1 + 1, b.0 + 1, b.1) {
            part2_answer = ans;
            break;
        }
        if let Some(ans) = is_free(&srs, max, b.0, b.1 + 1, c.0, c.1 + 1) {
            part2_answer = ans;
            break;
        }
        if let Some(ans) = is_free(&srs, max, c.0 - 1, c.1, d.0 - 1, d.1) {
            part2_answer = ans;
            break;
        }
        if let Some(ans) = is_free(&srs, max, a.0, a.1 - 1, d.0 - 1, d.1) {
            part2_answer = ans;
            break;
        }
    }

    (part1_answer, part2_answer)
}

fn is_free(srs: &Vec<SensorReading>, max: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> Option<i64> {
    let min = 0;
    let x1 = cmp::max(cmp::min(x1, max), min);
    let x2 = cmp::max(cmp::min(x2, max), min);
    let y1 = cmp::max(cmp::min(y1, max), min);
    let y2 = cmp::max(cmp::min(y2, max), min);
    let dx = if x1 <= x2 { 1 } else { -1 };
    let dy = if y1 <= y2 { 1 } else { -1 };
    let mut x = x1;
    let mut y = y1;
    'outer: while x != x2 && y != y2 {
        for sr in srs {
            let d = distance(sr.sensor.0, sr.sensor.1, x, y);
            let z = d <= sr.distance;
            if z {
                x += dx;
                y += dy;
                continue 'outer;
            }
        }
        return Some((4000000i64 * x as i64) + y as i64);
    }
    None
}

#[derive(Debug)]
struct SensorReading {
    sensor: (i32, i32),
    beacon: (i32, i32),
    distance: i32,
}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    i32::abs(x2 - x1) + i32::abs(y2 - y1)
}

fn parse_input(input: &'static str) -> Vec<SensorReading> {
    let mut ret = Vec::new();
    let re =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();
    for line in input.trim_end().split('\n') {
        let cap = re.captures(line).unwrap();
        let sensor = (
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );
        let beacon = (
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        );
        let distance = distance(sensor.0, sensor.1, beacon.0, beacon.1);
        let sr = SensorReading {
            sensor,
            beacon,
            distance,
        };
        ret.push(sr);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let lines = parse_input(include_str!("../input-example"));
        assert_eq!(lines.len(), 14);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"), 10, 20);
        assert_eq!(part1_answer, 26);
        assert_eq!(part2_answer, 56000011);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"), 2000000, 4000000);
        assert_eq!(part1_answer, 6124805);
        assert_eq!(part2_answer, 12555527364986);
    }
}
