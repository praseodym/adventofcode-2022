use std::collections::HashSet;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let input = input.trim();
    let part1_answer = distinct_chars_offset(input, 4);
    let part2_answer = distinct_chars_offset(input, 14);
    (part1_answer, part2_answer)
}

fn distinct_chars_offset(input: &str, n: usize) -> u32 {
    for i in n..input.len() {
        let mut set = HashSet::new();
        let mut chars = input.chars().skip(i - n);
        for _ in 0..n {
            let c = chars.next().unwrap();
            set.insert(c);
        }
        if set.len() == n {
            return i as u32;
        }
    }
    panic!("offset not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_answer() {
        assert_eq!((7, 19), run("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!((5, 23), run("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!((6, 23), run("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!((10, 29), run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!((11, 26), run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1093);
        assert_eq!(part2_answer, 3534);
    }
}
