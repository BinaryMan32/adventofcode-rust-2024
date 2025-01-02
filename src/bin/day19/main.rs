use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{iter::repeat, str::Lines};

struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Input {
    fn parse(mut input: Lines) -> Self {
        let patterns = input.next()
            .expect("patterns on first line")
            .split(", ")
            .sorted()
            .map(|s| s.to_owned())
            .collect_vec();
        let designs = input.skip(1)
            .map(|s| s.to_owned())
            .collect_vec();
        Self{patterns, designs}
    }
    
    fn count_ways_to_display_design(&self, design: &str) -> usize {
        // matches[i] stores the number of ways to display design[..i]
        let mut matches: Vec<usize> = Vec::from_iter(repeat(0).take(design.len() + 1));
        matches[0] = 1;
        for i in 0..design.len() {
            for pattern in &self.patterns {
                if design[i..].starts_with(pattern) {
                    matches[i + pattern.len()] += matches[i];
                }
            }
        }
        matches[design.len()]
    }

    fn count_can_display(&self) -> usize {
        self.designs.iter()
            .filter(|design| self.count_ways_to_display_design(design) > 0)
            .count()
    }

    fn count_ways_to_display(&self) -> usize {
        self.designs.iter()
            .map(|design| self.count_ways_to_display_design(design))
            .sum()
    }
}

fn part1(input: Lines) -> String {
    Input::parse(input).count_can_display().to_string()
}

fn part2(input: Lines) -> String {
    Input::parse(input).count_ways_to_display().to_string()
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
        verify!(part1, input, "6");
        verify!(part2, input, "16");
    }
}
