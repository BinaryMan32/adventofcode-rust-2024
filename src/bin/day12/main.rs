use advent_of_code::{create_runner, named, Named, Runner};
use glam::I16Vec2;
use itertools::Itertools;
use std::{collections::{HashMap, VecDeque}, str::Lines};

type Pos = I16Vec2;

const DIRECTIONS: [Pos; 4] = [
    Pos::new(1, 0),
    Pos::new(0, -1),
    Pos::new(-1, 0),
    Pos::new(0, 1),
];

const CORNERS: [Pos; 4] = [
    Pos::new(1, -1),
    Pos::new(-1, -1),
    Pos::new(-1, 1),
    Pos::new(1, 1),
];

#[derive(Clone)]
struct Plot {
    kind: char,
    visited: bool,
}

impl Plot {
    fn new(kind: char) -> Self {
        Self{kind, visited: false}
    }

    fn first_visit(&mut self) -> bool {
        if !self.visited {
            self.visited = true;
            true
        } else {
            false
        }
    }
}

struct GardenPlotMap {
    plots: Vec<Vec<Plot>>,
    size: Pos,
}

impl GardenPlotMap {
    fn parse(input: Lines) -> Self {
        let plots = input.into_iter()
            .map(|line| {
                line.chars().map(Plot::new).collect_vec()
            })
            .collect_vec();
        let size = Pos::new(plots[0].len() as i16, plots.len() as i16);
        Self{plots, size}
    }

    /**
     * Determines how many corners exist at this position.
     * 
     * Assume that `pos` is "inside" the queried region `kind`.
     * 
     * Here are the cases to consider, where:
     * - `p` is the queried position
     * - `I` is outside the region
     * - `X` is inside the region
     *
     * 1. a "concave" corner:
     *    ```
     *    pX pX
     *    XX XI
     *    ```
     *    Note that each `X` could be a different region, and it's possible for
     *    the region to be complex and wrap around such that the diagonal plot
     *    is the same region again.
     * 
     * 2. a "convex" corner:
     *    ```
     *    pI
     *    IX
     *    ```
     */
    fn count_corners(&self, pos: Pos, kind: char) -> usize {
        let in_adj = DIRECTIONS
            .map(|off| {
                self.get(pos + off).is_some_and(|p| p.kind == kind)
            });
        let in_corner = CORNERS
            .map(|off| {
                self.get(pos + off).is_some_and(|p| p.kind == kind)
            });
        [(0, 1), (1, 2), (2, 3), (3, 0)].into_iter()
            .filter(|&(a, b)| {
                (in_adj[a] == in_adj[b]) && (!in_adj[a] || !in_corner[a])
            })
            .count()
    }

    fn get_unsafe_mut(&mut self, pos: Pos) -> &mut Plot {
        &mut self.plots[pos.y as usize][pos.x as usize]
    }

    fn get_unsafe(&self, pos: Pos) -> &Plot {
        &self.plots[pos.y as usize][pos.x as usize]
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < self.size.x && pos.y >= 0 && pos.y < self.size.y
    }

    fn get_mut(&mut self, pos: Pos) -> Option<&mut Plot> {
        self.in_bounds(&pos).then(|| self.get_unsafe_mut(pos))
    }

    fn get(&self, pos: Pos) -> Option<&Plot> {
        self.in_bounds(&pos).then(|| self.get_unsafe(pos))
    }

    /**
     * Find a region which contains the position `pos` and return a description
     * of its shape.
     *
     * It's easy to count area using the number of plots with matching chars.
     * 
     * To figure out how to count perimeter, consider a few examples:
     *
     * shape  area perimeter
     *
     *    A      1         4
     *
     *   AA      2         6
     *
     *  AAA      3         8
     *
     *   AA      3         8      
     *    A
     *
     *  AAA      6        10
     *  AAA
     *
     *  AAA      9        12
     *  AAA
     *  AAA
     *
     *  AAA      8        16
     *  A A
     *  AAA
     *
     * One way to get the perimeter is to count the 4 edges of every plot.
     * We already have the area which is the number of plots.
     * Then, we can subtract the number of internal edges found while
     * traversing plots to find the area.
     * 
     * The number of sides is harder, but it's the same as the number of
     * corners which can be computed by looking at a position an its neighbors.
     */
    fn get_region(&mut self, pos: Pos) -> Region {
        let region_kind = self.get_unsafe_mut(pos).kind;
        let mut traverse: VecDeque<Pos> = VecDeque::new();
        let mut area: usize = 0;
        let mut internal_sides: usize = 0;
        let mut corners: usize = 0;
        traverse.push_back(pos);
        while let Some(pos) = traverse.pop_front() {
            for offset in DIRECTIONS.iter() {
                let new_pos = pos + offset;
                if let Some(new_plot) = self.get_mut(new_pos) {
                    if new_plot.kind == region_kind {
                        internal_sides += 1;
                        if new_plot.first_visit() {
                            traverse.push_back(new_pos);
                        }
                    }
                }
            }
            area += 1;
            corners += self.count_corners(pos, region_kind);
        }
        let total_sides = 4 * area;
        Region{area, perimeter: total_sides - internal_sides, sides: corners}
    }

    fn get_regions(&mut self) -> HashMap<char, Vec<Region>> {
        let mut regions: HashMap<char, Vec<Region>> = HashMap::new();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Pos::new(x, y);
                let plot = self.get_unsafe_mut(pos);
                if plot.first_visit() {
                    regions.entry(plot.kind).or_default().push(self.get_region(pos));
                } 
            }
        }
        regions
    }
}

#[derive(Debug, PartialEq)]
struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.perimeter
    }

    fn bulk_price(&self) -> usize {
        self.area * self.sides
    }
}

fn part1(input: Lines) -> String {
    let regions = GardenPlotMap::parse(input).get_regions();
    regions
        .values()
        .flat_map(|rs| rs.iter().map(|r| r.price()))
        .sum::<usize>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let regions = GardenPlotMap::parse(input).get_regions();
    regions
        .values()
        .flat_map(|rs| rs.iter().map(|r| r.bulk_price()))
        .sum::<usize>()
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
    use super::*;
    use advent_of_code::verify;

    #[test]
    fn get_regions() {
        assert_eq!(
            GardenPlotMap::parse(include_str!("example.txt").lines()).get_regions(),
            HashMap::from([
                ('C', vec![
                    Region{area: 14, perimeter: 28, sides: 22},
                    Region{area: 1, perimeter: 4, sides: 4}
                ]),
                ('E', vec![Region{area: 13, perimeter: 18, sides: 8}]),
                ('F', vec![Region{area: 10, perimeter: 18, sides: 12}]),
                ('I', vec![
                    Region{area: 4, perimeter: 8, sides: 4},
                    Region{area: 14, perimeter: 22, sides: 16}
                ]),
                ('J', vec![Region{area: 11, perimeter: 20, sides: 12}]),
                ('M', vec![Region{area: 5, perimeter: 12, sides: 6}]),
                ('R', vec![Region{area: 12, perimeter: 18, sides: 10}]),
                ('S', vec![Region{area: 3, perimeter: 8, sides: 6}]),
                ('V', vec![Region{area: 13, perimeter: 20, sides: 10}]),
            ])
        );
    }

    fn get_region_sides(input: Lines) -> HashMap<char, Vec<usize>> {
        GardenPlotMap::parse(input)
            .get_regions()
            .into_iter()
            .map(|(kind, regions)| {
                (kind, regions.into_iter().map(|r| r.sides).collect_vec())
            })
            .collect()
    }

    #[test]
    fn sides1() {
        let example = r"AAAA
BBCD
BBCC
EEEC";
        assert_eq!(
            get_region_sides(example.lines()),
            HashMap::from([
                ('A', vec![4]),
                ('B', vec![4]),
                ('C', vec![8]),
                ('D', vec![4]),
                ('E', vec![4]),
            ])
        );
    }

    #[test]
    fn sides2() {
        let example = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(
            get_region_sides(example.lines()),
            HashMap::from([
                ('E', vec![12]),
                ('X', vec![4, 4]),
            ])
        );
    }

    #[test]
    fn sides3() {
        let example = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(
            get_region_sides(example.lines()),
            HashMap::from([
                ('A', vec![12]),
                ('B', vec![4, 4]),
            ])
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "1930");
        verify!(part2, input, "1206");
    }
}
