use advent_of_code::{create_runner, named, Named, Runner};
use lazy_regex::{Regex, Lazy, lazy_regex};
use std::str::Lines;

pub static MUL_REGEX: Lazy<Regex> = lazy_regex!(r#"mul\(([0-9]{1,3}),([0-9]{1,3})\)"#);

fn part1(input: Lines) -> String {
    input
        .flat_map(|line| MUL_REGEX.captures_iter(line))
        .map(|m| {
            m.iter()
                .skip(1)
                .flatten()
                .map(|n| n.as_str().parse::<u64>().unwrap())
                .product::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

pub static DO_DONT_MUL_REGEX: Lazy<Regex> = lazy_regex!(r#"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)"#);

fn part2(input: Lines) -> String {
    input
        .flat_map(|line| DO_DONT_MUL_REGEX.captures_iter(line))
        .fold((true, 0),|(enabled, sum), m| {
            match (enabled, &m[0]) {
                (_, "do()") => (true, sum),
                (_, "don't()") => (false, sum),
                (true, _) => (enabled, sum + m.iter()
                    .skip(1)
                    .flatten()
                    .map(|n| n.as_str().parse::<u64>().unwrap())
                    .product::<u64>()
                ),
                (false, _) => (enabled, sum)
            }
        })
        .1
        .to_string()
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

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        let input2 = include_str!("example2.txt");
        verify!(part1, input, "161");
        verify!(part1, input2, "161");
        verify!(part2, input, "161");
        verify!(part2, input2, "48");
    }
}
