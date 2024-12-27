use advent_of_code::{create_runner, named, Named, Runner};
use glam::I8Vec2;
use itertools::Itertools;
use phf::phf_map;
use std::{iter::{once, repeat_n, RepeatN}, str::Lines};

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
    fn valid_path(&self, a: char, b: char) -> Vec<char> {
        let a_pos = self.key_positions.get(&a).unwrap();
        let b_pos = self.key_positions.get(&b).unwrap();
        let delta = b_pos - a_pos;
        match (delta.x.abs() > 0, delta.y.abs() > 0) {
            (false, false) => once('A').collect(),
            (true, false) => Self::x_chars(&delta).chain(once('A')).collect(),
            (false, true) => Self::y_chars(&delta).chain(once('A')).collect(),
            (true, true) => if a_pos.x == self.invalid_position.x {
                Self::x_chars(&delta)
                    .chain(Self::y_chars(&delta))
                    .chain(once('A')).collect()
            } else {
                // TODO this has an arbitrary preference for one of the two paths,
                // and both need to be checked
                Self::y_chars(&delta)
                    .chain(Self::x_chars(&delta))
                    .chain(once('A')).collect()
            }
        }
    }
    fn sequence(&self, code: &str) -> String {
        once('A')
            .chain(code.chars())
            .tuple_windows()
            .flat_map(|(a, b)| self.valid_path(a, b))
            .collect()
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
fn shortest_button_sequence(num_robot: &str) -> [String; 4] {
    let dir_robot1 = NUMERIC_KEY_MAP.sequence(num_robot);
    let dir_robot2 = DIRECTIONAL_KEY_MAP.sequence(&dir_robot1);
    let dir_me = DIRECTIONAL_KEY_MAP.sequence(&dir_robot2);
    [num_robot.to_owned(), dir_robot1, dir_robot2, dir_me]
}

fn code_complexity_sequence(code: &str, sequence_len: usize) -> usize {
    let numeric_code = code.strip_suffix("A").unwrap().parse::<usize>().unwrap();
    numeric_code * sequence_len
}

fn code_complexity(code: &str) -> usize {
    code_complexity_sequence(code, shortest_button_sequence(code).last().unwrap().len())
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
    use super::*;
    use advent_of_code::verify;
    use rstest::rstest;

    #[rstest]
    #[case(&NUMERIC_KEY_MAP, '1', '0', ">vA")]
    #[case(&NUMERIC_KEY_MAP, '0', '1', "^<A")]
    #[case(&NUMERIC_KEY_MAP, '1', 'A', ">>vA")]
    #[case(&NUMERIC_KEY_MAP, 'A', '1', "^<<A")]
    #[case(&NUMERIC_KEY_MAP, '4', 'A', ">>vvA")]
    #[case(&NUMERIC_KEY_MAP, 'A', '4', "^^<<A")]
    #[case(&DIRECTIONAL_KEY_MAP, '^', '<', "v<A")]
    #[case(&DIRECTIONAL_KEY_MAP, '<', '^', ">^A")]
    #[case(&DIRECTIONAL_KEY_MAP, 'A', '<', "v<<A")]
    #[case(&DIRECTIONAL_KEY_MAP, '<', 'A', ">>^A")]
    fn valid_path(#[case] key_map: &KeyMap, #[case] a: char, #[case] b: char, #[case] expected: String) {
        assert_eq!(key_map.valid_path(a, b).into_iter().join(""), expected);
    }

    #[test]
    fn test_shortest_button_sequence() {
        assert_eq!(
            shortest_button_sequence("029A").map(|s| s.len()),
            [
                "029A",
                "<A^A>^^AvvvA",
                "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
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
