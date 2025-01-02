use advent_of_code::{create_runner, named, Named, Runner};
use glam::I8Vec2;
use itertools::Itertools;
use phf::phf_map;
use std::{collections::HashMap, iter::{once, repeat_n, RepeatN}, str::Lines};

type Pos = I8Vec2;

struct KeyMap {
    key_positions: phf::Map<char, Pos>,
    invalid_position: Pos,
}

impl KeyMap {
    fn x_chars(delta: &Pos) -> RepeatN<char> {
        repeat_n(
            if delta.x > 0 { '>' } else { '<' },
            delta.x.unsigned_abs().into()
        )
    }
    fn y_chars(delta: &Pos) -> RepeatN<char> {
        repeat_n(
            if delta.y > 0 { 'v' } else { '^' },
            delta.y.unsigned_abs().into()
        )
    }
    fn valid_paths(&self, a: char, b: char) -> Vec<String> {
        let a_pos = self.key_positions.get(&a).unwrap();
        let b_pos = self.key_positions.get(&b).unwrap();
        let delta = b_pos - a_pos;
        match (delta.x.abs() > 0, delta.y.abs() > 0) {
            (false, false) => vec![once('A').collect()],
            (true, false) => vec![Self::x_chars(&delta).chain(once('A')).collect()],
            (false, true) => vec![Self::y_chars(&delta).chain(once('A')).collect()],
            (true, true) => {
                [
                    (self.invalid_position != Pos::new(b_pos.x, a_pos.y))
                        .then(|| Self::x_chars(&delta)
                            .chain(Self::y_chars(&delta))
                            .chain(once('A')).collect()
                        ),
                    (self.invalid_position != Pos::new(a_pos.x, b_pos.y))
                        .then(|| Self::y_chars(&delta)
                            .chain(Self::x_chars(&delta))
                            .chain(once('A')).collect()
                        ),
                ].into_iter().flatten().collect_vec()
            }
        }
    }
}

/**
 * Keys:
 * +---+---+---+
 * | 7 | 8 | 9 |
 * +---+---+---+
 * | 4 | 5 | 6 |
 * +---+---+---+
 * | 1 | 2 | 3 |
 * +---+---+---+
 *     | 0 | A |
 *     +---+---+
 */
static NUMERIC_KEY_MAP: KeyMap = KeyMap{
    key_positions: phf_map! {
        '7' => Pos{x: 0, y: 0},
        '8' => Pos{x: 1, y: 0},
        '9' => Pos{x: 2, y: 0},
        '4' => Pos{x: 0, y: 1},
        '5' => Pos{x: 1, y: 1},
        '6' => Pos{x: 2, y: 1},
        '1' => Pos{x: 0, y: 2},
        '2' => Pos{x: 1, y: 2},
        '3' => Pos{x: 2, y: 2},
        '0' => Pos{x: 1, y: 3},
        'A' => Pos{x: 2, y: 3},
    },
    invalid_position: Pos{x: 0, y: 3},
};

/**
 * Keys:
 *     +---+---+
 *     | ^ | A |
 * +---+---+---+
 * | < | v | > |
 * +---+---+---+
 */
static DIRECTIONAL_KEY_MAP: KeyMap = KeyMap{
    key_positions: phf_map! {
        '^' => Pos{x: 1, y: 0},
        'A' => Pos{x: 2, y: 0},
        '<' => Pos{x: 0, y: 1},
        'v' => Pos{x: 1, y: 1},
        '>' => Pos{x: 2, y: 1},
    },
    invalid_position: Pos{x: 0, y: 0},
};

struct KeyPadController<'a> {
    robot: Option<&'a mut KeyPadController<'a>>,
    keypad: &'static KeyMap,
    cache: HashMap<String, Vec<String>>,
}

impl<'a> KeyPadController<'a> {
    fn new(keypad: &'static KeyMap, robot: Option<&'a mut KeyPadController<'a>>) -> Self {
        Self {
            keypad,
            robot,
            cache: HashMap::new(),
        }
    }

    fn sequence_cached(&mut self, seq: &str) -> Vec<String> {
        match self.cache.get(seq) {
            None => {
                let result = self.sequence(seq);
                self.cache.insert(seq.to_owned(), result.clone());
                result
            },
            Some(result) => result.clone()
        }
    }

    fn parent_sequence(&mut self, seq: String) -> Vec<String> {
        match self.robot.as_mut() {
            None => vec![seq],
            Some(parent) => {
                let mut result = parent.sequence_cached(&seq);
                result.push(seq);
                result
            },
        }
    }

    fn best_path_from(&mut self, a: char, b: char) -> Vec<String> {
        self.keypad.valid_paths(a, b)
            .into_iter()
            .map(|path| self.parent_sequence(path))
            .min_by_key(|paths| paths[0].len())
            .unwrap()
    }

    fn sequence(&mut self, seq: &str) -> Vec<String> {
        once('A')
            .chain(seq.chars())
            .tuple_windows()
            .map(|(a, b)| self.best_path_from(a, b))
            .reduce(|a, b| {
                a.into_iter().zip(b.into_iter())
                    .map(|(a, b)| a + &b)
                    .collect_vec()
            })
            .unwrap()
    }
}


/*
 * The example `029A` contains one move from 2 to 9 which requires moving on
 * both axes. From the perspective of the numeric keypad, all permutations of
 * `>^^` are equally valid, but do they fare differently from the controlling
 * directional keypad?
 * 
 * >^^A - vA <^A A >A 
 * ^>^A - <A >vA <^A >A
 * ^^>A - <A A >vA >A
 * 
 * The middle ordering is obviously bad since it splits up pressing the same
 * button which could otherwise be handled with a single button press. The
 * others appear to be roughly equivalent, but we can't tell the relative
 * differences without expanding one more time. At this level a human is
 * pressing buttons directly so ordering is irrelevant.
 * 
 * vA <^A - [ <vA >^A ] [ v<<A >^A >A ] 
 * <A >vA - [ v<<A >>^A ] [ vA <A >^A ]
 * 
 * This seems to suggest that each keypad can be done independently as long
 * as all moves on one axis are done first.
 */
fn shortest_button_sequence(code: &str) -> Vec<String> {
    let mut stage1 = &mut KeyPadController::new(
        &DIRECTIONAL_KEY_MAP,
        None
    );
    let mut stage2 = KeyPadController::new(
        &DIRECTIONAL_KEY_MAP,
        Some(&mut stage1)
    );
    let mut stage3 = KeyPadController::new(
        &NUMERIC_KEY_MAP,
        Some(&mut stage2)
    );
    let mut seqs = stage3.sequence(code);
    seqs.push(code.to_owned());
    seqs
}

fn code_complexity_sequence(code: &str, sequence_len: usize) -> usize {
    let numeric_code = code.strip_suffix("A").unwrap().parse::<usize>().unwrap();
    numeric_code * sequence_len
}

fn code_complexity(code: &str) -> usize {
    code_complexity_sequence(code, shortest_button_sequence(code)[0].len())
}

fn part1(input: Lines) -> String {
    input.into_iter()
        .map(code_complexity)
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
    use std::collections::HashSet;

    use super::*;
    use advent_of_code::verify;
    use rstest::rstest;

    #[rstest]
    #[case(&NUMERIC_KEY_MAP, '1', '0', vec![">vA"])]
    #[case(&NUMERIC_KEY_MAP, '0', '1', vec!["^<A"])]
    #[case(&NUMERIC_KEY_MAP, '1', 'A', vec![">>vA"])]
    #[case(&NUMERIC_KEY_MAP, 'A', '1', vec!["^<<A"])]
    #[case(&NUMERIC_KEY_MAP, '4', 'A', vec![">>vvA"])]
    #[case(&NUMERIC_KEY_MAP, 'A', '4', vec!["^^<<A"])]
    #[case(&NUMERIC_KEY_MAP, '7', '6', vec![">>vA", "v>>A"])]
    #[case(&NUMERIC_KEY_MAP, '6', '7', vec!["<<^A", "^<<A"])]
    #[case(&DIRECTIONAL_KEY_MAP, '^', '<', vec!["v<A"])]
    #[case(&DIRECTIONAL_KEY_MAP, '<', '^', vec![">^A"])]
    #[case(&DIRECTIONAL_KEY_MAP, 'A', '<', vec!["v<<A"])]
    #[case(&DIRECTIONAL_KEY_MAP, '<', 'A', vec![">>^A"])]
    fn valid_path(#[case] key_map: &KeyMap, #[case] a: char, #[case] b: char, #[case] expected: Vec<&str>) {
        assert_eq!(
            key_map.valid_paths(a, b).into_iter().collect::<HashSet<String>>(),
            expected.into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>()
        );
    }

    #[test]
    fn test_sequence() {
        assert_eq!(
            shortest_button_sequence("029A").into_iter()
                .map(|s| s.len())
                .collect_vec(),
            [
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
                "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
                "<A^A>^^AvvvA",
                "029A",
            ].map(|s| s.len())
        );
    }

    #[rstest]
    #[case("029A", 68, 29)]
    #[case("980A", 60, 980)]
    #[case("179A", 68, 179)]
    #[case("456A", 64, 456)]
    #[case("379A", 68, 379)]
    fn test_code_complexity_sequence(#[case] code: &str, #[case] sequence_len: usize, #[case] numeric: usize) {
        assert_eq!(code_complexity_sequence(code, sequence_len), sequence_len * numeric)
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "126384");
        verify!(part2, input, "0");
    }
}
