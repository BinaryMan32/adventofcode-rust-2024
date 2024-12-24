use advent_of_code::{create_runner, named, Named, Runner};
use itertools::iterate;
use std::str::Lines;


type Secret = u64;

fn mix(a: Secret, b: Secret) -> Secret {
    a ^ b
}

fn prune(a: Secret) -> Secret {
    a & 0xffffff
}

fn next_secret_number(num: Secret) -> Secret {
    let num = prune(mix(num, num << 6));
    let num = prune(mix(num, num >> 5));
    prune(mix(num, num << 11))
}

fn nth_secret_number(num: Secret, nth: usize) -> Secret {
    iterate(num, |n| next_secret_number(*n))
        .nth(nth)
        .expect("nth must exist")
}

fn part1(input: Lines) -> String {
    input
        .map(|line| line.parse::<Secret>().expect("numeric"))
        .map(|n| nth_secret_number(n, 2000))
        .sum::<Secret>()
        .to_string()
}

fn part2(input: Lines) -> String {
    input.take(0).count().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let runner: &Runner = create_runner!();
    runner.run(named!(part1), input);
    runner.run(named!(part2), input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::verify;
    use itertools::Itertools;

    #[test]
    fn test_next() {
        assert_eq!(
            iterate(123, |n| next_secret_number(*n)).take(11).collect_vec(),
            vec![
                123,
                15887950,
                16495136,
                527345,
                704524,
                1553684,
                12683156,
                11100544,
                12249484,
                7753432,
                5908254,
            ]
        );

    }

    #[test]
    fn test_nth() {
        assert_eq!(nth_secret_number(1, 2000), 8685429);
        assert_eq!(nth_secret_number(10, 2000), 4700978);
        assert_eq!(nth_secret_number(100, 2000), 15273692);
        assert_eq!(nth_secret_number(2024, 2000), 8667524);
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "37327623");
        verify!(part2, input, "0");
    }
}