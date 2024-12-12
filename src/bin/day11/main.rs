use advent_of_code::{create_runner, named, Named, Runner};
use itertools::{iterate, Itertools};
use num::Integer;
use std::{collections::HashMap, str::Lines};

fn parse_numbers(mut input: Lines) -> Vec<u64> {
    input.next()
        .expect("at least one line")
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("numbers only"))
        .collect_vec()
}

fn build_histogram(stones: Vec<u64>) -> HashMap<u64, usize> {
    stones.into_iter().fold(HashMap::new(), |mut histogram, stone| {
        *histogram.entry(stone).or_insert(0) += 1;
        histogram
    })
}

fn split_even_len_str(stone: u64) -> Option<[u64; 2]> {
    Some(stone.to_string())
        .filter(|s| s.len().is_even())
        .map(|num| {
            let mid = num.len() / 2;
            [&num[..mid], &num[mid..]].map(|n| n.parse().unwrap())
        })
}

fn blink_one_stone<F>(stone: u64, mut out: F)
    where F: FnMut(u64) -> ()
{
    if stone == 0 {
        out(1)
    } else if let Some(stones) = split_even_len_str(stone) {
        stones.into_iter().for_each(out);
    } else {
        out(stone * 2024)
    }
}

fn blink_once(stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut next = HashMap::new();
    for (&stone, count) in stones {
        blink_one_stone(stone, |stone| *next.entry(stone).or_insert(0) += count);
    }
    next
}

fn blink_many(input: Lines, count: usize) -> usize {
    iterate(build_histogram(parse_numbers(input)), blink_once)
        .nth(count)
        .unwrap()
        .into_values()
        .sum()

}

fn part1(input: Lines) -> String {
    blink_many(input, 25).to_string()
}

fn part2(input: Lines) -> String {
    blink_many(input, 75).to_string()
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
        assert_eq!(split_even_len_str(0), None);
        assert_eq!(split_even_len_str(12), Some([1, 2]));
        assert_eq!(split_even_len_str(123), None);
        assert_eq!(split_even_len_str(1234), Some([12, 34]));
        assert_eq!(split_even_len_str(12345), None);
        assert_eq!(split_even_len_str(253000), Some([253, 0]));
        assert_eq!(split_even_len_str(512072), Some([512, 72]));
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "55312");
    }
}
