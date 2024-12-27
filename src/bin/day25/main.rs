use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{array, str::Lines};

#[derive(Debug, PartialEq, Eq, Hash)]
enum SchematicKind {
    Lock,
    Key,
}

impl SchematicKind {
    fn invert(&self) -> Self {
        match self {
            SchematicKind::Key => SchematicKind::Lock,
            SchematicKind::Lock => SchematicKind::Key,
        }
    }
    
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Schematic {
    heights: [u8; Self::NUM_HEIGHTS],
    kind: SchematicKind,
}

impl Schematic {
    const NUM_HEIGHTS: usize = 5;
    const MAX_HEIGHT: u8 = 5;

    fn parse(input: &[&str]) -> Self {
        let heights = array::from_fn(|col| {
            input.iter()
                .map(|row| row.chars().nth(col).unwrap())
                .tuple_windows()
                .position(|(a, b)| a != b)
                .unwrap() as u8
        });
        let mut schematic = Self{heights, kind: SchematicKind::Lock};
        if input[0].starts_with('.') {
            schematic = schematic.invert();
        }
        schematic
    }

    fn invert(&self) -> Self {
        Self {
            heights: self.heights.map(|h| Self::MAX_HEIGHT - h),
            kind: self.kind.invert(),
        }
    }

    fn fits_without_overlapping(&self, other: &Self) -> bool {
        self.kind != other.kind &&
        self.heights.iter().zip(other.heights.iter())
            .all(|(a, b)| a + b <= Self::MAX_HEIGHT)
    }
}

struct Input {
    keys: Vec<Schematic>,
    locks: Vec<Schematic>,
}

impl Input {
    fn parse(input: Lines) -> Self {
        let mut keys = Vec::new();
        let mut locks = Vec::new();
        for chunk in input.filter(|line| !line.is_empty()).chunks(7).into_iter() {
            let schematic = Schematic::parse(&chunk.collect_vec());
            match schematic.kind {
                SchematicKind::Key => keys.push(schematic),
                SchematicKind::Lock => locks.push(schematic),
            };
        }
        Self{keys, locks}
    }
    
    fn count_fits_without_overlapping(&self) -> usize {
        self.locks.iter()
            .map(|lock| {
                self.keys.iter()
                    .filter(|key| key.fits_without_overlapping(lock))
                    .count()
            })
            .sum()
    }
}

fn part1(input: Lines) -> String {
    Input::parse(input)
        .count_fits_without_overlapping()
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
    fn parse_first_lock() {
        let first_lock = include_str!("example.txt").lines().take(7).collect_vec();
        assert_eq!(
            Schematic::parse(&first_lock),
            Schematic{heights: [0, 5, 3, 4, 3], kind: SchematicKind::Lock}
        );
    }

    #[test]
    fn parse_first_key() {
        let first_key = include_str!("example.txt").lines().skip(16).take(7).collect_vec();
        assert_eq!(
            Schematic::parse(&first_key),
            Schematic{heights: [5, 0, 2, 1, 3], kind: SchematicKind::Key}
        );
    }

    #[test]
    fn parse() {
        let input = Input::parse(include_str!("example.txt").lines());
        assert_eq!(input.locks.len(), 2);
        assert_eq!(input.keys.len(), 3);
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "3");
        verify!(part2, input, "0");
    }
}
