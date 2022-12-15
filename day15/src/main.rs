use regex::Regex;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"), 2000000);
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str, y: i32) -> (u32, u32) {
    let mut part1_answer: u32 = 0;
    let part2_answer: u32 = 0;

    let srs = parse_input(input);
    'outer: for x in -10000000..10000000 {
        // 'outer: for x in -4..27 {
        for sr in &srs {
            if sr.beacon.0 == x && sr.beacon.1 == y {
                // print!("B");
                continue 'outer;
            }
        }

        let mut covered = false;
        for sr in &srs {
            let d = distance(sr.sensor.0, sr.sensor.1, x, y);
            let z = d <= sr.distance;
            //println!("({},{}) d: {:?}, b: {:?}", x, y, d, b);
            if z {
                covered = true;
                break;
            }
        }

        if covered {
            // print!("#");
            part1_answer += 1;
        } else {
            // print!(".");
        }
    }
    // println!("{}", part1_answer);
    // println!();

    (part1_answer, part2_answer)
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
        println!("line: {}", line);
        let cap = re.captures(line).unwrap();
        let sensor = (
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );
        let beacon = (
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        );
        //let distance = i32::abs(sensor.0 - beacon.0) + i32::abs(sensor.0 - beacon.0);
        let distance = distance(sensor.0, sensor.1, beacon.0, beacon.1);
        let sr = SensorReading {
            sensor,
            beacon,
            distance,
        };
        println!("{:?}", sr);
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
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"), 10);
        assert_eq!(part1_answer, 26);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"), 2000000);
        assert_eq!(part1_answer, 6124805);
        // assert_eq!(part2_answer, 0);
    }
}
