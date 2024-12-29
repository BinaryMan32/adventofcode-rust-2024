use advent_of_code::{create_runner, named, Named, Runner};
use glam::{DMat2, DVec2, U64Vec2};
use itertools::Itertools;
use lazy_regex::{lazy_regex, Captures, Lazy, Regex};
use std::str::Lines;

type Pos = U64Vec2;

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

    fn offset_prize(&self, offset: u64) -> Self {
        Self{prize: self.prize + offset, ..*self}
    }

    const COST: DVec2 = DVec2{x: 3.0, y: 1.0};

    /**
     * Determine the minimum number of tokens to spend to move from (0, 0)
     * to the prize location.
     *
     * This can be written as a system of equations where `x` is unknown:
     * A * x = b
     *
     * This expands to the following, where `pa` and `pb` are the number of
     * button presses for the `A` and `B` buttons:
     *
     *  /       \   /  \     /       \
     * | a.x b.x | | pa | = | prize.x |
     * | a.y b.y | | pb |   | prize.y |
     *  \       /   \  /     \       /
     *
     * Apply https://en.wikipedia.org/wiki/Cramer%27s_rule to solve.
     */
    #[allow(non_snake_case)] 
    fn min_cost(&self) -> Option<u64> {
        let A = DMat2::from_cols(self.a.as_dvec2(), self.b.as_dvec2());
        let b = self.prize.as_dvec2();
        let presses = DVec2::new(
            DMat2::from_cols(b, A.col(1)).determinant(),
            DMat2::from_cols(A.col(0), b).determinant()
        ) / A.determinant();
        (presses.trunc() == presses).then_some(presses.dot(Self::COST) as u64)
    }
}

fn part1(input: Lines) -> String {
    Machine::parse_all(input)
        .into_iter()
        .flat_map(|m| m.min_cost())
        .sum::<u64>()
        .to_string()
}

fn part2(input: Lines) -> String {
    Machine::parse_all(input)
        .into_iter()
        .map(|m| m.offset_prize(10000000000000))
        .flat_map(|m| m.min_cost())
        .sum::<u64>()
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
    fn test_part2_wins() {
        // No solution given for the example in part 2, but it does say:
        // Now, it is only possible to win a prize on the second and fourth claw machines.
        assert_eq!(
            Machine::parse_all(include_str!("example.txt").lines())
                .into_iter()
                .map(|m| m.offset_prize(10000000000000))
                .map(|m| m.min_cost().is_some())
                .collect_vec(),
            [false, true, false, true]
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "480");
        verify!(part2, input, "875318608908");
    }
}
