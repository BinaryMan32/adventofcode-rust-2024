use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use num::pow;
use std::str::Lines;

struct Number {
    value: u64,
    shift: u64,
}

impl Number {
    fn parse(text: &str) -> Option<Self> {
        let value = text.parse().ok()?;
        let shift = pow(10, text.len());
        Some(Self{value, shift})
    }
}

struct Equation {
    test_value: u64,
    numbers: Vec<Number>,
}

impl Equation {
    fn parse(line: &str) -> Option<Self> {
        let (test_value, numbers) = line.split_once(": ")?;
        let test_value = test_value.parse().ok()?;
        let numbers = numbers.split_ascii_whitespace().flat_map(Number::parse).collect_vec();
        Some(Self{test_value, numbers})
    }

    fn can_solve_add_mul(&self) -> bool {
        self.can_solve_add_mul_aux(&self.numbers[1..], self.numbers[0].value)
    }

    fn can_solve_add_mul_aux(&self, remaining: &[Number], result: u64) -> bool {
        if remaining.is_empty() {
            result == self.test_value
        } else if result > self.test_value {
            false
        } else {
            let first = remaining[0].value;
            let remaining = &remaining[1..];
            self.can_solve_add_mul_aux(remaining, result + first) || self.can_solve_add_mul_aux(remaining, result * first)
        }
    }

    fn can_solve_add_mul_cat(&self) -> bool {
        self.can_solve_add_mul_cat_aux(&self.numbers[1..], self.numbers[0].value)
    }

    fn can_solve_add_mul_cat_aux(&self, remaining: &[Number], result: u64) -> bool {
        if remaining.is_empty() {
            result == self.test_value
        } else if result > self.test_value {
            false
        } else {
            let first = &remaining[0];
            let remaining = &remaining[1..];
            self.can_solve_add_mul_cat_aux(remaining, result + first.value) ||
                self.can_solve_add_mul_cat_aux(remaining, result * first.value) ||
                self.can_solve_add_mul_cat_aux(remaining, result * first.shift + first.value)
        }
    }
}

fn part1(input: Lines) -> String {
    input
        .flat_map(Equation::parse)
        .filter(Equation::can_solve_add_mul)
        .map(|eq| eq.test_value)
        .sum::<u64>()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .flat_map(Equation::parse)
        .filter(Equation::can_solve_add_mul_cat)
        .map(|eq| eq.test_value)
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
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "3749");
        verify!(part2, input, "11387");
    }
}
