use advent_of_code::{create_runner, named, Named, Runner};
use glam::U8Vec2;
use itertools::{repeat_n, Itertools};
use std::{collections::{HashMap, VecDeque}, str::Lines};

type Pos = U8Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellKind {
    Track,
    Wall,
    Border,
}

struct Racetrack {
    cells: Vec<Vec<CellKind>>,
    size: Pos,
    start: Pos,
    end: Pos,
}

impl Racetrack {
    fn parse(input: Lines) -> Self {
        let mut start = None;
        let mut end = None;
        let mut cells = input
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        match c {
                            'S' => {
                                start = Some(Pos{x: x as u8, y: y as u8});
                                CellKind::Track
                            },
                            'E' => {
                                end = Some(Pos{x: x as u8, y: y as u8});
                                CellKind::Track
                            },
                            '.' => CellKind::Track,
                            '#' => CellKind::Wall,
                            u => panic!("unexpected char {u}")                            
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        let size = Pos{x: cells[0].len() as u8, y: cells.len() as u8};

        // overwrite all border cells with a different kind to make it easier
        // to know not to visit them without checking coordinates
        cells.first_mut().unwrap().fill(CellKind::Border);
        cells.last_mut().unwrap().fill(CellKind::Border);
        cells.iter_mut().for_each(|row| {
            *row.first_mut().unwrap() = CellKind::Border;
            *row.last_mut().unwrap() = CellKind::Border;
        });

        Self{cells, size, start: start.expect("start"), end: end.expect("end")}
    }
    
    fn get_cell(&self, pos: Pos) -> CellKind {
        self.cells[pos.y as usize][pos.x as usize]
    }

    fn neighbors(&self, pos: Pos) -> [(Pos, CellKind); 4] {
        [
            Pos{x: pos.x + 1, y: pos.y},
            Pos{x: pos.x, y: pos.y - 1},
            Pos{x: pos.x - 1, y: pos.y},
            Pos{x: pos.x, y: pos.y + 1},
        ].map(|neighbor| (neighbor, self.get_cell(neighbor)))
    }
 }

struct TimeSolver<'a> {
    racetrack: &'a Racetrack,
    times: Vec<Vec<Option<usize>>>,
}

impl<'a> TimeSolver<'a> {
    fn new(racetrack: &'a Racetrack) -> Self {
        let times = repeat_n(
            repeat_n(None, racetrack.size.x as usize)
                .collect_vec(),
            racetrack.size.y as usize
        )
        .collect_vec();
        Self{racetrack, times}
    }

    fn solve(racetrack: &'a Racetrack, start: Pos) -> Self {
        let mut solver = Self::new(racetrack);
        solver.find_shortest_times(start);
        solver
    }

    fn get_time(&self, pos: Pos) -> Option<usize> {
        self.times[pos.y as usize][pos.x as usize]
    }

    fn replace_if_better(&mut self, pos: Pos, time: usize) -> bool {
        let existing = &mut self.times[pos.y as usize][pos.x as usize];
        if existing.as_ref().is_none_or(|old| time < *old) {
            *existing = Some(time);
            true
        } else {
            false
        }
    }

    fn find_shortest_times(&mut self, start: Pos) {
        let mut traverse = VecDeque::new();
        traverse.push_back((start, 0));
        while let Some((pos, time)) = traverse.pop_front() {
            if self.replace_if_better(pos, time) {
                for (npos, kind) in self.racetrack.neighbors(pos) {
                    if kind == CellKind::Track {
                        traverse.push_back((npos, time + 1));
                    }
                }
            }
        }
    }
}

fn get_cheat_histogram(racetrack: Racetrack) -> HashMap<usize, usize> {
    let mut histogram = HashMap::new();
    let from_start = TimeSolver::solve(&racetrack, racetrack.start);
    let from_end = TimeSolver::solve(&racetrack, racetrack.end);
    let best_time_without_cheat = from_end.get_time(racetrack.start)
        .expect("time from start to end");
    for y in 0..racetrack.size.y {
        for x in 0..racetrack.size.x {
            let start_pos = Pos{x, y};
            if let Some(start_time) = from_start.get_time(start_pos) {
                for (cheat_start_pos, kind) in racetrack.neighbors(start_pos) {
                    if kind == CellKind::Wall {
                        for (cheat_end_pos, _kind) in racetrack.neighbors(cheat_start_pos) {
                            // a path to the end without cheating implies CellKind::Track
                            if let Some(end_time) = from_end.get_time(cheat_end_pos) {    
                                let time_with_cheat = 2 + start_time + end_time;
                                let saved = best_time_without_cheat.saturating_sub(time_with_cheat);
                                if saved > 0 {
                                    *histogram.entry(saved).or_default() += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    histogram
}

fn part1(input: Lines) -> String {
    let racetrack = Racetrack::parse(input);
    get_cheat_histogram(racetrack)
        .into_iter()
        .filter_map(|(saved, count)| (saved >= 100).then_some(count))
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
    fn test_get_cheat_histogram() {
        let racetrack = Racetrack::parse(include_str!("example.txt").lines());
        assert_eq!(
            get_cheat_histogram(racetrack),
            [
                (2, 14), // There are 14 cheats that save 2 picoseconds.
                (4, 14), // There are 14 cheats that save 4 picoseconds.
                (6, 2), // There are 2 cheats that save 6 picoseconds.
                (8, 4), // There are 4 cheats that save 8 picoseconds.
                (10, 2), // There are 2 cheats that save 10 picoseconds.
                (12, 3), // There are 3 cheats that save 12 picoseconds.
                (20, 1), // There is one cheat that saves 20 picoseconds.
                (36, 1), // There is one cheat that saves 36 picoseconds.
                (38, 1), // There is one cheat that saves 38 picoseconds.
                (40, 1), // There is one cheat that saves 40 picoseconds.
                (64, 1), // There is one cheat that saves 64 picoseconds.
            ].into_iter().collect()
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        // One of the rare puzzles where this test is pointless.
        // The example is small enough that no cheat can save 100 picoseconds.
        verify!(part1, input, "0");
        verify!(part2, input, "0");
    }
}
