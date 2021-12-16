use crate::puzzle_input;
use std::collections::HashSet;
use std::f64::consts::{FRAC_PI_2, PI};
use std::ops;

// I got stuck on this one.
// https://github.com/prscoelho/aoc2019/blob/master/src/aoc10/mod.rs

pub fn run() {
    let input = parse_input(&puzzle_input::read_all_lines("./input/2019-d10-input1.txt"));

    println!("** Part 1 Final: {:?}", find_most_asteroids(&input));
    let blast_seq = blast_asteroids(&input, &Point { x: 20, y: 19 });
    println!("** Part 2 Final: {:?}", get_happy(&blast_seq[199]));
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn angle(&self) -> f64 {
        // y.atan2(x)
        let result = (self.y as f64).atan2(self.x as f64);
        if result < 0.0 {
            result + 2.0 * PI
        } else {
            result
        }
    }

    fn mag(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}
impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        // because 0,0 is top left instead of bottom left,
        // reverse the y values. seems hacky: any better way to do this?
        Point {
            x: self.x - rhs.x,
            y: rhs.y - self.y,
        }
    }
}

fn find_asteroids(data: &Vec<Point>, src: &Point) -> usize {
    let mut set = HashSet::new();
    data.iter()
        .filter(|p| *p != src)
        .map(|p| {
            let diff = *p - *src;
            diff.angle()
        })
        .filter(|x| set.insert(x.to_string()))
        .count()
}

fn blast_asteroids(data: &Vec<Point>, src: &Point) -> Vec<Point> {
    let mut list: Vec<(f64, i32, Point)> = data
        .iter()
        .filter(|p| *p != src)
        .map(|p| {
            let diff = *p - *src;
            (diff.angle(), diff.mag(), *p)
        })
        .collect();

    list.sort_by(|a, b| {
        // sort by reverse angle (clockwise), and then magnitude
        a.0.partial_cmp(&b.0).unwrap().reverse().then(a.1.cmp(&b.1))
    });

    let n = list.len();
    let mut blast_order = Vec::with_capacity(n);

    for (idx, v) in list.iter().enumerate() {
        if v.0 <= FRAC_PI_2 {
            blast_order.push(list.remove(idx));
            break;
        }
    }
    if let None = blast_order.last() {
        blast_order.push(list.remove(0));
    }

    'outer: for _ in 1..n {
        for (idx, v) in list.iter().enumerate() {
            if v.0 < blast_order.last().unwrap().0 {
                blast_order.push(list.remove(idx));
                continue 'outer;
            }
        }
        // reached end of list without removing, remove first element
        blast_order.push(list.remove(0));
        println!("{:?} ", list.len());
    }

    blast_order.iter().map(|x| x.2).collect()
}

fn find_most_asteroids(data: &Vec<Point>) -> i32 {
    let mut max = 0;
    let mut p = &data[0];
    for point in data {
        let n = find_asteroids(data, point);
        if n > max {
            max = n;
            p = point;
        }
    }
    println!("{:?} can see the most ({:?}) asteroids", p, max);
    max as i32
}

fn get_happy(p: &Point) -> i32 {
    p.x as i32 * 100 + p.y as i32
}

fn parse_input(input: &Vec<String>) -> Vec<Point> {
    let mut data: Vec<Point> = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                data.push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asteroid_field_1() {
        let input = ".#..#
        .....
        #####
        ....#
        ...##"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        let data = parse_input(&input);
        assert_eq!(find_asteroids(&data, &Point { x: 1, y: 0 }), 7);
        assert_eq!(find_asteroids(&data, &Point { x: 3, y: 4 }), 8);
        assert_eq!(find_asteroids(&data, &Point { x: 0, y: 2 }), 6);

        assert_eq!(find_most_asteroids(&data), 8);
    }

    #[test]
    fn test_asteroid_field_2() {
        let input = "......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        let data = parse_input(&input);
        assert_eq!(find_most_asteroids(&data), 33);
    }

    #[test]
    fn test_asteroid_field_3() {
        let input = "#.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###."
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        let data = parse_input(&input);
        assert_eq!(find_most_asteroids(&data), 35);
    }

    #[test]
    fn test_asteroid_field_4() {
        let input = ".#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#.."
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        let data = parse_input(&input);
        assert_eq!(find_most_asteroids(&data), 41);
    }

    #[test]
    fn test_asteroid_field_5() {
        let input = ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        let data = parse_input(&input);
        assert_eq!(find_most_asteroids(&data), 210);

        let blast_seq = blast_asteroids(&data, &Point { x: 11, y: 13 });
        assert_eq!(&blast_seq[0], &Point { x: 11, y: 12 });
        assert_eq!(&blast_seq[1], &Point { x: 12, y: 1 });
        assert_eq!(&blast_seq[2], &Point { x: 12, y: 2 });
        assert_eq!(&blast_seq[9], &Point { x: 12, y: 8 });
        assert_eq!(&blast_seq[19], &Point { x: 16, y: 0 });
        assert_eq!(&blast_seq[49], &Point { x: 16, y: 9 });
        assert_eq!(&blast_seq[99], &Point { x: 10, y: 16 });
        assert_eq!(&blast_seq[198], &Point { x: 9, y: 6 });
        assert_eq!(&blast_seq[199], &Point { x: 8, y: 2 });
        assert_eq!(&blast_seq[200], &Point { x: 10, y: 9 });
        assert_eq!(&blast_seq[298], &Point { x: 11, y: 1 });
        assert_eq!(get_happy(&blast_seq[199]), 802);
    }
}
