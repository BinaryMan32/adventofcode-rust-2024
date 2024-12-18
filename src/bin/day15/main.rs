use advent_of_code::{create_runner, named, Named, Runner};
use glam::I16Vec2;
use itertools::Itertools;
use std::str::Lines;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    Box,
    Wall,
}

impl Cell {
    fn parse(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            'O' => Cell::Box,
            '#' => Cell::Wall,
            u => panic!("unexpected cell character {u}")
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            u => panic!("unexpected direction character {u}")
        }
    }

    fn offset(self) -> Pos {
        match self {
            Direction::Up => Pos::new(0, -1),
            Direction::Down => Pos::new(0, 1),
            Direction::Left => Pos::new(-1, 0),
            Direction::Right => Pos::new(1, 0),
        }
    }  
}

struct WarehouseMap {
    cells: Vec<Vec<Cell>>,
    robot: Pos,
    moves: Vec<Direction>,
}

type Pos = I16Vec2;
type PosComp = i16;

impl WarehouseMap {
    fn parse(mut input: Lines) -> Self {
        let mut robot = Pos::new(0, 0);
        let cells = input
            .by_ref()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().map(|(x, c)| {
                    if c == '@' {
                        robot = Pos::new(x as PosComp, y as PosComp);
                        Cell::Empty
                    } else {
                        Cell::parse(c)
                    }
                })
                .collect_vec()
            })
            .collect_vec();
        let moves = input
            .flat_map(|line| {
                line.chars().map(Direction::parse)
            })
            .collect_vec();
        Self{cells, robot, moves}
    }

    fn get_cell(&self, pos: &Pos) -> Cell {
        self.cells[pos.y as usize][pos.x as usize]
    }

    fn set_cell(&mut self, pos: Pos, c: Cell) {
        self.cells[pos.y as usize][pos.x as usize] = c;
    }

    fn find_empty_cell(&self, pos: Pos, offset: &Pos) -> Option<Pos> {
        match self.get_cell(&pos) {
            Cell::Empty => Some(pos),
            Cell::Box => self.find_empty_cell(pos + offset, offset),
            Cell::Wall => None,
        }
    }

    fn step(&mut self, dir: Direction) {
        let offset = dir.offset();
        let target = self.robot + offset;
        match self.get_cell(&target) {
            Cell::Empty => self.robot = target,
            Cell::Box => {
                if let Some(empty) = self.find_empty_cell(target + offset, &offset) {
                    self.set_cell(target, Cell::Empty);
                    self.set_cell(empty, Cell::Box);
                    self.robot = target;
                }
            },
            Cell::Wall => (),
        };
    }

    fn simulate(&mut self) {
        for dir in self.moves.clone() {
            self.step(dir);
        }
    }

    fn box_gps_coordinates(&self) -> usize {
        self.cells.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell == Cell::Box)
                    .map(move |(x, _)| y * 100 + x)
            })
            .sum()
    }
}

fn part1(input: Lines) -> String {
    let mut map = WarehouseMap::parse(input);
    map.simulate();
    map.box_gps_coordinates().to_string()
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
        verify!(part1, input, "10092");
        verify!(part2, input, "0");
    }
}
