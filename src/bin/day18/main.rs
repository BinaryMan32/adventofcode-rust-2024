use advent_of_code::{create_runner, named, Named, Runner};
use glam::I8Vec2;
use itertools::{repeat_n, Itertools};
use std::{collections::VecDeque, str::Lines};

type Pos = I8Vec2;
const DIRECTIONS: [Pos; 4] = [
    Pos::new(1, 0),
    Pos::new(0, -1),
    Pos::new(-1, 0),
    Pos::new(0, 1),
];

fn pos_to_string(pos: &Pos) -> String {
    format!("{x},{y}", x=pos.x, y=pos.y)
}

struct Input {
    part1_size: usize,
    coordinates: Vec<Pos>,
    size: Pos,
}

impl Input {
    fn parse(mut input: Lines) -> Self {
        let part1_size = input.by_ref()
            .next()
            .expect("first line")
            .parse::<usize>()
            .expect("part1_size numeric");
        let coordinates = input
            .map(|line| {
                let (x, y) = line.split(',').map(|n| n.parse::<i8>().expect("numeric")).collect_tuple().expect("2 components");
                Pos{x, y}
            })
            .collect_vec();
        let max_x = coordinates.iter().map(|c| c.x).max().expect("at least one x");
        let max_y = coordinates.iter().map(|c| c.y).max().expect("at least one y");
        let size = Pos::new(max_x + 1, max_y + 1);
        Self{part1_size, coordinates, size}
    }
}

struct Space<'a> {
    input: &'a Input,
    corrupted: Vec<Vec<bool>>,
    steps: Vec<Vec<Option<usize>>>,
}

impl<'a> Space<'a> {
    fn new(input: &'a Input) -> Self {
        let corrupted = repeat_n(
            repeat_n(false, input.size.x as usize).collect_vec(),
            input.size.y as usize
        ).collect_vec();
        let steps = repeat_n(
            repeat_n(None, input.size.x as usize).collect_vec(),
            input.size.y as usize
        ).collect_vec();
        Self{input, corrupted, steps}
    }
    fn corrupt_first_n(&mut self, n: usize) {
        for pos in self.input.coordinates.iter().take(n) {
            self.corrupt(pos);
        }
    }
    fn corrupt(&mut self, pos: &Pos) {
        self.corrupted[pos.y as usize][pos.x as usize] = true;
    }
    fn is_safe(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < self.input.size.x
            && pos.y >= 0 && pos.y < self.input.size.y
            && !self.corrupted[pos.y as usize][pos.x as usize]
    }
    fn set_better(&mut self, pos: &Pos, steps: usize) -> bool {
        let old_steps = &mut self.steps[pos.y as usize][pos.x as usize];
        if old_steps.is_none_or(|old| steps < old) {
            *old_steps = Some(steps);
            true
        } else {
            false
        }
    }
    fn min_steps(&mut self) -> Option<usize> {
        let mut traverse = VecDeque::new();
        if self.steps[0][0].is_some() {
            self.steps.iter_mut().for_each(|row| row.fill(None));
        }
        traverse.push_back((Pos::new(0, 0), 0));
        while let Some((pos, steps)) = traverse.pop_front() {
            if self.set_better(&pos, steps) {
                for dir in DIRECTIONS.iter() {
                    let new_pos = pos + dir;
                    if self.is_safe(&new_pos) {
                        traverse.push_back((new_pos, steps + 1));
                    }
                }
            }
        }
        self.steps[self.input.size.y as usize - 1][self.input.size.x as usize - 1]
    }
}

fn part1(input: Lines) -> String {
    let input = Input::parse(input);
    let mut space = Space::new(&input);
    space.corrupt_first_n(input.part1_size);
    space.min_steps().expect("exit visited").to_string()
}

fn part2(input: Lines) -> String {
    let input = Input::parse(input);
    let mut space = Space::new(&input);
    space.corrupt_first_n(input.part1_size);
    pos_to_string(
        input.coordinates[input.part1_size..].iter()
        .find(|pos| {
            space.corrupt(pos);
            space.min_steps().is_none()
        })
        .expect("some blocked")
    )
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
        verify!(part1, input, "22");
        verify!(part2, input, "6,1");
    }
}
