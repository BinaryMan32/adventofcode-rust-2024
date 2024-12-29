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

    fn cells_within_distance(&self, center: Pos, max_distance: u8) -> Vec<Pos> {
        (center.y.saturating_sub(max_distance)..=(center.y + max_distance).min(self.size.y - 1))
            .flat_map(|y| {
                let x_distance = max_distance - center.y.abs_diff(y);
                (center.x.saturating_sub(x_distance)..=(center.x + x_distance).min(self.size.x - 1))
                    .map(move |x| Pos{x, y})
            })
            .collect_vec()
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

fn manhattan_distance(a: Pos, b: Pos) -> u8 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn get_cheat_histogram(racetrack: Racetrack, max_cheat: u8, threshold: usize) -> HashMap<usize, usize> {
    let mut histogram = HashMap::new();
    let from_start = TimeSolver::solve(&racetrack, racetrack.start);
    let from_end = TimeSolver::solve(&racetrack, racetrack.end);
    let best_time_without_cheat = from_end.get_time(racetrack.start)
        .expect("time from start to end");
    for y in 0..racetrack.size.y {
        for x in 0..racetrack.size.x {
            let start_pos = Pos{x, y};
            if let Some(start_time) = from_start.get_time(start_pos) {
                for cheat_end_pos in racetrack.cells_within_distance(start_pos, max_cheat) {
                    if let Some(end_time) = from_end.get_time(cheat_end_pos) {
                        let cheat_time = manhattan_distance(start_pos, cheat_end_pos) as usize;
                        let time_with_cheat = cheat_time + start_time + end_time;
                        let saved = best_time_without_cheat.saturating_sub(time_with_cheat);
                        if saved >= threshold && saved > 0 {
                            *histogram.entry(saved).or_default() += 1;
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
    get_cheat_histogram(racetrack, 2, 100)
        .into_values()
        .sum::<usize>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let racetrack = Racetrack::parse(input);
    get_cheat_histogram(racetrack, 20, 100)
        .into_values()
        .sum::<usize>()
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
    fn test_get_cheat_histogram_part1() {
        let racetrack = Racetrack::parse(include_str!("example.txt").lines());
        assert_eq!(
            get_cheat_histogram(racetrack, 2, 0),
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
    fn test_get_cheat_histogram_part2() {
        let racetrack = Racetrack::parse(include_str!("example.txt").lines());
        assert_eq!(
            get_cheat_histogram(racetrack, 20, 50),
            [
                (50, 32), // There are 32 cheats that save 50 picoseconds.
                (52, 31), // There are 31 cheats that save 52 picoseconds.
                (54, 29), // There are 29 cheats that save 54 picoseconds.
                (56, 39), // There are 39 cheats that save 56 picoseconds.
                (58, 25), // There are 25 cheats that save 58 picoseconds.
                (60, 23), // There are 23 cheats that save 60 picoseconds.
                (62, 20), // There are 20 cheats that save 62 picoseconds.
                (64, 19), // There are 19 cheats that save 64 picoseconds.
                (66, 12), // There are 12 cheats that save 66 picoseconds.
                (68, 14), // There are 14 cheats that save 68 picoseconds.
                (70, 12), // There are 12 cheats that save 70 picoseconds.
                (72, 22), // There are 22 cheats that save 72 picoseconds.
                (74, 4), // There are 4 cheats that save 74 picoseconds.
                (76, 3), // There are 3 cheats that save 76 picoseconds.
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
