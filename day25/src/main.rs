fn main() {
    let answer = run(include_str!("../input"));
    println!("answer: {}", answer);
}

fn run(input: &'static str) -> String {
    let snafu = parse_input(input);
    let sum = snafu.iter().map(|snafu| from_snafu(snafu)).sum();
    to_snafu(sum)
}

fn parse_input(input: &'static str) -> Vec<&'static str> {
    input.split_whitespace().collect()
}

fn from_snafu(snafu: &str) -> i64 {
    let mut ret = 0;
    for (i, c) in snafu.chars().rev().enumerate() {
        let s = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("invalid snafu digit: {}", c),
        };
        ret += 5i64.pow(i as u32) * s;
    }
    ret
}

fn to_snafu(n: i64) -> String {
    let mut n = n;
    let mut ret: Vec<char> = Vec::new();

    while n > 0 {
        let quot = n / 5;
        let rem = n % 5;

        let (c, o) = match rem {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => panic!("remainder too high: {}", rem),
        };

        ret.push(c);
        n = quot + o;
    }
    ret.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_snafu() {
        assert_eq!(from_snafu("1"), 1);
        assert_eq!(from_snafu("2"), 2);
        assert_eq!(from_snafu("1="), 3);
        assert_eq!(from_snafu("1-"), 4);
        assert_eq!(from_snafu("1-0---0"), 12345);
        assert_eq!(from_snafu("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(4890), "2=-1=0");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(32), "112");
    }

    #[test]
    fn test_example_parse() {
        let snafu = parse_input(include_str!("../input-example"));
        assert_eq!(snafu.len(), 13);
    }

    #[test]
    fn test_example_answer() {
        let answer = run(include_str!("../input-example"));
        assert_eq!(answer, "2=-1=0");
    }

    #[test]
    fn test_input_answer() {
        let answer = run(include_str!("../input"));
        assert_eq!(answer, "2=01-0-2-0=-0==-1=01");
    }
}
