use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

type Page = u8;
const NUM_PAGES: usize = 100;

#[derive(Clone, Copy, Debug, PartialEq)]
struct PageSet(u128);

impl PageSet {
    fn empty() -> Self {
        Self(0)
    }

    fn from_iter<'a, I>(pages: I) -> Self
    where
        I: Iterator<Item = &'a Page>,
    {
        Self(pages
            .map(|p| 1u128 << p)
            .reduce(|a, b| a | b)
            .unwrap_or(0)
        )
    }

    fn page_bit(page: Page) -> u128 {
        1 << page
    }

    fn add(&mut self, page: Page) {
        self.0 |= Self::page_bit(page)
    }

    fn remove(&mut self, page: Page) {
        self.0 &= !Self::page_bit(page)
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn contains_all(&self, other: &PageSet) -> bool {
        (self.0 & other.0) == other.0
    }

    fn intersect(&self, other: &PageSet) -> Self {
        Self(self.0 & other.0)
    }
}

struct Rules {
    predecessors: [PageSet; NUM_PAGES]
}

impl Rules {
    fn parse_rule(line: &str) -> Option<(u8, u8)> {
        line.split('|')
            .flat_map(|n| n.parse::<Page>().ok())
            .next_tuple()
    }

    fn parse(input: &mut Lines) -> Self {
        let mut predecessors = [PageSet::empty(); NUM_PAGES];
        for (before, after) in input.map_while(Self::parse_rule) {
            predecessors[after as usize].add(before);
        }
        Rules{ predecessors }
    }

    fn is_update_valid(&self, update: &Update) -> bool {
        let all_updated_pages = update.as_set();
        let mut seen = PageSet::empty();
        for &page in update.0.iter() {
            if !seen.contains_all(&self.predecessors[page as usize].intersect(&all_updated_pages)) {
                return false
            }
            seen.add(page);
        }
        true
    }

    fn reorder_update(&self, update: &Update) -> Update {
        let mut remaining = update.0.clone();
        let mut remaining_set = update.as_set();
        let mut ordered = Vec::new();
        while let Some((next_index, &next)) = remaining.iter().find_position(|&&p| self.predecessors[p as usize].intersect(&remaining_set).is_empty()) {
            remaining.swap_remove(next_index);
            remaining_set.remove(next);
            ordered.push(next);
        }
        Update(ordered)
    }
}

#[derive(Debug, PartialEq)]
struct Update(Vec<Page>);

impl Update {
    fn parse(line: &str) -> Self {
        Self(line.split(',').map(|n| n.parse::<Page>().unwrap()).collect_vec())
    }

    fn as_set(&self) -> PageSet {
        PageSet::from_iter(self.0.iter())
    }

    fn middle_page(&self) -> Page {
        self.0[self.0.len() / 2]
    }
}

fn parse(mut input: Lines) -> (Rules, Vec<Update>) {
    let rules = Rules::parse(&mut input);
    let updates = input.map(Update::parse).collect_vec();
    (rules, updates)
}

fn part1(input: Lines) -> String {
    let (rules, updates) = parse(input);
    updates.into_iter()
        .filter(|update| rules.is_update_valid(update))
        .map(|update| update.middle_page() as u64)
        .sum::<u64>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let (rules, updates) = parse(input);
    updates.into_iter()
        .filter(|update| !rules.is_update_valid(update))
        .map(|update| rules.reorder_update(&update).middle_page() as u64)
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
    use crate::parse;
    use rstest::rstest;

    use super::*;
    use advent_of_code::verify;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "143");
        verify!(part2, input, "123");
    }

    #[test]
    fn test_parse() {
        let (rules, updates) = parse("1|2\n3|4\n\n1,2,3".lines());
        assert_eq!(rules.predecessors[2], PageSet::from_iter([1].iter()));
        assert_eq!(rules.predecessors[4], PageSet::from_iter([3].iter()));
        assert_eq!(updates, vec![Update(vec![1,2,3])]);
    }

    #[rstest]
    #[case(&[1, 2], &[1, 2], true)]
    #[case(&[1, 2], &[1], true)]
    #[case(&[1, 2], &[2], true)]
    #[case(&[1, 2], &[3], false)]
    #[case(&[1, 2], &[2, 3], false)]
    #[case(&[1, 2], &[1, 2, 3], false)]
    fn test_page_set_contains_all(#[case] a: &[Page], #[case] b: &[Page], #[case] expected: bool) {
        assert_eq!(PageSet::from_iter(a.iter()).contains_all(&PageSet::from_iter(b.iter())), expected);
    }

    #[rstest]
    #[case(&[1, 2], &[1, 2], &[1, 2])]
    #[case(&[1, 2], &[1], &[1])]
    #[case(&[1, 2], &[2], &[2])]
    #[case(&[1, 2], &[3], &[])]
    #[case(&[1, 2], &[2, 3], &[2])]
    #[case(&[1, 2], &[1, 2, 3], &[1, 2])]
    fn test_page_set_intersect(#[case] a: &[Page], #[case] b: &[Page], #[case] expected: &[Page]) {
        assert_eq!(
            PageSet::from_iter(a.iter()).intersect(&PageSet::from_iter(b.iter())),
            PageSet::from_iter(expected.iter())
        );
    }

    #[test]
    fn test_is_update_valid() {
        let (rules, updates) = parse(include_str!("example.txt").lines());
        /* Because the first update does not include some page numbers, the ordering
         * rules involving those missing page numbers are ignored. */
        assert_eq!(updates[0], Update(vec![75,47,61,53,29]));
        assert!(rules.is_update_valid(&updates[0]));

        /* The second and third updates are also in the correct order according to the
         * rules. Like the first update, they also do not include every page number,
         * and so only some of the ordering rules apply - within each update, the
         * ordering rules that involve missing page numbers are not used. */
        assert!(rules.is_update_valid(&updates[1]));
        assert!(rules.is_update_valid(&updates[2]));

        /* The fourth update, 75,97,47,61,53, is not in the correct order: it would
         * print 75 before 97, which violates the rule 97|75. */
        assert!(!rules.is_update_valid(&updates[3]));

        /* The fifth update, 61,13,29, is also not in the correct order, since it
         * breaks the rule 29|13. */
        assert!(!rules.is_update_valid(&updates[4]));

        /* The last update, 97,13,75,29,47, is not in the correct order due to
         * breaking several rules. */
        assert!(!rules.is_update_valid(&updates[5]));
    }

    #[test]
    fn test_reorder() {
        let (rules, _) = parse(include_str!("example.txt").lines());
        assert_eq!(rules.reorder_update(&Update(vec![75,97,47,61,53])), Update(vec![97,75,47,61,53]));
        assert_eq!(rules.reorder_update(&Update(vec![61,13,29])), Update(vec![61,29,13]));
        assert_eq!(rules.reorder_update(&Update(vec![97,13,75,29,47])), Update(vec![97,75,47,29,13]));
    }
}
