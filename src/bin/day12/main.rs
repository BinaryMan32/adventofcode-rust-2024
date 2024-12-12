use advent_of_code::{create_runner, named, Named, Runner};
use glam::I16Vec2;
use itertools::Itertools;
use std::{collections::{HashMap, VecDeque}, str::Lines};

type Pos = I16Vec2;

const DIRECTIONS: [Pos; 4] = [
    Pos::new(0, -1),
    Pos::new(-1, 0),
    Pos::new(1, 0),
    Pos::new(0, 1),
];

struct GardenPlotMap {
    labels: Vec<Vec<Option<char>>>,
    size: Pos,
}

impl GardenPlotMap {
    fn parse(input: Lines) -> Self {
        let labels = input.into_iter()
            .map(|line| {
                line.chars().map(Some).collect_vec()
            })
            .collect_vec();
        let size = Pos::new(labels[0].len() as i16, labels.len() as i16);
        Self{labels, size}
    }

    fn get_unsafe(&self, pos: Pos) -> Option<char> {
        self.labels[pos.y as usize][pos.x as usize]
    }

    fn get(&self, pos: Pos) -> Option<char> {
        (pos.x >= 0 && pos.x < self.size.x && pos.y >= 0 && pos.y < self.size.y)
            .then(|| self.get_unsafe(pos))
            .flatten()
    }

    fn clear(&mut self, pos: Pos) {
        self.labels[pos.y as usize][pos.x as usize] = None;
    }

    fn get_region(&mut self, pos: Pos) -> Region {
        let region_label = self.get_unsafe(pos);
        let mut traverse: VecDeque<Pos> = VecDeque::new();
        let mut area: usize = 0;
        let mut traversals: usize = 0;
        traverse.push_back(pos);
        while let Some(pos) = traverse.pop_front() {
            if self.get_unsafe(pos).is_some() {
                for offset in DIRECTIONS.iter() {
                    let new_pos = pos + offset;
                    if self.get(new_pos) == region_label {
                        traverse.push_back(new_pos);
                    }
                }
                area += 1;
                self.clear(pos);
            }
            traversals += 1
        }
        let total_sides = 4 * area;
        let internal_sides = 2 * (traversals - 1);
        Region{area, perimeter: total_sides - internal_sides}
    }

    fn get_regions(&mut self) -> HashMap<char, Vec<Region>> {
        let mut regions: HashMap<char, Vec<Region>> = HashMap::new();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Pos::new(x, y);
                if let Some(label) = self.get_unsafe(pos) {
                    regions.entry(label).or_default().push(self.get_region(pos));
                } 
            }
        }
        regions
    }
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.perimeter
    }
}

fn part1(input: Lines) -> String {
    let regions = GardenPlotMap::parse(input).get_regions();
    #[cfg(test)] dbg!(&regions);
    regions
        .values()
        .flat_map(|rs| rs.iter().map(|r| r.price()))
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
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "1930");
        verify!(part2, input, "0");
    }
}
