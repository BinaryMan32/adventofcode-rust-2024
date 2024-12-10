use advent_of_code::{create_runner, named, Named, Runner};
use glam::I16Vec2;
use itertools::Itertools;
use std::str::Lines;

type Pos = I16Vec2;

struct TrailMap {
    height: Vec<Vec<u8>>,
    size: Pos,
}

const OFFSETS: [Pos; 4] = [
    Pos::new(-1, 0),
    Pos::new(1, 0),
    Pos::new(0, -1),
    Pos::new(0, 1),
];

impl TrailMap {
    fn parse(input: Lines) -> Self {
        let height = input.map(|line| {
            line.chars().map(|c| c.to_digit(10).expect("must be a digit") as u8).collect_vec()
        }).collect_vec();
        let size = Pos::new(height[0].len() as i16, height.len() as i16);
        Self{height, size}
    }

    fn get_height(&self, pos: &Pos) -> Option<u8> {
        (pos.x >= 0 && pos.x < self.size.x && pos.y >= 0 && pos.y < self.size.y)
            .then(|| self.height[pos.y as usize][pos.x as usize])
    }

    fn find_trail_ends(&self, pos: Pos, expected_h: u8) -> Vec<Pos> {
        match self.get_height(&pos).filter(|&h| h == expected_h) {
            None => Vec::new(),
            Some(9) => vec![pos],
            Some(h) => {
                OFFSETS.iter()
                    .map(|off| pos + off)
                    .flat_map(|p| self.find_trail_ends(p, h + 1))
                    .collect_vec()
            }       
        }
    }

    fn trailhead_score(&self, pos: &Pos) -> usize {
        self.find_trail_ends(*pos, 0).iter().unique().count()
    }

    fn find_trailheads(&self) -> Vec<Pos> {
        self.height.iter().enumerate()
            .flat_map(|(y, row)|{
                row.iter()
                    .positions(|&h| (h == 0))
                    .map(move |x| Pos::new(x as i16, y as i16))
            })
            .collect_vec()
    }

    fn trailhead_rating(&self, pos: &Pos) -> usize {
        self.find_trail_ends(*pos, 0).len()
    }
}

fn part1(input: Lines) -> String {
    let trail_map = TrailMap::parse(input);
    trail_map.find_trailheads()
        .iter()
        .map(|pos| trail_map.trailhead_score(pos))
        .sum::<usize>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let trail_map = TrailMap::parse(input);
    trail_map.find_trailheads()
        .iter()
        .map(|pos| trail_map.trailhead_rating(pos))
        .sum::<usize>()
        .to_string()}

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
    fn parse() {
        let trail_map = TrailMap::parse("012\n345\n".lines());
        assert_eq!(trail_map.height, vec![vec![0, 1, 2], vec![3, 4, 5]]);
        assert_eq!(trail_map.size, Pos::new(3, 2));
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "36");
        verify!(part2, input, "81");
    }
}
