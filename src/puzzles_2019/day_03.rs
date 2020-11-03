use std::cmp;
use std::hash;
use std::collections::HashSet;

pub fn run() {
    let input = super::read_all_lines("./input/2019-d03-input1.txt");
    let path1 = compute_path(&input[0]);
    let path2 = compute_path(&input[1]);

    let nearest = nearest_intersection(&path1, &path2);
    println!("** Part 1 Final: {0}", nearest);

    let shortest = shortest_path(&path1, &path2);
    println!("** Part 2 Final: {0}", shortest);
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    steps: i32
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

fn insert(point: Point, path: &mut HashSet<Point>) {
    if ! path.insert(point) {
        println!("Collision on {:?}. Original is {:?}", point, path.get(&point));
    }
}

fn next(last: Point, path: &mut HashSet<Point>, direction: char, n: i32) -> Point {
    let mut steps = last.steps;
    let mut pt = last;
    for m in 1..=n {
        steps += 1;
        match direction {
            'U' => pt = Point { x: last.x, y: last.y + m, steps: steps },
            'D' => pt = Point { x: last.x, y: last.y - m, steps: steps },
            'L' => pt = Point { x: last.x - m, y: last.y, steps: steps },
            'R' => pt = Point { x: last.x + m, y: last.y, steps: steps },
            _ => {
                panic!("Bad direction {}", direction);
            }
        }
        insert(pt, path);
    }
    pt
}

fn compute_path(input: &str) -> HashSet<Point> {
    let mut path: HashSet<Point> = HashSet::new();
    let mut last = Point { x: 0, y: 0, steps: 0 };

    for elem in input.split(',') {
        let direction = elem.chars().next().unwrap();
        let n = elem.trim_start_matches(|c| c == 'R' || c == 'L' || c == 'U' || c == 'D').parse::<i32>().unwrap();
        last = next(last, &mut path, direction, n);
    }
    println!("Final path has {:?} elements, last is {:?}", path.len(), last);
    path
}

fn nearest_intersection(path1: &HashSet<Point>, path2: &HashSet<Point>) -> i32 {
    let mut nearest = std::i32::MAX;
    for point in path1.intersection(&path2) {
        let md = point.x.abs() + point.y.abs();
        println!("{:?} -> {:?}", point, md);
        if md < nearest {
            nearest = md;
        }
    }
    nearest
}

fn shortest_path(path1: &HashSet<Point>, path2: &HashSet<Point>) -> i32 {
    let mut shortest = std::i32::MAX;

    for x in path1.intersection(&path2) {
        let point1 = path1.get(x).unwrap();
        let point2 = path2.get(x).unwrap();
        let sum = point1.steps + point2.steps;
        println!("{:?} <-> {:?} ==> {:?}", point1, point2, sum);
        if sum < shortest {
            shortest = sum;
        }
    }
    shortest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_md_1() {
        let path1 = compute_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = compute_path("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(nearest_intersection(&path1, &path2), 159);
    }
    #[test]
    fn test_compute_md_2() {
        let path3 = compute_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path4 = compute_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(nearest_intersection(&path3, &path4), 135);
    }

    #[test]
    fn test_compute_combined_path_1() {
        let path1 = compute_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = compute_path("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(shortest_path(&path1, &path2), 610);
    }

    #[test]
    fn test_compute_combined_path_2() {
        let path3 = compute_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path4 = compute_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(shortest_path(&path3, &path4), 410);
    }
}