use crate::puzzle_input;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d09-input.txt");
    let grid = Grid::new(&input);

    println!("** Part 1 Final: {:?}", grid.find_risk());
    assert_eq!(491, grid.find_risk());
    println!("** Part 2 Final: {:?}", grid.find_basins());
}

#[derive(Clone, Copy, Debug)]
struct Point {
    y: usize,
    x: usize,
    height: i32,
}
impl hash::Hash for Point {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl cmp::Eq for Point {}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    data: HashMap<(usize, usize), Point>,
    width: usize,
    height: usize,
}
impl Grid {
    fn new(input: &[String]) -> Grid {
        let mut data: HashMap<(usize, usize), Point> = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, col) in row.trim().chars().enumerate() {
                data.insert(
                    (y, x),
                    Point {
                        x,
                        y,
                        height: col.to_digit(10).unwrap() as i32,
                    },
                );
            }
        }

        Grid {
            data,
            height: input.len(),
            width: input[0].len(),
        }
    }

    fn neighbors(&self, p: &Point) -> Vec<&Point> {
        let mut result: Vec<&Point> = vec![];
        // bad things happen if you try to take a usize
        if p.y > 0 {
            result.push(self.data.get(&(p.y - 1, p.x)).unwrap());
        }
        if p.x > 0 {
            result.push(self.data.get(&(p.y, p.x - 1)).unwrap());
        }
        if let Some(n) = self.data.get(&(p.y + 1, p.x)) {
            result.push(n);
        }
        if let Some(n) = self.data.get(&(p.y, p.x + 1)) {
            result.push(n);
        }
        result
    }

    fn basin(&self, p: &Point, seen: &mut HashSet<Point>) {
        if seen.contains(p) {
            return;
        }
        seen.insert(*p);
        self.neighbors(p)
            .iter()
            .filter(|n| n.height != 9)
            .for_each(|n| self.basin(n, seen));
    }

    fn find_basins(&self) -> i32 {
        let mut result: Vec<i32> = self
            .data
            .values()
            .filter(|p| self.neighbors(p).iter().all(|n| n.height > p.height))
            .map(|p| {
                let mut seen = HashSet::new();
                self.basin(p, &mut seen);
                seen.len() as i32
            })
            .collect();

        result.sort_by(|a, b| b.cmp(a));
        result.iter().take(3).product()
    }

    fn find_risk(&self) -> i32 {
        self.data
            .values()
            .filter(|p| self.neighbors(p).iter().all(|n| n.height > p.height))
            .map(|p| p.height + 1)
            .sum()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = self.data.get(&(y, x)).unwrap();
                print!("  {:?}  ", p.height);
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "2199943210
            3987894921
            9856789892
            8767896789
            9899965678",
        );

        let grid = Grid::new(&input);
        grid.dump();
        assert_eq!(15, grid.find_risk());
        assert_eq!(1134, grid.find_basins());
    }
}
