use crate::puzzle_input;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d12-input1.txt");
    println!("** Part 1 Final: {:?}", move_the_ferry(input.clone()));
    println!("** Part 2 Final: {:?}", move_ferry_via_waypoint(input));
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
enum Direction {
    East = 0, // 0
    South,    // 1
    West,     // 2
    North,    // 3
}
impl Direction {
    fn from(d: char) -> Direction {
        match d {
            'N' => Direction::North,
            'S' => Direction::South,
            'W' => Direction::West,
            _ => Direction::East,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}
impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Mul<i32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/** Part 1: Rotate the Ferry */
fn rotate_ferry(start: &Direction, deg: i32, rotation: char) -> Direction {
    let i = *start as i32;
    let steps = deg / 90;
    let result = if rotation == 'R' {
        (i + steps) % 4
    } else {
        (i - steps + 4) % 4
    };

    match FromPrimitive::from_i32(result) {
        None => Direction::East,
        Some(direction) => direction,
    }
}

/** Part 1: Move the Ferry */
fn move_the_ferry(input: Vec<String>) -> i32 {
    let mut list = input;
    let mut facing = Direction::East;
    let mut coord = Coord { x: 0, y: 0 };
    for heading in list.iter_mut() {
        let i = heading.remove(0);
        let n = heading.parse::<i32>().unwrap();
        if i == 'L' || i == 'R' {
            facing = rotate_ferry(&facing, n, i);
        } else {
            let direction = if i == 'F' { facing } else { Direction::from(i) };
            coord = coord
                + match direction {
                    Direction::East => Coord { x: n, y: 0 },
                    Direction::West => Coord { x: -n, y: 0 },
                    Direction::North => Coord { x: 0, y: n },
                    Direction::South => Coord { x: 0, y: -n },
                };
        }
    }
    coord.x.abs() + coord.y.abs()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Waypoint {
    offset: Coord,
    position: Coord,
}
impl Waypoint {
    fn new(start: Coord) -> Waypoint {
        Waypoint {
            offset: start,
            position: start,
        }
    }

    fn adjust(&mut self, delta: Coord) {
        self.offset = self.offset + delta;
        self.position = self.position + delta;
    }
}

fn rotate_waypoint(waypoint: Waypoint, ferry: &Coord, deg: i32, r: char) -> Waypoint {
    let mut result = Coord { x: 0, y: 0 };
    // Action L means to rotate the waypoint around the ship left (counter-clockwise)
    // the given number of degrees.
    // Action R means to rotate the waypoint around the ship right (clockwise)
    // the given number of degrees.
    if deg == 180 {
        result.x = -1 * waypoint.offset.x;
        result.y = -1 * waypoint.offset.y;
    } else if (r == 'L' && deg == 90) || (r == 'R' && deg == 270) {
        result.y = waypoint.offset.x;
        result.x = -1 * waypoint.offset.y;
    } else {
        result.y = -1 * waypoint.offset.x;
        result.x = waypoint.offset.y;
    }
    Waypoint {
        offset: result,
        position: result + *ferry,
    }
}

fn move_ferry_via_waypoint(input: Vec<String>) -> i32 {
    let mut list = input;
    let mut ferry_pos = Coord { x: 0, y: 0 };
    let mut waypoint = Waypoint::new(Coord { x: 10, y: 1 });

    for heading in list.iter_mut() {
        let i = heading.remove(0);
        let n = heading.parse::<i32>().unwrap();
        match i {
            'L' | 'R' => {
                waypoint = rotate_waypoint(waypoint, &ferry_pos, n, i);
            },
            'N' => waypoint.adjust(Coord {x: 0,  y: n}),
            'S' => waypoint.adjust(Coord {x: 0,  y: -n}),
            'E' => waypoint.adjust(Coord {x: n,  y: 0}),
            'W' => waypoint.adjust(Coord {x: -n, y: 0}),
            'F' => {
                ferry_pos = ferry_pos + ( waypoint.offset * n );
                waypoint.position = ferry_pos + waypoint.offset;
            },
            _ => {}

        }
    }

    ferry_pos.x.abs() + ferry_pos.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_ferry() {
        assert_eq!(rotate_ferry(&Direction::East, 90, 'R'), Direction::South);
        assert_eq!(rotate_ferry(&Direction::North, 180, 'R'), Direction::South);
        assert_eq!(rotate_ferry(&Direction::West, 270, 'R'), Direction::South);

        assert_eq!(rotate_ferry(&Direction::North, 90, 'L'), Direction::West);
        assert_eq!(rotate_ferry(&Direction::East, 180, 'L'), Direction::West);
        assert_eq!(rotate_ferry(&Direction::South, 270, 'L'), Direction::West);
    }

    #[test]
    fn test_rotate_waypoint() {
        // 10 units east and 4 units north:
        let test = Waypoint {
            offset: Coord { x: 10, y: 4 },
            position: Coord { x: 10, y: 1 },
        };

        let origin = Coord { x: 0, y: 0 };

        // R 180 --> 10 units west and 4 units south
        let mut result = rotate_waypoint(test.clone(), &origin, 180, 'R');
        assert_eq!(result.offset, Coord { x: -10, y: -4 });
        assert_eq!(result.position, result.offset);
        // R 90 --> 4 units east and 10 units south
        result = rotate_waypoint(test.clone(), &origin, 90, 'R');
        assert_eq!(result.offset, Coord { x: 4, y: -10 });
        assert_eq!(result.position, result.offset);
        // R 270 --> 4 units west and 10 units north
        result = rotate_waypoint(test.clone(), &origin, 270, 'R');
        assert_eq!(result.offset, Coord { x: -4, y: 10 });
        assert_eq!(result.position, result.offset);

        // L 270 --> 4 units east and 10 units south
        result = rotate_waypoint(test.clone(), &origin, 270, 'L');
        assert_eq!(result.offset, Coord { x: 4, y: -10 });
        assert_eq!(result.position, result.offset);
        // L 90 --> 4 units west and 10 units north
        result = rotate_waypoint(test.clone(), &origin, 90, 'L');
        assert_eq!(result.offset, Coord { x: -4, y: 10 });
        assert_eq!(result.position, result.offset);
    }

    #[test]
    fn test_move_the_ferry() {
        let input: Vec<String> = "F10
        N3
        F7
        R90
        F11"
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect();

        assert_eq!(move_the_ferry(input.clone()), 25);
        assert_eq!(move_ferry_via_waypoint(input.clone()), 286);
    }
}
