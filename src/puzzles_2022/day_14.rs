use crate::puzzle_input;
use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d14-input.txt");

    let mut rocks = Rocks::new(&input, false);
    println!("** Part 1 Final: {:?}", rocks.fill_with_sand());
    rocks.draw();

    rocks = Rocks::new(&input, true);
    println!("** Part 2 Final: {:?}", rocks.fill_with_sand());
    rocks.draw();
}

const OFFSETS: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];

fn add_rocks(data: &mut HashMap<(i32, i32), char>, p1: (i32, i32), p2: (i32, i32)) {
    if p1.0 == p2.0 {
        for i in cmp::min(p1.1, p2.1)..=cmp::max(p1.1, p2.1) {
            data.insert((p1.0, i), '#');
        }
    } else {
        // p1.1 == p2.1
        for i in cmp::min(p1.0, p2.0)..=cmp::max(p1.0, p2.0) {
            data.insert((i, p1.1), '#');
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Rocks {
    data: HashMap<(i32, i32), char>,
    y_max: i32,
    floor: bool,
}
impl Rocks {
    fn new(input: &[String], floor: bool) -> Rocks {
        let mut data = HashMap::new();
        let mut y_max = 0 as i32;

        for line in input.iter() {
            // 503,4 -> 502,4 -> 502,9 -> 494,9
            for t in line
                .split(" -> ")
                .map(|s| {
                    let mut p = s.split(",");
                    (
                        p.next().unwrap().parse::<i32>().unwrap(),
                        p.next().unwrap().parse::<i32>().unwrap(),
                    )
                })
                .inspect(|p| {
                    if p.1 > y_max {
                        y_max = p.1;
                    }
                })
                .tuple_windows::<(_, _)>()
            {
                add_rocks(&mut data, t.0, t.1);
            }
        }
        if floor {
            y_max += 1;
        }
        Rocks { data, y_max, floor }
    }

    fn fall(&mut self, start: (i32, i32)) -> Option<(i32, i32)> {
        for o in OFFSETS {
            let p = (start.0 + o.0, start.1 + o.1);
            if !self.data.contains_key(&p) {
                if self.floor && p.1 > self.y_max {
                    break;
                }
                return Some(p);
            }
        }
        None
    }

    fn fill_with_sand(&mut self) -> i32 {
        let mut n = 0;
        'grain: loop {
            let mut grain = (500, 0);
            while let Some(next) = self.fall(grain) {
                grain = next;
                if !self.floor && grain.1 == self.y_max {
                    break 'grain;
                }
            }
            self.data.insert(grain, 'o');
            n += 1;
            if grain == (500, 0) {
                break;
            }
        }
        n
    }

    #[allow(dead_code)]
    fn draw(&self) {
        let mut x_min = 500;
        let mut x_max = 500;
        self.data.keys().for_each(|p| {
            if p.0 < x_min {
                x_min = p.0;
            } else if p.0 > x_max {
                x_max = p.0;
            }
        });

        for y in 0..=self.y_max {
            for x in x_min..=x_max {
                if let Some(p) = self.data.get(&(x, y)) {
                    print!("{}", p);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        if self.floor {
            for _ in x_min..=x_max {
                print!("#");
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
            "498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9",
        );
        let mut rocks = Rocks::new(&input, false);
        assert_eq!(rocks.fill_with_sand(), 24);
        rocks.draw();

        println!("====");

        rocks = Rocks::new(&input, true);
        rocks.draw();
        assert_eq!(rocks.fill_with_sand(), 93);
        rocks.draw();
    }
}
