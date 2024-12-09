use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use core::fmt;
use std::{collections::VecDeque, fmt::Write, iter::repeat_n, str::Lines};

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
        #[cfg(test)] println!("Disk={self}");
        let next_free = 0usize;
        let last_used = self.blocks.len();
        while let Some((next_free, last_used)) =
            self.next_free(next_free + 1)
                .zip(self.last_used(last_used - 1))
                .filter(|(nf, lu)| nf < lu
        ) {
            self.blocks[next_free] = self.blocks[last_used];
            self.blocks[last_used] = FREE_ID;
            #[cfg(test)] println!("Disk={self}");
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

#[derive(Clone, Copy)]
struct Chunk {
    start: usize,
    length: usize,
}

struct DiskMap {
    files: Vec<Chunk>,
    empty: VecDeque<Chunk>,
}

impl fmt::Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut files = self.files.iter().enumerate().collect_vec();
        files.sort_by_key(|f| f.1.start);
        let mut prev = 0;
        for (i, chunk) in files {
            for _ in prev..chunk.start {
                f.write_char('.')?;
            }
            for _ in 0..(chunk.length) {
                write!(f, "{}", i)?;
            }
            prev = chunk.start + chunk.length;
        }
        Ok(())
    }
}

fn sum_of_n(n: usize) -> usize {
    n * (n-1) / 2
}

impl DiskMap {
    fn parse(line: &str) -> Self {
        let mut start: usize = 0;
        let mut is_file = true;
        let mut files: Vec<Chunk> = Vec::new();
        let mut empty: VecDeque<Chunk> = VecDeque::new();
        for length in line.chars().flat_map(|c| c.to_digit(10).map(|n| n as usize)) {
            if is_file {
                files.push(Chunk{start, length});
            } else {
                empty.push_back(Chunk{start, length});
            }
            is_file = !is_file;
            start += length;
        }
        Self{files, empty}
    }

    fn free_chunk_at(&mut self, index: usize, used_length: usize) {
        let empty = self.empty[index];
        if empty.length > used_length {
            self.empty[index].start += used_length;
            self.empty[index].length -= used_length;
        } else {
            self.empty.remove(index);
        }
    }

    fn get_compacted_file_start(&mut self, file: Chunk) -> Option<usize> {
        self.empty.iter()
            .take_while(|empty| empty.start < file.start)
            .position(|empty| empty.length >= file.length)
            .map(|index| {
                let file_start = self.empty[index].start;
                self.free_chunk_at(index, file.length);
                file_start
            })
    }

    fn compact(&mut self) {
        #[cfg(test)] println!("DiskMap={self}");
        for id in (0..self.files.len()).rev() {
            if let Some(start) = self.get_compacted_file_start(self.files[id]) {
                self.files[id].start = start;
                #[cfg(test)] println!("DiskMap={self}");
            }
        }
    }

    fn checksum(&self) -> usize {
        self.files.iter().enumerate().map(|(id, chunk)| {
            id * (chunk.start * chunk.length + sum_of_n(chunk.length)) 
        })
        .sum()
    }
}

fn part2(mut input: Lines) -> String {
    let mut disk_map = DiskMap::parse(input.next().expect("one line of input"));
    disk_map.compact();
    disk_map.checksum().to_string()
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
        verify!(part2, input, "2858");
    }
}
