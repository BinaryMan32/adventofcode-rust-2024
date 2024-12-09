use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use core::fmt;
use std::{fmt::Write, iter::repeat_n, str::Lines};

type FileId = i16;
const FREE_ID: FileId = -1i16;

struct Disk {
    blocks: Vec<FileId>,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in self.blocks.iter() {
            if b == FREE_ID {
                f.write_char('.')?;
            } else {
                write!(f, "{}", b)?;
            }
        }
        Ok(())
    }
}

impl Disk {
    fn parse(line: &str) -> Self {
        let mut id: FileId = FREE_ID;
        let mut is_file = false;
        let blocks = line.chars()
            .flat_map(|c| c.to_digit(10).map(|n| n as usize))
            .flat_map(|length| {
                is_file = !is_file;
                if is_file {id += 1};
                repeat_n(if is_file {id} else {FREE_ID}, length)
            })
            .collect_vec();
        Self{blocks}
    }

    fn next_free(&self, from: usize) -> Option<usize> {
        self.blocks[from..].iter().position(|&id| id == FREE_ID).map(|i| i + from)
    }

    fn last_used(&self, from: usize) -> Option<usize> {
        self.blocks[0..=from].iter().rposition(|&id| id != FREE_ID)
    }

    fn compact(&mut self) {
        #[cfg(test)] println!("blocks={self}");
        let next_free = 0usize;
        let last_used = self.blocks.len();
        while let Some((next_free, last_used)) =
            self.next_free(next_free + 1)
                .zip(self.last_used(last_used - 1))
                .filter(|(nf, lu)| nf < lu
        ) {
            self.blocks[next_free] = self.blocks[last_used];
            self.blocks[last_used] = FREE_ID;
            #[cfg(test)] println!("blocks={self}");
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter(|&(_, &id)| (id != FREE_ID))
            .map(|(i, &id)| id as usize * i)
            .sum()
    }
}

fn part1(mut input: Lines) -> String {
    let mut disk = Disk::parse(input.next().expect("one line of input"));
    disk.compact();
    disk.checksum().to_string()
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
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "1928");
        verify!(part2, input, "0");
    }
}
