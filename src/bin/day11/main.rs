use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use num::Integer;
use std::str::Lines;

fn parse_numbers(mut input: Lines) -> Vec<u64> {
    input.next()
        .expect("at least one line")
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("numbers only"))
        .collect_vec()
}

fn maybe_even_len_str(stone: u64) -> Option<String> {
    Some(stone.to_string()).filter(|s| s.len().is_even())
}

fn split_even_len_str(num: &str) -> [u64; 2] {
    let mid = num.len() / 2;
    [&num[..mid], &num[mid..]].map(|n| n.parse().unwrap())
}

fn count_stones(stone: u64, blinks: usize) -> usize {
    if blinks == 0 {
        1
    } else if stone == 0 {
        count_stones(1, blinks - 1)
    } else if let Some(even_len_str) = maybe_even_len_str(stone) {
        if blinks == 1 {
            2
        } else {
            split_even_len_str(&even_len_str)
                .map(|n| count_stones(n, blinks - 1))
                .into_iter()
                .sum()
        }
    } else {
        count_stones(stone * 2024, blinks - 1)
    }
}

fn part1(input: Lines) -> String {
    parse_numbers(input)
        .into_iter()
        .map(|n| count_stones(n, 25))
        .sum::<usize>()
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
    fn test_split_even_len_str() {
        assert_eq!(split_even_len_str(&"253000"), [253, 0]);
        assert_eq!(split_even_len_str(&"512072"), [253, 0]);
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "55312");
        verify!(part2, input, "0");
    }
}
