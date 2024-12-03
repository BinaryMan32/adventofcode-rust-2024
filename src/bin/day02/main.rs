use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

fn parse_report(line: &str) -> Vec<i8> {
    line.split_ascii_whitespace().map(|num| num.parse().unwrap()).collect_vec()
}

fn parse_reports(input: Lines) -> Vec<Vec<i8>> {
    input.into_iter().map(parse_report).collect_vec()
}

fn is_report_delta(report: &Vec<i8>, min: i8, max: i8) -> bool {
    report.iter().tuple_windows().map(|(a, b)| b - a).all(|d| d >= min && d <= max)
}

fn is_report_safe(report: &Vec<i8>) -> bool {
    is_report_delta(report, -3, -1) || is_report_delta(report, 1, 3)
}

fn part1(input: Lines) -> String {
    parse_reports(input)
        .into_iter()
        .filter(is_report_safe)
        .count()
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

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "2");
        verify!(part2, input, "0");
    }
}
