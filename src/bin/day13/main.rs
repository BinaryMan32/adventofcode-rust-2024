use advent_of_code::{create_runner, named, Named, Runner};
use glam::IVec2;
use itertools::Itertools;
use lazy_regex::{lazy_regex, Captures, Lazy, Regex};
use num::integer::div_rem;
use std::str::Lines;

type Pos = IVec2;

#[derive(Debug, PartialEq)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

pub static BUTTON_REGEX: Lazy<Regex> = lazy_regex!(r#"Button [AB]: X\+([0-9]+), Y\+([0-9]+)"#);
pub static PRIZE_REGEX: Lazy<Regex> = lazy_regex!(r#"Prize: X=([0-9]+), Y=([0-9]+)"#);

impl Machine {

    fn pos_from_captures(captures: Captures) -> Option<Pos> {
        let (x, y) = captures.iter()
            .flat_map(|n| n.unwrap().as_str().parse().ok())
            .collect_tuple()
            .expect("2 matching groups");
        Some(Pos{x, y})
    }

    fn parse(lines: (&str, &str, &str)) -> Option<Self> {
        let (a, b, prize) = lines;
        let a = Self::pos_from_captures(BUTTON_REGEX.captures(a)?)?;
        let b = Self::pos_from_captures(BUTTON_REGEX.captures(b)?)?;
        let prize = Self::pos_from_captures(PRIZE_REGEX.captures(prize)?)?;
        Some(Self{a, b, prize})
    }
    
    fn parse_all(input: Lines) -> Vec<Self> {
        input
            .chunk_by(|line| line.is_empty())
            .into_iter()
            .filter(|(empty, _chunk)| !empty)
            .map(|(_empty, chunk)| {
                Self::parse(chunk.collect_tuple().expect("3 elements")).expect("successfully parsed")
            })
            .collect_vec()
    }

    /**
     * Determine the minimum number of tokens to spend to move from (0, 0)
     * to the prize location.
     * tokens = 3 * A + B
     * equations for button presses to reach prize:
     * px = ax * A + bx * B
     * py = ay * A + by * B
     * Solve for B:
     * (py - ay * A) / by = B
     * Substituting B
     * px = ax * A + bx * (py - ay * A) / by
     * Multiply by
     * px * by = ax * by * A + bx * (py - ay * A)
     * Distribute bx
     * px * by = ax * by * A + bx * py - bx * ay * A
     * Collect
     * px * by - bx * py = A * (bx * py - bx * ay)
     * In the end, brute force is just easier
     */
    fn min_cost(&self) -> Option<i32> {
        (0..=100).flat_map(|a| {
            let remainder = self.prize - a * self.a;
            let (bx, bxr) = div_rem(remainder.x, self.b.x);
            let (by, byr) = div_rem(remainder.y, self.b.y);
            (bxr == 0 && byr == 0 && bx == by).then_some(3 * a + bx)
        }).min()
    }
}

fn part1(input: Lines) -> String {
    Machine::parse_all(input)
        .into_iter()
        .flat_map(|m| m.min_cost())
        .sum::<i32>()
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
    fn parse() {
        assert_eq!(Machine::parse_all(include_str!("example.txt").lines()),
            vec![
                Machine{
                    a: Pos::new(94, 34),
                    b: Pos::new(22, 67),
                    prize: Pos::new(8400, 5400)
                },
                Machine{
                    a: Pos::new(26, 66),
                    b: Pos::new(67, 21),
                    prize: Pos::new(12748, 12176)
                },
                Machine{
                    a: Pos::new(17, 86),
                    b: Pos::new(84, 37),
                    prize: Pos::new(7870, 6450)
                },
                Machine{
                    a: Pos::new(69, 23),
                    b: Pos::new(27, 71),
                    prize: Pos::new(18641, 10279)
                },
            ]
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "480");
        verify!(part2, input, "0");
    }
}
