use crate::puzzle_input;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2021-d02-input.txt");

    let p = find_position(&input);
    println!("** Part 1 Final: {:?}", p.h * p.d);

    let h = find_heading(&input);
    println!("** Part 2 Final: {:?}", h.h * h.d);
}

#[derive(Clone, Copy, Debug)]
struct Position {
    h: i32,
    d: i32,
}
impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            h: self.h + rhs.h,
            d: self.d + rhs.d,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Heading {
    h: i32,
    d: i32,
    aim: i32,
}
impl std::ops::Add for Heading {
    type Output = Heading;

    fn add(self, rhs: Heading) -> Self::Output {
        Heading {
            h: self.h + rhs.h,
            d: self.d + rhs.d,
            aim: self.aim + rhs.aim,
        }
    }
}

// forward X increases the horizontal position by X units.
// down X increases the depth by X units.
// up X decreases the depth by X units.
fn find_position(course: &[String]) -> Position {
    let mut position = Position { h: 0, d: 0 };

    for entry in course {
        let mut split = entry.split_whitespace();
        let direction = split.next().unwrap();
        let units = split.next().unwrap().parse::<i32>().unwrap();

        position = match direction {
            "forward" => position + Position { h: units, d: 0 },
            "up" => position + Position { h: 0, d: -units },
            "down" => position + Position { h: 0, d: units },
            _ => position,
        };
    }

    position
}

// down X increases your aim by X units.
// up X decreases your aim by X units.
// forward X does two things:
// It increases your horizontal position by X units.
// It increases your depth by your aim multiplied by X.
fn find_heading(course: &[String]) -> Heading {
    let mut heading = Heading { h: 0, d: 0, aim: 0 };

    for entry in course {
        let mut split = entry.split_whitespace();
        let direction = split.next().unwrap();
        let units = split.next().unwrap().parse::<i32>().unwrap();

        heading = match direction {
            "forward" => {
                heading
                    + Heading {
                        h: units,
                        d: heading.aim * units,
                        aim: 0,
                    }
            }
            "up" => {
                heading
                    + Heading {
                        h: 0,
                        d: 0,
                        aim: -units,
                    }
            }
            "down" => {
                heading
                    + Heading {
                        h: 0,
                        d: 0,
                        aim: units,
                    }
            }
            _ => heading,
        };
    }

    heading
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2",
        );

        let p = find_position(&input);
        assert_eq!(p.h, 15);
        assert_eq!(p.d, 10);

        let h = find_heading(&input);
        assert_eq!(h.h, 15);
        assert_eq!(h.d, 60);
    }
}
