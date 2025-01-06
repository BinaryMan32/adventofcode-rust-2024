use advent_of_code::{create_runner, named, Named, Runner};
use itertools::{FoldWhile, Itertools};
use std::{cmp::Ordering, collections::{hash_map::Entry, HashMap, HashSet, VecDeque}, fmt::Display, fs::File, io::Write, str::{FromStr, Lines}};

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
    
    fn color(&self) -> &str {
        match self {
            Operation::And => "cyan",
            Operation::Or => "magenta",
            Operation::Xor => "yellow",
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

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::And => f.write_str("AND"),
            Operation::Or => f.write_str("OR"),
            Operation::Xor => f.write_str("XOR"),
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
    initial_values: HashMap<String, bool>,
    cached_values: HashMap<String, bool>,
    expressions: HashMap<String, Expression>,
    output_mapping: HashMap<String, String>,
}

impl Input {
    fn parse_value(line: &str) -> (String, bool) {
        let (a, b) = line.split_once(": ").unwrap();
        (a.to_owned(), b.parse::<u8>().unwrap() != 0)
    }

    fn parse(mut input: Lines) -> Self {
        let initial_values = input.by_ref()
            .take_while(|line| !line.is_empty())
            .map(Self::parse_value)
            .collect();
        let cached_values = HashMap::new();
        let expressions = input
            .map(Expression::parse)
            .collect();
        let output_mapping = HashMap::new();
        Self{initial_values, cached_values, expressions, output_mapping}
    }

    fn eval_expression(&mut self, expression: &Expression) -> Option<bool> {
        self.eval_wire(&expression.a)
            .zip(self.eval_wire(&expression.b))
            .map(|(a, b)| expression.op.apply(a, b))
        
    }

    fn eval_wire(&mut self, key: &str) -> Option<bool> {
        self.cached_values.get(key)
            .or_else(|| self.initial_values.get(key))
            .cloned()
            .or_else(|| {
                let mapped = self.output_mapping.get(key).cloned();
                let mapped_key = mapped.as_deref().unwrap_or(key);
                self.expressions.get(mapped_key).cloned().and_then(|expression| {
                    self.eval_expression(&expression)
                        .inspect(|&value| {
                            self.cached_values.insert(key.to_owned(), value);
                        })
                })
            })
    }

    fn connected_gates(&self, key: &str) -> HashSet<String> {
        let mut gates = HashSet::new();
        let mut traverse = VecDeque::new();
        traverse.push_back(key);
        while let Some(key) = traverse.pop_front() {
            if let Some(expression) = self.expressions.get(key) {
                if gates.insert(key.to_owned()) {
                    traverse.push_back(&expression.a);
                    traverse.push_back(&expression.b);
                }
            }
        }
        gates.remove(key);
        gates
    }

    fn swap_outputs(&mut self, swap: &[&str]) {
        self.cached_values.clear();
        self.output_mapping.clear();
        for (&a, &b) in swap.into_iter().tuples() {
            self.output_mapping.insert(a.to_owned(), b.to_owned());
            self.output_mapping.insert(b.to_owned(), a.to_owned());
        }
    }

    fn get_number(&mut self, prefix: &str) -> u64 {
        (0..63)
            .fold_while(0, |num, i| {
                match self.eval_wire(&format!("{prefix}{i:02}")) {
                    Some(true) => FoldWhile::Continue(num | (1<<i)),
                    Some(false) => FoldWhile::Continue(num),
                    None => FoldWhile::Done(num),
                }
            })
            .into_inner()
    }

    fn set_number(&mut self, prefix: &str, num: u64) {
        self.cached_values.clear();
        for i in 0..u64::BITS {
            match self.initial_values.entry(format!("{prefix}{i:02}")) {
                Entry::Occupied(mut entry) => *entry.get_mut() = num & (1 << i) != 0,
                Entry::Vacant(_) => assert!(num < (1 << i)),
            }
        }
    }

    fn write_dot(&mut self, bad_output_bits: u64, mut writer: &mut impl Write) -> std::io::Result<()> {
        writeln!(&mut writer, "digraph {{")?;
        for (input, value) in self.initial_values.iter() {
            writeln!(&mut writer, "{input} [label = \"{input}\\n{num}\", style = \"filled\", fillcolor = \"lightblue\"];", num=*value as u8)?;
        }
        let expressions = self.expressions.keys().cloned().collect_vec();
        let mut sinks: Vec<String> = Vec::new();
        for output in expressions.into_iter() {
            let expression = self.expressions.get(self.output_mapping.get(&output).unwrap_or(&output)).unwrap().clone();
            let value = self.eval_wire(&output).unwrap();
            let label = format!("{op}\\n{output}\\n{num}", op=expression.op, num = value as u8);
            let mut attributes: Vec<(&str, &str)> = vec![
                ("label", &label),
            ];
            let mut edge_color = "black";
            if output.starts_with('z') {
                let bit = output[1..].parse::<usize>().unwrap();
                attributes.push(("style", "filled"));
                if (bad_output_bits & (1 << bit)) == 0 {
                    attributes.push(("fillcolor", "green"));
                } else {
                    attributes.push(("fillcolor", "red"));
                    edge_color = "red";
                }
                sinks.push(output.clone());
            } else {
                attributes.push(("style", "filled"));
                attributes.push(("fillcolor", expression.op.color()));
            }
            writeln!(&mut writer, "{output} [{attrs}];",
                attrs=attributes.into_iter().map(|(k, v)| format!("{k} = \"{v}\"")).join(", ")
            )?;
            writeln!(&mut writer, "{{{a}, {b}}} -> {output} [color=\"{edge_color}\"];", a=expression.a, b=expression.b)?;
        }
        // add some edges to try to make bits adjacent
        sinks.sort();
        writeln!(&mut writer, "{{ rank = same; {nodes} [style=invis] }}", nodes=sinks.into_iter().rev().join(" -> "))?;
        writeln!(&mut writer, "}}")?;
        Ok(())
    }
}

fn part1(input: Lines) -> String {
    let mut input = Input::parse(input);
    input.get_number("z").to_string()
}

fn bit_wires(num: u64, prefix: &str) -> Vec<String> {
    (0..u64::BITS)
        .filter(|b| num & (1 << b) != 0)
        .map(|b| format!("{prefix}{b:02}"))
        .collect_vec()
}

fn test_circuit(input: &mut Input, x: u64, y: u64, expected: u64) -> u64 {
    input.set_number("x", x);
    input.set_number("y", y);
    let z = input.get_number("z");
    z ^ expected
}

fn test_circuit_commutative(input: &mut Input, x: u64, y: u64, expected: u64) -> u64 {
    test_circuit(input, x, y, expected) | test_circuit(input, y, x, expected)
}

fn test_sum(input: &mut Input) -> u64 {
    const INPUT_BITS: usize = 45;
    const MAX_INPUT: u64 = (1 << INPUT_BITS) - 1;
    test_circuit(input, 0, 0, 0) |
    test_circuit(input, MAX_INPUT, MAX_INPUT, MAX_INPUT + MAX_INPUT) |
    (0..INPUT_BITS)
        .map(|i| {
            let bit = 1 << i;
            test_circuit(input, bit, bit, bit + bit) |
            test_circuit_commutative(input, 0, bit, bit)
        })
        .reduce(|a, b| a | b)
        .unwrap()
}

#[cfg(test)]
fn test_and(input: &mut Input) -> u64 {
    use rand::{distributions::Uniform, Rng};
    const INPUT_BITS: usize = 6;
    const MAX_INPUT: u64 = (1 << INPUT_BITS) - 1;
    let range = Uniform::new_inclusive(0, MAX_INPUT);
    rand::thread_rng().sample_iter(range)
        .take(INPUT_BITS * 4)
        .tuples()
        .map(|(x, y)| test_circuit_commutative(input, x, y, x & y))
        .reduce(|a, b| a | b)
        .unwrap()
}

fn maybe_replace_wires_with_upstream(input: &mut Input, bad_wires: &[String], swapped_pairs: usize, tester: &impl Fn(&mut Input) -> u64) -> Option<Vec<String>> {
    println!("upstream[{len}]={joined}", len=bad_wires.len(), joined=format_wires(bad_wires));
    bad_wires.iter()
    .map(|key| (key, input.connected_gates(key)))
    .fold(HashMap::<String, Vec<String>>::new(), |mut counts, (wire, connected)| {
        for c in connected {
            counts.entry(c.to_owned()).or_default().push(wire.to_owned());
        }
        counts
    })
    .into_iter()
    .map(|(upstream, wires)| {
        bad_wires.iter()
                .filter(|w| !wires.contains(w))
                .map(|w| w.to_owned())
                .chain(std::iter::once(upstream.clone()))
                .to_owned()
                .collect_vec()
    })
    .filter_map(|simplified| find_swapped_wires_internal(input, &simplified, swapped_pairs, tester))
    .next()
}

/**
 * Checking all permutations of 8 wires = 8! = 40320.
 * Number of pairings is 8! / (2^4 * 4!) = 104, but writing an algorithm to return only these pairings is harder.
 * 
 */
fn check_wire_swaps(input: &mut Input, bad_wires: &[String], swapped_pairs: usize, tester: &impl Fn(&mut Input) -> u64) -> Option<Vec<String>> {
    println!("   swaps[{len}]={joined}", len=bad_wires.len(), joined=format_wires(bad_wires));
    bad_wires.into_iter()
        .combinations(swapped_pairs)
        .flat_map(|firsts| {
            let others = bad_wires.iter()
                .filter(|w| !firsts.contains(w))
                .collect_vec();
            others.into_iter()
                .permutations(swapped_pairs)
                .map(move |others| firsts.iter().cloned().interleave_shortest(others.into_iter()).map(|s| s as &str).collect_vec())
        })
        .find(|swap| {
            input.swap_outputs(swap);
            tester(input) == 0
        })
        .map(|result| result.into_iter().map(|s| s.to_owned()).collect_vec())
}

fn find_swapped_wires_internal(input: &mut Input, bad_wires: &[String], swapped_pairs: usize, tester: &impl Fn(&mut Input) -> u64) -> Option<Vec<String>> {
    match bad_wires.len().cmp(&(swapped_pairs * 2)) {
        Ordering::Less => None,
        Ordering::Equal => check_wire_swaps(input, bad_wires, swapped_pairs, tester)
            .or_else(|| maybe_replace_wires_with_upstream(input, bad_wires, swapped_pairs, tester)),
        Ordering::Greater => maybe_replace_wires_with_upstream(input, bad_wires, swapped_pairs, tester),
    }
}

fn format_wires(wires: &[String]) -> String {
    wires.into_iter().sorted().join(",")
}

fn find_swapped_wires(input: Lines, swapped_pairs: usize, tester: &impl Fn(&mut Input) -> u64) -> Option<String> {
    let mut input = Input::parse(input);
    let bad_bits = tester(&mut input);
    let bad_wires = bit_wires(bad_bits, "z");
    find_swapped_wires_internal(&mut input, &bad_wires, swapped_pairs, tester)
        .map(|wires| format_wires(&wires))
}

fn part2(input: Lines) -> String {
    let mut input = Input::parse(input);
    let swap_wires = vec!["gjc", "qjj", "z17", "wmp", "z26", "gvm", "z39", "qsb"];
    input.swap_outputs(&swap_wires);
    let bad_output_bits = test_sum(&mut input);
    let mut out = File::create("gates.dot").unwrap();
    input.write_dot(bad_output_bits, &mut out).unwrap();
    format_wires(&swap_wires.into_iter().map(|s| s.to_owned()).collect_vec())
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
    fn part1_example1() {
        let input = include_str!("example.txt");
        verify!(part1, input, "4");
    }

    #[test]
    fn part1_example2() {
        let input = include_str!("example2.txt");
        verify!(part1, input, "2024"); 
    }

    #[test]
    fn part2_example3() {
        let input = include_str!("example3.txt").lines();
        assert_eq!(find_swapped_wires(input, 2, &test_and), Some("z00,z01,z02,z05".to_owned())); 
    }
}
