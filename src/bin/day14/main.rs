use advent_of_code::{create_runner, named, Named, Runner};
use glam::IVec2;
use itertools::Itertools;
use lazy_regex::{lazy_regex, Lazy, Regex};
use std::{cmp::Ordering, str::Lines};
type Pos = IVec2;
type Comp = i32;
struct Robot {
    pos: Pos,
    vel: Pos,
}

pub static ROBOT_REGEX: Lazy<Regex> = lazy_regex!(r#"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)"#);

impl Robot {
    fn parse(line: &str) -> Self {
        let (px, py, vx, vy) = ROBOT_REGEX.captures(line)
            .expect("robot match")
            .iter()
            .skip(1)
            .map(|n| n.expect("group").as_str().parse().expect("robot values numeric"))
            .collect_tuple()
            .expect("4 groups");
        Self{pos: Pos::new(px, py), vel: Pos::new(vx, vy)}
    }
}

struct RestroomMap {
    size: Pos,
    robots: Vec<Robot>,
}

impl RestroomMap {
    fn parse(mut input: Lines) -> Self {
        let (size_x, size_y) = input.next().expect("size required")
            .split_whitespace()
            .flat_map(|n| n.parse().ok())
            .collect_tuple()
            .expect("size has 2 numeric");
        let size = Pos::new(size_x, size_y);
        let robots = input.map(Robot::parse).collect_vec();
        Self{size, robots}
    }
    
    fn initial_positions(&self) -> Vec<Pos> {
        self.robots.iter().map(|r| r.pos).collect_vec()
    }

    fn wrap_position(&self, pos: Pos) -> Pos {
        Pos::new(pos.x.rem_euclid(self.size.x), pos.y.rem_euclid(self.size.y))
    }

    fn advance_positions(&self, positions: &[Pos], seconds: usize) -> Vec<Pos> {
        positions.iter().zip(self.robots.iter())
            .map(|(pos, robot)| {
                self.wrap_position(pos + robot.vel * seconds as Comp)
            })
            .collect_vec()
    }

    fn quadrant_component(component: Comp, mid: Comp, weight: usize) -> Option<usize> {
        match component.cmp(&mid) {
            Ordering::Less => Some(0),
            Ordering::Equal => None,
            Ordering::Greater => Some(weight),
        }
    }

    fn quadrant(&self, pos: &Pos) -> Option<usize> {
        let mid = self.size / 2;
        let qx = Self::quadrant_component(pos.x, mid.x, 1);
        let qy = Self::quadrant_component(pos.y, mid.y, 2);
        qx.zip(qy).map(|(qx, qy)| qx + qy)
    }

    fn safety_factor(&self, positions: &[Pos]) -> usize {
        let mut counts = [0; 4];
        for pos in positions {
            if let Some(quadrant) = self.quadrant(pos) {
                counts[quadrant] += 1;
            }
        }
        counts.into_iter().product()
    }
}

fn part1(input: Lines) -> String {
    let map = RestroomMap::parse(input);
    let positions = map.advance_positions(&map.initial_positions(), 100);
    map.safety_factor(&positions).to_string()
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
    fn wrap_position() {
        let map = RestroomMap::parse("5 5".lines());
        assert_eq!(map.wrap_position(Pos::new(-1, -1)), Pos::new(4, 4));
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "12");
        verify!(part2, input, "0");
    }
}
