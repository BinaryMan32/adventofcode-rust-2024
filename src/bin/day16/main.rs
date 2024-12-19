use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{collections::{HashSet, VecDeque}, iter::repeat_n, ops::Add, str::Lines};

#[derive(Clone, Copy)]
enum Direction {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}
const DIRECTIONS: [Direction; 4] = [
    Direction::East,
    Direction::North,
    Direction::West,
    Direction::South,
];

impl Direction {
    fn cw(&self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    fn ccw(&self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        }
    }

    fn flip(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Add<Direction> for Pos {
    type Output = Pos;

    fn add(mut self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::East => self.x += 1,
            Direction::North => self.y -= 1,
            Direction::West => self.x -= 1,
            Direction::South => self.y += 1,
        }
        self
    }
}

struct Maze {
    tiles: Vec<Vec<bool>>,
    size: Pos,
    start: Pos,
    end: Pos, 
}

impl Maze {
    fn parse(input: Lines) -> Self {
        let mut start = Pos{x: 0, y: 0};
        let mut end = Pos{x: 0, y: 0};
        let tiles = input.into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.chars().enumerate()
                    .map(|(x, c)| {
                        match c {
                            '#' => false,
                            '.' => true,
                            'S' => {
                                start = Pos{x, y};
                                true
                            },
                            'E' => {
                                end = Pos{x, y};
                                true
                            },
                            u => panic!("unexpected char {u}")
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        let size = Pos{x: tiles[0].len(), y: tiles.len()};
        Self{tiles, size, start, end}
    }
    
    fn is_empty(&self, pos: &Pos) -> bool {
        self.tiles[pos.y][pos.x]
    }
}

type Score = u64;

struct Solver<'a> {
    maze: &'a Maze,
    best: Vec<Vec<[Option<Score>; 4]>>,
}

impl<'a> Solver<'a> {
    fn new(maze: &'a Maze) -> Self {
        let best = repeat_n(
            repeat_n([None; 4], maze.size.x)
                .collect_vec(),
            maze.size.y
        ).collect_vec();
        Self{maze, best}
    }

    fn set_better(&mut self, pos: &Pos, dir: Direction, score: Score) -> bool {
        let existing = &mut self.best[pos.y][pos.x][dir as usize];
        if existing.is_none_or(|old| score < old) {
            *existing = Some(score);
            true
        } else {
            false
        }
    }

    fn best_score_at(&self, pos: &Pos) -> Score {
        *self.best[pos.y][pos.x].iter().flatten().min().expect("didn't reach end")
    }

    fn min_score_to_end(&mut self) -> Score {
        let mut traverse = VecDeque::new();
        traverse.push_back((self.maze.start.clone(), Direction::East, 0));
        while let Some((pos, dir, score)) = traverse.pop_front() {
            if self.maze.is_empty(&pos) && self.set_better(&pos, dir, score) {
                traverse.push_back((pos.clone() + dir, dir, score + 1));
                traverse.push_back((pos.clone(), dir.cw(), score + 1000));
                traverse.push_back((pos, dir.ccw(), score + 1000));
            }
        }
        self.best_score_at(&self.maze.end)
    }

    fn find_best_path_tiles(&self, pos: &Pos, dir: Direction, score: Score, tiles: &mut HashSet<Pos>) {
        if self.best[pos.y][pos.x][dir as usize] == Some(score) {
            tiles.insert(pos.clone());
            if score >= 1 {
                self.find_best_path_tiles(&(pos.clone() + dir.flip()), dir, score - 1, tiles);
            }
            if score >= 1000 {
                self.find_best_path_tiles(pos, dir.cw(), score - 1000, tiles);
                self.find_best_path_tiles(pos, dir.ccw(), score - 1000, tiles);
            }
        }
    }

    fn tiles_on_best_path(&mut self) -> usize {
        let best = self.min_score_to_end();
        let mut visited: HashSet<Pos> = HashSet::new();
        for &dir in DIRECTIONS.iter() {
            self.find_best_path_tiles(&self.maze.end, dir, best, &mut visited);
        }
        visited.len()
    }
}

fn part1(input: Lines) -> String {
    let maze = Maze::parse(input);
    Solver::new(&maze).min_score_to_end().to_string()
}

fn part2(input: Lines) -> String {
    let maze = Maze::parse(input);
    Solver::new(&maze).tiles_on_best_path().to_string()
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
    fn parse() {
        let maze = Maze::parse(include_str!("example.txt").lines());
        assert!(!maze.is_empty(&Pos{x: 0, y: 0}));
        assert!(maze.is_empty(&Pos{x: 1, y: 1}));
        assert_eq!(maze.size, Pos{x: 15, y: 15});
        assert_eq!(maze.start, Pos{x: 1, y: 13});
        assert_eq!(maze.end, Pos{x: 13, y: 1});
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "7036");
        verify!(part2, input, "45");
    }

    #[test]
    fn example2() {
        let input = include_str!("example2.txt");
        verify!(part1, input, "11048");
        verify!(part2, input, "64");
    }
}
