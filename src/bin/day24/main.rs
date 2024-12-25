use advent_of_code::{create_runner, named, Named, Runner};
use itertools::{FoldWhile, Itertools};
use std::{collections::HashMap, str::{FromStr, Lines}};

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
    
}

#[derive(Debug)]
struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(Self::Err{})
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Expression {
    a: String,
    b: String,
    op: Operation,
}

impl Expression {
    fn parse(line: &str) -> (String, Expression) {
        let (expression, key) = line.split_once(" -> ").unwrap();
        let (a, op, b) = expression.split_ascii_whitespace().collect_tuple().unwrap();
        let a = a.to_owned();
        let b = b.to_owned();
        let op = op.parse::<Operation>().unwrap();
        (key.to_owned(), Self{a, b, op})
    }
}

struct Input {
    values: HashMap<String, bool>,
    expressions: HashMap<String, Expression>,
}

impl Input {
    fn parse_value(line: &str) -> (String, bool) {
        let (a, b) = line.split_once(": ").unwrap();
        (a.to_owned(), b.parse::<u8>().unwrap() != 0)
    }

    fn parse(mut input: Lines) -> Self {
        let values = input.by_ref()
            .take_while(|line| !line.is_empty())
            .map(Self::parse_value)
            .collect();
        let expressions = input
            .map(Expression::parse)
            .collect();
        Self{values, expressions}
    }

    fn eval_expression(&mut self, expression: &Expression) -> bool {
        let a = self.eval_wire(&expression.a).unwrap();
        let b = self.eval_wire(&expression.b).unwrap();
        expression.op.apply(a, b)
    }

    fn eval_wire(&mut self, key: &str) -> Option<bool> {
        self.values.get(key)
            .cloned()
            .or_else(|| {
                self.expressions.get(key).cloned().map(|expression| {
                    let value = self.eval_expression(&expression);
                    self.values.insert(key.to_owned(), value);
                    value
                })
            })
    }

    fn get_number(&mut self) -> u64 {
        (0..63)
            .fold_while(0, |num, i| {
                match self.eval_wire(&format!("z{i:02}")) {
                    Some(true) => FoldWhile::Continue(num | (1<<i)),
                    Some(false) => FoldWhile::Continue(num),
                    None => FoldWhile::Done(num),
                }
            })
            .into_inner()
    }
}

fn part1(input: Lines) -> String {
    let mut input = Input::parse(input);
    input.get_number().to_string()
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
    fn parse_value() {
        assert_eq!(Input::parse_value("x00: 1"), ("x00".to_owned(), true));
    }

    #[test]
    fn parse_expression() {
        assert_eq!(
            Expression::parse("x00 AND y00 -> z00"),
            ("z00".to_owned(), Expression{a: "x00".to_owned(), b: "y00".to_owned(), op: Operation::And})
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "4");
        verify!(part2, input, "0");
    }

    #[test]
    fn example2() {
        let input = include_str!("example2.txt");
        verify!(part1, input, "2024");
        verify!(part2, input, "0");
    }
}
