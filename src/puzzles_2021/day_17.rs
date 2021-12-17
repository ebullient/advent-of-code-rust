use crate::puzzle_input;
use std::cmp;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

pub fn run() {
    let input: Vec<i32> = puzzle_input::read_string("./input/2021-d17-input.txt")
        .replace("target area: ", "")
        .split(", ")
        .flat_map(|s| s.get(2..).unwrap().split(".."))
        .flat_map(|s| s.parse::<i32>())
        .collect();

    let rules = Rules::new(input[0], input[1], input[2], input[3]);
    let winner = rules.find_max_y();

    println!("** Part 1 Final: {:?}", winner.2);
    println!("** Part 2 Final: {:?}", rules.count_all_possible());
}

#[derive(Clone, Copy, Debug)]
struct Trajectory {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

fn to_zero(v: i32) -> i32 {
    match v.cmp(&0) {
        Ordering::Greater => v - 1,
        Ordering::Less => v + 1,
        Ordering::Equal => 0,
    }
}

#[derive(Clone, Debug)]
struct Rules {
    target_x: RangeInclusive<i32>,
    target_y: RangeInclusive<i32>,
}
impl Rules {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Rules {
        Rules {
            target_x: RangeInclusive::new(x1, x2),
            target_y: RangeInclusive::new(y1, y2),
        }
    }

    fn valid_waypoint(&self, p: &Trajectory) -> bool {
        // is there a possibility of this thing landing in bounds?
        p.x <= *self.target_x.end() && p.y >= *self.target_y.start()
    }

    fn target_contains(&self, p: &Trajectory) -> bool {
        self.target_x.contains(&p.x) && self.target_y.contains(&p.y)
    }

    fn step(&self, p: &Trajectory) -> Option<Trajectory> {
        let next = Trajectory {
            x: p.x + p.v_x,
            y: p.y + p.v_y,
            v_x: to_zero(p.v_x),
            v_y: p.v_y - 1,
        };
        if self.valid_waypoint(&next) {
            Some(next)
        } else {
            None
        }
    }

    fn fire(&self, v_x: i32, v_y: i32) -> (Option<Trajectory>, i32) {
        let mut max_y = 0;
        let mut prev = Trajectory {
            x: 0,
            y: 0,
            v_x,
            v_y,
        };
        while let Some(next) = self.step(&prev) {
            max_y = cmp::max(max_y, next.y);
            if self.target_contains(&next) {
                return (Some(next), max_y);
            }
            if next.v_x == 0 && next.x <= *self.target_x.start() {
                break;
            }
            prev = next;
        }
        (None, 0)
    }

    fn find_max_y(&self) -> (i32, i32, i32) {
        let mut winner = (0, 0, 0);

        for v_x in 1..=*self.target_x.end() {
            for v_y in 1..self.target_y.start().abs() {
                let (_, max_y) = self.fire(v_x, v_y);
                if max_y != 0 && max_y > winner.2 {
                    winner = (v_x, v_y, max_y);
                }
            }
        }
        winner
    }

    fn count_all_possible(&self) -> i32 {
        let mut total = 0;
        for v_x in 1..=*self.target_x.end() {
            for v_y in *self.target_y.start()..self.target_y.start().abs() {
                let (p, _) = self.fire(v_x, v_y);
                if p.is_some() {
                    total += 1;
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rules = Rules::new(20, 30, -10, -5);
        assert_eq!(3, rules.fire(7, 2).1);
        assert_eq!(6, rules.fire(6, 3).1);
        assert_eq!(0, rules.fire(9, 0).1);
        assert_eq!(45, rules.fire(6, 9).1);
        assert_eq!(0, rules.fire(17, -4).1);
    }

    #[test]
    fn test_calculate() {
        let rules = Rules::new(20, 30, -10, -5);
        assert_eq!((6, 9, 45), rules.find_max_y());
        assert_eq!(112, rules.count_all_possible());
    }
}
