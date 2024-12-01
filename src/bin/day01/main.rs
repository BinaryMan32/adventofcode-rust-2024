use advent_of_code::{create_runner, named, Named, Runner};
use num::abs;
use std::{collections::HashMap, str::Lines};

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
    let (xs, ys) = parse_lists(input);
    let y_counts: HashMap<i64, i64> = ys.iter().fold(HashMap::new(), |mut counts, y| {
        counts.entry(*y).and_modify(|c| *c += 1).or_insert(1);
        counts
    });
    xs.iter()
        .map(|x| x * y_counts.get(x).unwrap_or(&0))
        .sum::<i64>()
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
        verify!(part1, input, "11");
        verify!(part2, input, "31");
    }
}
