use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{collections::{HashMap, HashSet, VecDeque}, iter::once, str::Lines};

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

fn all_connected(connections: &HashMap<Computer, HashSet<Computer>>, set: &HashSet<Computer>) -> HashSet<Computer> {
    set.iter()
        .map(|a| connections.get(a).cloned().unwrap_or_default())
        .reduce(|a, b| {
            a.intersection(&b).cloned().collect()
        })
        .unwrap_or_default()
}

fn are_connected(connections: &HashMap<Computer, HashSet<Computer>>, a_set: &HashSet<Computer>, b_set: &HashSet<Computer>) -> bool {
    a_set.is_subset(&all_connected(connections, b_set)) &&
        b_set.is_subset(&all_connected(connections, a_set))
}

fn largest_connected_component(connections: HashMap<Computer, HashSet<Computer>>) -> Option<HashSet<Computer>> {
    let mut largest: Option<HashSet<Computer>> = None;
    let mut processing: VecDeque<HashSet<Computer>> = connections.keys()
        .sorted()
        .map(|c| HashSet::from_iter(once(c.to_owned())))
        .collect();
    while let Some(a) = processing.pop_front() {
        if let Some(b_pos) = processing.iter().position(|b| are_connected(&connections, &a, &b)) {
            let b= processing.remove(b_pos).unwrap();
            processing.push_back(a.union(&b)
                .cloned()
                .collect());
        } else {
            largest = [largest, Some(a)].into_iter().flatten().max_by_key(|c| c.len())
        }
    }
    largest
}

fn format_largest_component(component: HashSet<Computer>) -> String {
    component.into_iter()
        .sorted()
        .join(",")
}

fn part2(input: Lines) -> String {
    largest_connected_component(parse_connections(input))
        .map(format_largest_component)
        .unwrap_or_default()
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
        verify!(part2, input, "co,de,ka,ta");
    }
}
