use advent_of_code::{create_runner, named, Named, Runner};
use num::abs;
use std::str::Lines;

fn parse_lists(input: Lines) -> (Vec<i64>, Vec<i64>) {
    input.map(|line| {
        let mut numbers = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap());
        let x = numbers.next().unwrap();
        let y = numbers.next().unwrap();
        (x, y)
    }).fold((Vec::new(), Vec::new()), |(mut xs, mut ys), (x, y)| {
        xs.push(x);
        ys.push(y);
        (xs, ys)
    })
}

fn part1(input: Lines) -> String {
    let (mut xs, mut ys) = parse_lists(input);
    xs.sort();
    ys.sort();
    xs.iter().zip(ys.iter())
        .map(|(x, y)| abs(x - y))
        .sum::<i64>()
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
        verify!(part1, input, "11");
        verify!(part2, input, "0");
    }
}
