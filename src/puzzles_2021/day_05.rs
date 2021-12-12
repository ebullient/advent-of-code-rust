use crate::puzzle_input;
use regex::Regex;
use std::collections::HashMap;
use std::cmp;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d05-input.txt");

    let grid = Grid::new(&input, false);
    println!("** Part 1 Final: {:?}", grid.at_least_2());

    let grid2 = Grid::new(&input, true);
    println!("** Part 2 Final: {:?}", grid2.at_least_2());
}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    data: HashMap<(i32, i32), i32>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: &Vec<String>, diagonals: bool) -> Grid {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"(\d+),(\d+)[^\d]+(\d+),(\d+)").unwrap();
        }

        let mut data: HashMap<(i32, i32), i32> = HashMap::new();
        let mut height: i32 = 0;
        let mut width: i32 = 0;
        for line in input {
            let cap = LINE_RE.captures(line).unwrap();
            let x1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y1 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let x2 = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let y2 = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
            height = cmp::max(y1, cmp::max(y2, height));
            width = cmp::max(x1, cmp::max(x2, width));
            if x1 == x2 {
                let b = cmp::min(y1, y2);
                let e = cmp::max(y1, y2) + 1;
                for y in b..e {
                    let counter = data.entry((x1, y)).or_insert(0);
                    *counter += 1;
                }
            } else if y1 == y2 {
                let b = cmp::min(x1, x2);
                let e = cmp::max(x1, x2) + 1;
                for x in b..e {
                    let counter = data.entry((x, y1)).or_insert(0);
                    *counter += 1;
                }
            } else if diagonals && (x1 - x2).abs() == (y1 - y2).abs() {
                let mut x = cmp::min(x1, x2);
                let range = 1 + (x1 - x2).abs();
                let up = if x == x1 { y1 < y2 } else { y2 < y1 };
                let mut y = if x == x1 { y1 } else { y2 };
                for _ in 0..range {
                    let counter = data.entry((x, y)).or_insert(0);
                    *counter += 1;
                    x += 1;
                    y = if up { y + 1 } else { y - 1 };
                }
            }
        }

        Grid {
            data: data,
            height: height + 1,
            width: width + 1,
        }
    }

    fn at_least_2(&self) -> usize {
        self.data.values().filter(|x| **x >= 2).count()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(pt) = self.data.get(&(x, y)) {
                    print!("{}", pt);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string("0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2");

        let grid = Grid::new(&input, false);
        grid.dump();

        assert_eq!(5, grid.at_least_2());

        let grid2 = Grid::new(&input, true);
        grid2.dump();
        assert_eq!(12, grid2.at_least_2());
    }
}
