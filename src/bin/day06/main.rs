use advent_of_code::{create_runner, named, Named, Runner};
use glam::I16Vec2;
use itertools::Itertools;
use std::{collections::HashSet, iter::successors, str::Lines};

type Pos = I16Vec2;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Guard {
    pos: Pos,
    dir: Direction,
}

impl Guard {
    fn new(pos: Pos) -> Self {
        Self{ pos, dir: Direction::North }
    }

    fn forward_pos(&self) -> Pos {
        self.pos + self.dir.offset()
    }
    
    fn move_to(&self, pos: Pos) -> Self {
        Self{pos, dir: self.dir}
    }

    fn turn_right(&self) -> Self {
        Self{pos: self.pos, dir: self.dir.right()}
    }

    fn next<M: LabMap>(self, lab_map: &M) -> Option<Self> {
        let forward_pos = self.forward_pos();
        lab_map.is_obstacle(&forward_pos).map(|is_obstacle| {
            if is_obstacle {
                self.turn_right()
            } else {
                self.move_to(forward_pos)
            }
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn offset(self) -> Pos {
        match self {
            Self::North => Pos{x:0, y:-1},
            Self::East => Pos{x:1, y:0},
            Self::South => Pos{x:0, y:1},
            Self::West => Pos{x:-1, y:0},
        }
    }
    fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

trait LabMap {
    fn is_obstacle(&self, pos: &Pos) -> Option<bool>;
}

struct OriginalMap {
    obstacles: Vec<Vec<bool>>,
    size: Pos,
}

impl OriginalMap {
    fn new(obstacles: Vec<Vec<bool>>) -> Self {
        let size = Pos{x: obstacles[0].len() as i16, y: obstacles.len() as i16};
        Self{obstacles, size}
    }

    fn add_obstacle(&self, pos: Pos) -> ModifiedMap {
        ModifiedMap{underlying: self, obstacle: pos}
    }
}

impl LabMap for OriginalMap {
    fn is_obstacle(&self, pos: &Pos) -> Option<bool> {
        if pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y {
            None
        } else {
            Some(self.obstacles[pos.y as usize][pos.x as usize])
        }
    }
}

struct ModifiedMap<'a> {
    underlying: &'a OriginalMap,
    obstacle: Pos,
}

impl LabMap for ModifiedMap<'_> {
    fn is_obstacle(&self, pos: &Pos) -> Option<bool> {
        if *pos == self.obstacle {
            Some(true)
        } else {
            self.underlying.is_obstacle(pos)
        }
    }
}

fn parse_input(input: Lines) -> (OriginalMap, Guard) {
    let mut guard_pos: Option<Pos> = None;
    let obstacles = input.enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => false,
                '#' => true,
                '^' => {
                    guard_pos = Some(Pos{x: x as i16, y: y as i16});
                    false
                },
                _ => panic!("unexpected char {c} at ({x}, {y})")
            }
        }).collect_vec()
    }).collect_vec();
    (OriginalMap::new(obstacles), Guard::new(guard_pos.expect("didn't find guard")))
}

fn part1(input: Lines) -> String {
    let (lab_map, guard_start) = parse_input(input);
    let mut positions = HashSet::new();
    for guard in successors(Some(guard_start), |g| g.next(&lab_map)) {
        positions.insert(guard.pos);
    }
    positions.len().to_string()
}

fn is_guard_stuck_in_loop(guard_start: Guard, lab_map: &ModifiedMap) -> bool {
    // detect a loop by storing all past guard states
    let mut guard_states = HashSet::new();
    for guard in successors(Some(guard_start), |g| g.next(lab_map)) {
        if !guard_states.insert(guard) {
            return true
        }
    }
    false
}

fn part2(input: Lines) -> String {
    let (lab_map, guard) = parse_input(input);
    (0..lab_map.size.x)
        .flat_map(|x| (0..lab_map.size.y).map(move |y| Pos{x, y}))
        .filter(|&obstacle| is_guard_stuck_in_loop(guard, &lab_map.add_obstacle(obstacle)))
        .count()
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
        verify!(part1, input, "41");
        verify!(part2, input, "6");
    }
}
