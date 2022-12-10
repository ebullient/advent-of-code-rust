use crate::puzzle_input;
use regex::Regex;
use std::collections::HashSet;
use std::ops;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d09-input.txt");

    let mut grid = Grid::new(2);
    println!("** Part 1 Final: {:?}", move_rope(&input, &mut grid));

    grid = Grid::new(10);
    println!("** Part 2 Final: {:?}", move_rope(&input, &mut grid));
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}
impl ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const U: Coord = Coord { x: 0, y: 1 };
const D: Coord = Coord { x: 0, y: -1 };
const UL: Coord = Coord { x: -1, y: 1 };
const L: Coord = Coord { x: -1, y: 0 };
const DL: Coord = Coord { x: -1, y: -1 };
const UR: Coord = Coord { x: 1, y: 1 };
const R: Coord = Coord { x: 1, y: 0 };
const DR: Coord = Coord { x: 1, y: -1 };

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    // It literally does not matter: this is an imaginary
    // grid not a real one, so let's do (x, y) using cartesian
    // semantics rather than bonkers array indices
    data: HashSet<Coord>,
    snake: Vec<Coord>,
    len: usize,
}
impl Grid {
    fn new(len: usize) -> Grid {
        let mut data = HashSet::new();
        data.insert(Coord { x: 0, y: 0 });
        Grid {
            data,
            snake: vec![Coord { x: 0, y: 0 }; len],
            len,
        }
    }

    fn step(&mut self, d: char, n: i32) {
        let offset = match d {
            'U' => &U,
            'D' => &D,
            'L' => &L,
            'R' => &R,
            _ => panic!("Unknown direction: {:?}", d),
        };

        for _step in 0..n {
            self.snake[0] = self.snake[0] + *offset;
            for j in 1..self.len {
                self.snake[j] = self.tail(j - 1, j);
            }
            self.data.insert(self.snake[self.len - 1]);
        }
    }

    fn tail(&mut self, i: usize, j: usize) -> Coord {
        if self.snake[i] == self.snake[j]
            || ((self.snake[i].y - self.snake[j].y).abs() <= 1
                && (self.snake[i].x - self.snake[j].x).abs() <= 1)
        {
            return self.snake[j];
        }

        self.snake[j]
            + if self.snake[i].x == self.snake[j].x {
                if self.snake[i].y > self.snake[j].y {
                    U
                } else {
                    D
                }
            } else if self.snake[i].y == self.snake[j].y {
                if self.snake[i].x > self.snake[j].x {
                    R
                } else {
                    L
                }
            } else if self.snake[i].x > self.snake[j].x {
                if self.snake[i].y > self.snake[j].y {
                    UR
                } else {
                    DR
                }
            } else {
                if self.snake[i].y > self.snake[j].y {
                    UL
                } else {
                    DL
                }
            }
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.snake = vec![Coord { x: 0, y: 0 }; self.len];
        self.data.clear();
        self.data.insert(Coord { x: 0, y: 0 });
    }
}

fn move_rope(input: &[String], grid: &mut Grid) -> usize {
    lazy_static! {
        static ref INSTR: Regex = Regex::new(r"(L|R|U|D) (\d+)").unwrap();
    }

    for line in input {
        if INSTR.is_match(line) {
            let caps = INSTR.captures(line).unwrap();
            grid.step(
                (&caps[1]).chars().next().unwrap(),
                (&caps[2]).parse::<i32>().unwrap(),
            );
        } else {
            panic!("What is this? {:?}", line);
        }
    }

    grid.data.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2",
        );

        let mut grid = Grid::new(2);

        grid.step('R', 4);
        assert_eq!(grid.snake[0], Coord { x: 4, y: 0 });
        assert_eq!(grid.snake[1], Coord { x: 3, y: 0 });

        grid.step('U', 4);
        assert_eq!(grid.snake[0], Coord { x: 4, y: 4 });
        assert_eq!(grid.snake[1], Coord { x: 4, y: 3 });

        grid.step('L', 3);
        assert_eq!(grid.snake[0], Coord { x: 1, y: 4 });
        assert_eq!(grid.snake[1], Coord { x: 2, y: 4 });

        grid.step('D', 1);
        assert_eq!(grid.snake[0], Coord { x: 1, y: 3 });
        assert_eq!(grid.snake[1], Coord { x: 2, y: 4 });

        grid.step('R', 4);
        assert_eq!(grid.snake[0], Coord { x: 5, y: 3 });
        assert_eq!(grid.snake[1], Coord { x: 4, y: 3 });

        grid.step('D', 1);
        assert_eq!(grid.snake[0], Coord { x: 5, y: 2 });
        assert_eq!(grid.snake[1], Coord { x: 4, y: 3 });

        grid.step('L', 5);
        assert_eq!(grid.snake[0], Coord { x: 0, y: 2 });
        assert_eq!(grid.snake[1], Coord { x: 1, y: 2 });

        grid.step('R', 2);
        assert_eq!(grid.snake[0], Coord { x: 2, y: 2 });
        assert_eq!(grid.snake[1], Coord { x: 1, y: 2 });

        assert_eq!(grid.data.len(), 13);
        grid.reset();

        assert_eq!(move_rope(&input, &mut grid), 13);
    }

    #[test]
    fn test_2() {
        let input: Vec<String> = puzzle_input::split_string(
            "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2",
        );

        let mut grid = Grid::new(10);

        grid.step('R', 4);
        assert_eq!(grid.snake[0], Coord { x: 4, y: 0 });
        assert_eq!(grid.snake[1], Coord { x: 3, y: 0 });
        assert_eq!(grid.snake[3], Coord { x: 1, y: 0 });
        assert_eq!(grid.snake[9], Coord { x: 0, y: 0 });

        grid.step('U', 4);
        assert_eq!(grid.snake[0], Coord { x: 4, y: 4 });
        assert_eq!(grid.snake[1], Coord { x: 4, y: 3 });
        assert_eq!(grid.snake[3], Coord { x: 3, y: 2 });
        assert_eq!(grid.snake[9], Coord { x: 0, y: 0 });

        grid.step('L', 3);
        assert_eq!(grid.snake[0], Coord { x: 1, y: 4 });
        assert_eq!(grid.snake[1], Coord { x: 2, y: 4 });
        assert_eq!(grid.snake[3], Coord { x: 3, y: 2 });
        assert_eq!(grid.snake[9], Coord { x: 0, y: 0 });

        grid.step('D', 1);
        assert_eq!(grid.snake[0], Coord { x: 1, y: 3 });
        assert_eq!(grid.snake[1], Coord { x: 2, y: 4 });
        assert_eq!(grid.snake[3], Coord { x: 3, y: 2 });
        assert_eq!(grid.snake[9], Coord { x: 0, y: 0 });

        grid.step('R', 4);
        assert_eq!(grid.snake[0], Coord { x: 5, y: 3 });
        assert_eq!(grid.snake[1], Coord { x: 4, y: 3 });
        assert_eq!(grid.snake[3], Coord { x: 3, y: 2 });
        assert_eq!(grid.snake[9], Coord { x: 0, y: 0 });

        grid.step('D', 1);
        assert_eq!(grid.snake[0], Coord { x: 5, y: 2 });
        assert_eq!(grid.snake[1], Coord { x: 4, y: 3 });
        assert_eq!(grid.snake[3], Coord { x: 3, y: 2 });
        assert_eq!(grid.snake[9], Coord { x: 0, y: 0 });

        grid.step('L', 5);
        assert_eq!(grid.snake[0], Coord { x: 0, y: 2 });
        assert_eq!(grid.snake[1], Coord { x: 1, y: 2 });
        assert_eq!(grid.snake[3], Coord { x: 3, y: 2 });
        assert_eq!(grid.snake[9], Coord { x: 0, y: 0 });

        grid.step('R', 2);
        assert_eq!(grid.snake[0], Coord { x: 2, y: 2 });
        assert_eq!(grid.snake[1], Coord { x: 1, y: 2 });
        assert_eq!(grid.snake[3], Coord { x: 3, y: 2 });
        assert_eq!(grid.snake[9], Coord { x: 0, y: 0 });

        assert_eq!(grid.data.len(), 1);
        grid.reset();

        assert_eq!(move_rope(&input, &mut grid), 1);
    }

    #[test]
    fn test_3() {
        let input: Vec<String> = puzzle_input::split_string(
            "R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20",
        );

        let mut grid = Grid::new(10);
        assert_eq!(move_rope(&input, &mut grid), 36);
    }
}
