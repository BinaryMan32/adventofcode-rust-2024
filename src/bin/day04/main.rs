use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

#[derive(PartialEq, Clone, Copy)]
struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ row: self.row + rhs.row, col: self.col + rhs.col }
    }
}

struct WordSearch {
    letters: Vec<Vec<char>>,
    size: Pos
}

const DIRECTIONS: &[Pos] = &[
    Pos::new(-1, -1),
    Pos::new(-1, 0),
    Pos::new(-1, 1),
    Pos::new(0, -1),
    Pos::new(0, 1),
    Pos::new(1, -1),
    Pos::new(1, 0),
    Pos::new(1, 1)
];

impl WordSearch {
    fn create(input: Lines) -> Self {
        let letters = input.into_iter()
            .map(|line| {
                line.chars().collect_vec()
            })
            .collect_vec();
        let size = Pos::new(letters.len() as isize, letters[0].len() as isize);
        Self { letters, size }
    }

    fn word_count_str(&self, word: &str) -> usize {
        self.word_count_chars(&word.chars().collect_vec())
    }

    fn word_count_chars(&self, word: &[char]) -> usize {
        (0..self.size.row)
            .flat_map(|row| {
                (0..self.size.col)
                    .map(move |col| self.word_count_from(word, &Pos::new(row, col)))
            })
            .sum::<usize>()
    }

    fn word_count_from(&self, word: &[char], start: &Pos) -> usize {
        DIRECTIONS.iter().filter(|dir| self.is_word_at(word, start, dir)).count()
    }

    fn is_word_at(&self, word: &[char], start: &Pos, dir: &Pos) -> bool {
        match word.first() {
            Some(&a) if (self.char_at(start).is_some_and(|b| a == b)) =>
                self.is_word_at(&word[1..], &(*start + *dir), dir),
            Some(_) => false,
            None => true,
        }
    }

    fn char_at(&self, pos: &Pos) -> Option<char> {
        self.letters
            .get(pos.row as usize)
            .and_then(|row| row.get(pos.col as usize))
            .copied()
    }

    fn is_ms(a: char, b: char) -> bool {
        a == 'M' && b == 'S' || a == 'S' && b == 'M'
    }
    fn is_x_mas(&self, row: usize, col: usize) -> bool {
        (self.letters[row][col] == 'A')
            && Self::is_ms(self.letters[row-1][col-1], self.letters[row+1][col+1])
            && Self::is_ms(self.letters[row-1][col+1], self.letters[row+1][col-1])
    }

    fn x_mas_count(&self) -> usize {
        (1..(self.size.row as usize) - 1)
            .map(|row| {
                (1..(self.size.col as usize) - 1)
                    .filter(|&col| self.is_x_mas(row, col))
                    .count()
            })
            .sum()
    }
}

fn part1(input: Lines) -> String {
    WordSearch::create(input).word_count_str("XMAS").to_string()
}

fn part2(input: Lines) -> String {
    WordSearch::create(input).x_mas_count().to_string()
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
    fn char_at() {
        let ws = WordSearch::create("ab\ncd".lines());

        assert_eq!(ws.char_at(&Pos::new(0, 0)), Some('a'));
        assert_eq!(ws.char_at(&Pos::new(0, 1)), Some('b'));
        assert_eq!(ws.char_at(&Pos::new(1, 0)), Some('c'));
        assert_eq!(ws.char_at(&Pos::new(1, 1)), Some('d'));

        assert_eq!(ws.char_at(&Pos::new(-1, -1)), None);
        assert_eq!(ws.char_at(&Pos::new(-1, 0)), None);
        assert_eq!(ws.char_at(&Pos::new(-1, 1)), None);
        assert_eq!(ws.char_at(&Pos::new(-1, 2)), None);

        assert_eq!(ws.char_at(&Pos::new(0, -1)), None);
        assert_eq!(ws.char_at(&Pos::new(0, 2)), None);

        assert_eq!(ws.char_at(&Pos::new(1, -1)), None);
        assert_eq!(ws.char_at(&Pos::new(1, 2)), None);

        assert_eq!(ws.char_at(&Pos::new(2, -1)), None);
        assert_eq!(ws.char_at(&Pos::new(2, 0)), None);
        assert_eq!(ws.char_at(&Pos::new(2, 1)), None);
        assert_eq!(ws.char_at(&Pos::new(2, 2)), None);
    }

    #[test]
    fn is_word_at() {
        let ws = WordSearch::create("abc\ndef\nghi".lines());

        assert!(ws.is_word_at(&['a', 'e', 'i'], &Pos::new(0, 0), &Pos::new(1, 1)));
        assert!(!ws.is_word_at(&['a', 'e', 'x'], &Pos::new(0, 0), &Pos::new(1, 1)));
        assert!(!ws.is_word_at(&['a', 'x', 'y'], &Pos::new(0, 0), &Pos::new(1, 1)));
        assert!(!ws.is_word_at(&['x', 'y', 'z'], &Pos::new(0, 0), &Pos::new(1, 1)));

        assert!(ws.is_word_at(&['i', 'e', 'a'], &Pos::new(2, 2), &Pos::new(-1, -1)));
        assert!(!ws.is_word_at(&['i', 'e', 'x'], &Pos::new(2, 2), &Pos::new(-1, -1)));
        assert!(!ws.is_word_at(&['i', 'x', 'y'], &Pos::new(2, 2), &Pos::new(-1, -1)));
        assert!(!ws.is_word_at(&['x', 'y', 'z'], &Pos::new(2, 2), &Pos::new(-1, -1)));
    }

    #[test]
    fn word_count_str() {
        let ws = WordSearch::create("abc\ndef\nghi".lines());
        assert_eq!(ws.word_count_str("abc"), 1);
        assert_eq!(ws.word_count_str("cba"), 1);
        assert_eq!(ws.word_count_str("aei"), 1);
        assert_eq!(ws.word_count_str("iea"), 1);
        assert_eq!(ws.word_count_str("xyz"), 0);
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "18");
        verify!(part2, input, "9");
    }
}
