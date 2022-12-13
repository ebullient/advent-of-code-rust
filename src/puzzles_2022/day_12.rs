use crate::puzzle_input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d12-input.txt");
    let grid = Grid::new(&input);
    grid.dump();

    println!("** Part 1 Final: {:?}", grid.find_path());
    println!("** Part 2 Final: {:?}", grid.find_all_paths());
}

// elevation of each square of the grid is given by a single lowercase letter:
// a is lowest, z is highest; S == current position, E == best signal
// objective: fewest steps possible. elevation at most one higher.

const LOWER: i32 = '`' as i32;
const OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    data: HashMap<(i32, i32), char>,
    start: (i32, i32),
    end: (i32, i32),
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &[String]) -> Grid {
        let mut data: HashMap<(i32, i32), char> = HashMap::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut max = '`';

        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.trim().chars().enumerate() {
                let here = (y as i32, x as i32);
                if c == 'S' {
                    start = here;
                    data.insert(start, 'a'); // start is like an 'a'
                } else if c == 'E' {
                    end = here;
                } else {
                    data.insert(here, c);
                    if c > max {
                        max = c;
                    }
                }
            }
        }
        data.insert(end, char::from_u32(max as u32 + 1).unwrap());

        Grid {
            data,
            start,
            end,
            height: input.len(),
            width: input[0].len(),
        }
    }

    fn altitude(&self, pt: (i32, i32)) -> i32 {
        *self.data.get(&pt).unwrap() as i32 - LOWER
    }

    fn value(&self, pt: (i32, i32)) -> char {
        if pt == self.start {
            'S'
        } else if pt == self.end {
            'E'
        } else {
            *self.data.get(&pt).unwrap()
        }
    }

    fn neighbors(&self, p: (i32, i32)) -> Vec<(i32, i32)> {
        let mut result = Vec::with_capacity(4);
        for o in OFFSETS {
            let r = (p.0 + o.0, p.1 + o.1);
            if let Some(_) = self.data.get(&r) {
                result.push(r);
            }
        }
        result
    }

    fn bfs(&self, start: (i32, i32)) -> usize {
        let mut queue: VecDeque<((i32, i32), String)> = VecDeque::new();
        let mut seen: HashSet<(i32, i32)> = HashSet::new();

        queue.push_back((start, String::from(self.value(start))));

        while let Some(current) = queue.pop_front() {
            if current.0 == self.end {
                println!("We made it! {:?}: {:?}", current.1.len() - 1, current.1);
                return current.1.len() - 1;
            }
            if seen.insert(current.0) {
                let a = self.altitude(current.0);
                // println!(
                //     "Looking at {:?}: {:?} == {:?} :: {:?}",
                //     current.0,
                //     self.value(current.0),
                //     a,
                //     &current.1
                // );
                self.neighbors(current.0)
                    .iter()
                    .filter(|n| self.altitude(**n) - a <= 1)
                    .for_each(|n| {
                        queue.push_back((*n, format!("{}{}", current.1, self.value(current.0))))
                    });
            }
        }
        0
    }

    fn find_path(&self) -> usize {
        self.bfs(self.start)
    }

    fn find_all_paths(&self) -> usize {
        self.data
            .iter()
            .filter(|x| *x.1 == 'a')
            .map(|x| self.bfs(*x.0))
            .filter(|x| *x != 0)
            .min()
            .unwrap()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data.get(&(y as i32, x as i32)).unwrap());
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
            "Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi",
        );
        let grid = Grid::new(&input);
        grid.dump();

        assert_eq!(grid.find_path(), 31);
        assert_eq!(grid.find_all_paths(), 29);
    }
}
