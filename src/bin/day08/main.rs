use advent_of_code::{create_runner, named, Named, Runner};
use glam::I16Vec2;
use itertools::Itertools;
use std::{cmp::max, collections::HashMap, str::Lines};

type Pos = I16Vec2;

struct AntennaMap {
    size: Pos,
    antennas: HashMap<char, Vec<Pos>>,
}

impl AntennaMap {
    fn parse(input: Lines) -> Self {
        let mut size = Pos::new(0, 0);
        let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();
        for (y, row) in input.enumerate() {
            size.x = max(size.x, row.len() as i16);
            size.y = max(size.y, (y + 1) as i16);
            for (x, c) in row.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_default().push(Pos::new(x as i16, y as i16));
                }
            }
        }
        Self{size, antennas}
    }

    fn find_antinodes(&self, a: &Pos, b: &Pos) -> [Pos; 2] {
        let delta = b - a;
        [a - delta, b + delta]
    }

    fn in_bounds(&self, p: &Pos) -> bool {
        p.x >= 0 && p.x < self.size.x && p.y >= 0 && p.y < self.size.y
    }
    
    fn count_unique_antinodes_in_bounds(&self) -> usize {
        self.antennas.values().flat_map(|positions| {
            positions.iter().tuple_combinations().flat_map(|(a, b)| {
                self.find_antinodes(a, b)
            })
        })
        .filter(|p| self.in_bounds(p))
        .unique()
        .count()
    }
}

fn part1(input: Lines) -> String {
    AntennaMap::parse(input).count_unique_antinodes_in_bounds().to_string()
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
        verify!(part1, input, "14");
        verify!(part2, input, "0");
    }
}
