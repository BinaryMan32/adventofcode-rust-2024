use advent_of_code::{create_runner, named, Named, Runner};
use glam::I16Vec2;
use itertools::Itertools;
use std::{collections::{HashSet, VecDeque}, str::Lines};

#[derive(Clone, Copy, Debug, PartialEq)]
enum BoxHalf {
    Left,
    Right,
}

impl BoxHalf {
    const RIGHT_OFFSET: Pos = Pos::new(1, 0);

    fn origin(self, pos: Pos) -> Pos {
        match self {
            Self::Left => pos,
            Self::Right => pos - Self::RIGHT_OFFSET,
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    Box(Option<BoxHalf>),
    Wall,
}

impl Cell {
    fn parse(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            'O' => Cell::Box(None),
            '[' => Cell::Box(Some(BoxHalf::Left)),
            ']' => Cell::Box(Some(BoxHalf::Right)),
            '#' => Cell::Wall,
            u => panic!("unexpected cell character {u}")
        }
    }

    fn is_box_locatable(&self) -> bool {
        match self {
            Self::Box(half) => half.is_none_or(|h| h == BoxHalf::Left),
            _ => false,
        }
    }

    #[cfg(test)]
    fn display(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Box(None) => 'O',
            Cell::Box(Some(BoxHalf::Left)) => '[',
            Cell::Box(Some(BoxHalf::Right)) => ']',
            Cell::Wall => '#',
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

    #[cfg(test)]
    fn display(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
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
    fn parse_narrow(input: Lines) -> Self {
        Self::parse(input, |c| [Some(c), None])
    }

    fn parse_wide(input: Lines) -> Self {
        Self::parse(input, |c: char| {
            match c {
                '#' => [Some('#'), Some('#')],
                'O' => [Some('['), Some(']')],
                '.' => [Some('.'), Some('.')],
                '@' => [Some('@'), Some('.')],
                u => panic!("unxepected char {u}"),
            }
        })
    }
    fn parse(mut input: Lines, expand: impl Fn(char) -> [Option<char>; 2]) -> Self {
        let mut robot = Pos::new(0, 0);
        let cells = input
            .by_ref()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                .flat_map(|c| expand(c).into_iter().flatten())
                .enumerate().map(|(x, c)| {
                    if c == '@' {
                        robot = Pos::new(x as PosComp, y as PosComp);
                        Cell::parse('.')
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
            Cell::Box(_) => self.find_empty_cell(pos + offset, offset),
            Cell::Wall => None,
        }
    }

    fn find_moveable_wide_blocks_horz(&self, pos: Pos, dir: Direction, blocks: &mut HashSet<Pos>) -> bool {
        let offset = dir.offset() * 2;
        let box_offset = if offset.x < 0 { -BoxHalf::RIGHT_OFFSET } else { Pos::ZERO };
        let mut traverse = VecDeque::new();
        traverse.push_back(pos);
        while let Some(pos) = traverse.pop_front() {
            match self.get_cell(&pos) {
                Cell::Empty => (),
                Cell::Box(Some(_)) => {
                    let box_pos = pos + box_offset;
                    if blocks.insert(box_pos) {
                        traverse.push_back(pos + offset)
                    }
                },
                Cell::Box(None) => panic!("narrow boxes use simpler algorithm"),
                Cell::Wall => return false,
            }
        }
        true
    }

    fn find_moveable_wide_blocks_vert(&self, pos: Pos, dir: Direction, blocks: &mut HashSet<Pos>) -> bool {
        let offset = dir.offset();
        let mut traverse = VecDeque::new();
        traverse.push_back(pos);
        while let Some(pos) = traverse.pop_front() {
            match self.get_cell(&pos) {
                Cell::Empty => (),
                Cell::Box(Some(half)) => {
                    let pos = half.origin(pos);
                    if blocks.insert(pos) {
                        let next_pos = pos + offset;
                        traverse.push_back(next_pos);
                        traverse.push_back(next_pos + BoxHalf::RIGHT_OFFSET);
                    }
                },
                Cell::Box(None) => panic!("narrow boxes use simpler algorithm"),
                Cell::Wall => return false,
            }
        }
        true
    }

    fn move_wide_blocks(&mut self, pos: Pos, dir: Direction) -> bool {
        let offset = dir.offset();
        let mut blocks: HashSet<Pos> = HashSet::new();
        let do_move = if offset.x != 0 {
            self.find_moveable_wide_blocks_horz(pos, dir, &mut blocks)
        } else {
            self.find_moveable_wide_blocks_vert(pos, dir, &mut blocks)
        };
        if do_move {
            for pos in blocks.iter() {
                self.set_cell(*pos, Cell::Empty);
                self.set_cell(pos + BoxHalf::RIGHT_OFFSET, Cell::Empty);
            }
            for pos in blocks {
                let dest = pos + offset;
                self.set_cell(dest, Cell::Box(Some(BoxHalf::Left)));
                self.set_cell(dest + BoxHalf::RIGHT_OFFSET, Cell::Box(Some(BoxHalf::Right)));
            }
        }
        do_move
    }

    fn step(&mut self, dir: Direction) {
        let offset = dir.offset();
        let target = self.robot + offset;
        match self.get_cell(&target) {
            Cell::Empty => self.robot = target,
            Cell::Box(None) => {
                if let Some(empty) = self.find_empty_cell(target + offset, &offset) {
                    self.set_cell(target, Cell::Empty);
                    self.set_cell(empty, Cell::Box(None));
                    self.robot = target;
                }
            },
            Cell::Box(Some(_)) => {
                if self.move_wide_blocks(target, dir) {
                    self.robot = target
                }
            }
            Cell::Wall => (),
        };
    }

    #[cfg(test)]
    fn print_map(&self, maybe_dir: Option<Direction>) {
        match maybe_dir {
            Some(dir) => println!("\nMove {c}:", c=dir.display()),
            None => println!("Initial state:"),
        }
        for (y, row) in self.cells.iter().enumerate() {
            let line = row.iter()
                .enumerate()
                .map(|(x, c)| {
                    if self.robot == Pos::new(x as PosComp, y as PosComp) {
                        '@'
                    } else {
                        c.display()
                    }
                })
                .collect::<String>();
            println!("{line}");
        }
    }

    fn simulate(&mut self) {
        #[cfg(test)] self.print_map(None);
        for dir in self.moves.clone() {
            self.step(dir);
            #[cfg(test)] self.print_map(Some(dir));
        }
    }

    fn box_gps_coordinates(&self) -> usize {
        self.cells.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell.is_box_locatable())
                    .map(move |(x, _)| y * 100 + x)
            })
            .sum()
    }
}

fn part1(input: Lines) -> String {
    let mut map = WarehouseMap::parse_narrow(input);
    map.simulate();
    map.box_gps_coordinates().to_string()
}

fn part2(input: Lines) -> String {
    let mut map = WarehouseMap::parse_wide(input);
    map.simulate();
    map.box_gps_coordinates().to_string()
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
        verify!(part2, input, "9021");
    }

    #[test]
    fn example_small() {
        let input = include_str!("example_small.txt");
        verify!(part2, input, "618");
    }
}
