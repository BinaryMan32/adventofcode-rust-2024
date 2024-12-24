use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{collections::HashMap, str::Lines};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn parse_char(c: char) -> Self {
        match c {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            p => panic!("unrecognized color code {p}")
        }
    }

    fn parse_str(s: &str) -> Vec<Self> {
        s.chars().map(Self::parse_char).collect_vec()
    }
}

struct Input {
    patterns: Vec<Vec<Color>>,
    designs: Vec<Vec<Color>>,
}

impl Input {
    fn parse(mut input: Lines) -> Self {
        let patterns = input.next()
            .expect("patterns on first line")
            .split(", ")
            .map(Color::parse_str)
            .sorted()
            .collect_vec();
        let designs = input.skip(1)
            .map(Color::parse_str)
            .collect_vec();
        Self{patterns, designs}
    }
    
}


#[derive(Default)]
struct PatternNode {
    next: HashMap<Color, PatternNode>,
    is_end: bool,    
}

impl PatternNode {
    fn new(patterns: Vec<Vec<Color>>) -> Self {
        Self::new_from_slice(
            patterns.iter()
                .map(|p| &p[..])
                .collect_vec()
        )
    }
    fn new_from_slice(patterns: Vec<&[Color]>) -> Self {
        let mut out: PatternNode = Default::default();
        for (first, group) in &patterns.into_iter().chunk_by(|p| p.first()) {
            if let Some(c) = first {
                out.next.insert(*c, Self::new_from_slice(group.map(|p| &p[1..]).collect_vec()));
            } else {
                out.is_end = true;
            }
        }
        out
    }
    fn can_display(&self, design: &[Color]) -> bool {
        self.can_display_aux(design, self)
    }
    fn can_display_aux(&self, design: &[Color], root: &Self) -> bool {
        match design.first() {
            None => self.is_end,
            Some(c) => self.next.get(c).is_some_and(|p| p.can_display_aux(&design[1..], root))
                || self.is_end && root.can_display_aux(design, root)
        }
    }
}

fn part1(input: Lines) -> String {
    let input = Input::parse(input);
    let patterns = PatternNode::new(input.patterns);
    input.designs.into_iter()
        .filter(|d| patterns.can_display(d))
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
        verify!(part1, input, "6");
        verify!(part2, input, "0");
    }
}
