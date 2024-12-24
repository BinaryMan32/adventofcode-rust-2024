use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{collections::{HashMap, HashSet}, str::Lines};

type Computer = String;

fn parse_connection(line: &str) -> (Computer, Computer) {
    line.splitn(2, '-')
        .map(|s| s.to_owned())
        .collect_tuple()
        .unwrap()
}

fn parse_connections(input: Lines) -> HashMap<Computer, HashSet<Computer>> {
    let mut sets: HashMap<Computer, HashSet<Computer>> = Default::default();
    for (a, b) in input.map(parse_connection) {
        sets.entry(a.clone()).or_default().insert(b.clone());
        sets.entry(b.clone()).or_default().insert(a.clone());
    }
    sets
}

fn find_interconnected(connections: HashMap<Computer, HashSet<Computer>>) -> Vec<[Computer; 3]> {
    connections.iter()
        .flat_map(|(a, a_conns)| {
            a_conns.iter()
                .filter(|b| *a < **b)
                .flat_map(|b| {
                    connections[b].intersection(a_conns)
                        .filter_map(|c| {
                            if *b < *c {
                                Some([a.to_owned(), b.to_owned(), c.to_owned()])
                            } else {
                                None
                            }
                        })
                })
        })
        .collect_vec()
}

fn part1(input: Lines) -> String {
    find_interconnected(parse_connections(input))
        .into_iter()
        .filter(|set| {
            set.iter().any(|computer| computer.starts_with('t'))
        })
        .count()
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
    fn test_find_interconnected() {
        let mut interconnected = find_interconnected(parse_connections(include_str!("example.txt").lines()));
        interconnected.sort();
        assert_eq!(
            interconnected,
            vec![
                ["aq", "cg", "yn"],
                ["aq", "vc", "wq"],
                ["co", "de", "ka"],
                ["co", "de", "ta"],
                ["co", "ka", "ta"],
                ["de", "ka", "ta"],
                ["kh", "qp", "ub"],
                ["qp", "td", "wh"],
                ["tb", "vc", "wq"],
                ["tc", "td", "wh"],
                ["td", "wh", "yn"],
                ["ub", "vc", "wq"],
            ]
        )
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "7");
        verify!(part2, input, "0");
    }
}
