use crate::puzzle_input;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::ops::RangeInclusive;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d11-input1.txt");
    let grid = Grid::new(&input);

    println!("** Part 1 Final: {:?}", count_occupied_seats(&grid, true));
    println!("** Part 2 Final: {:?}", count_occupied_seats(&grid, false));
}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    data: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &[String]) -> Grid {
        let mut data: HashMap<(usize, usize), char> = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, col) in row.trim().chars().enumerate() {
                data.insert((y, x), col);
            }
        }
        Grid {
            data,
            height: input.len(),
            width: input[0].len(),
        }
    }

    fn get(&self, pt: (usize, usize)) -> char {
        *self.data.get(&pt).unwrap()
    }

    fn keys(&self) -> Keys<(usize, usize), char> {
        self.data.keys()
    }

    fn put(&mut self, pt: (usize, usize), seat: char) {
        self.data.insert(pt, seat);
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data.get(&(y, x)).unwrap());
            }
            println!();
        }
        println!();
    }
}

fn count_occupied_seats(grid: &Grid, boxed: bool) -> i32 {
    let mut prev = grid.clone();
    let mut last = 0;
    loop {
        let (next, occupied) = if boxed {
            box_step(&prev)
        } else {
            visible_step(&prev)
        };
        if occupied == last {
            // steady state
            return occupied;
        }
        last = occupied;
        prev = next;
    }
}

fn box_step(start: &Grid) -> (Grid, i32) {
    let mut next = start.clone();
    let mut occupied = 0;

    for point in start.keys() {
        let seat = start.get(*point);
        let box_pts = get_box(start, *point);
        let result = match seat {
            'L' => check_box_around(start, &box_pts, true),
            '#' => check_box_around(start, &box_pts, false),
            _ => seat,
        };
        if result == '#' {
            occupied += 1;
        }
        next.put(*point, result);
    }
    (next, occupied)
}

fn box_range(i: usize, max: usize) -> RangeInclusive<usize> {
    std::ops::RangeInclusive::new(
        if i == 0 { 0 } else { i - 1 },
        if i + 1 == max { i } else { i + 1 },
    )
}

fn get_box(grid: &Grid, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut pts = Vec::with_capacity(8);
    for y1 in box_range(point.0, grid.height) {
        for x1 in box_range(point.1, grid.width) {
            if y1 == point.0 && x1 == point.1 {
                continue;
            }
            pts.push((y1, x1));
        }
    }
    pts
}

fn check_box_around(start: &Grid, pts: &[(usize, usize)], is_empty: bool) -> char {
    // If a seat is empty (L) and there are no occupied seats adjacent to it,
    // the seat becomes occupied.
    // If a seat is occupied (#) and four or more seats adjacent to it are also occupied,
    // the seat becomes empty.
    let mut occupied = 0;
    for pt in pts {
        if start.get(*pt) == '#' {
            occupied += 1;
            if occupied == 4 || is_empty {
                return 'L';
            }
        }
    }
    '#'
}

fn visible_step(start: &Grid) -> (Grid, i32) {
    let mut next = start.clone();
    let mut occupied = 0;

    // it now takes five or more visible occupied seats for an occupied seat to become empty
    // empty seats that see no occupied seats become occupied,
    // seats matching no rule don't change, and floor never changes.
    for point in start.keys() {
        let seat = start.get(*point);
        if seat == '.' {
            continue;
        }
        // First visible empty/occupied seat in each direction
        let visible_seats = visible_seats(start, *point);
        let result = match seat {
            'L' => {
                if visible_seats == 0 {
                    '#'
                } else {
                    'L'
                }
            }
            '#' => {
                if visible_seats >= 5 {
                    'L'
                } else {
                    '#'
                }
            }
            _ => seat,
        };
        if result == '#' {
            occupied += 1;
        }
        next.put(*point, result);
    }
    (next, occupied)
}

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn visible_seats(grid: &Grid, point: (usize, usize)) -> i32 {
    let mut occupied = 0;
    let y = point.0 as i32;
    let x = point.1 as i32;

    for offset in OFFSETS.iter() {
        let mut new_y = y + offset.0;
        let mut new_x = x + offset.1;
        while new_y >= 0 && new_y < grid.height as i32 && new_x >= 0 && new_x < grid.width as i32 {
            let seat = grid.get((new_y as usize, new_x as usize));
            if seat != '.' {
                if seat == '#' {
                    occupied += 1;
                }
                break;
            }
            new_y += offset.0;
            new_x += offset.1;
        }
    }
    occupied
}

#[allow(dead_code)]
fn dump(data: &[Vec<char>]) {
    for y in data.iter() {
        for x in y.iter() {
            print!("{}", x);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL"
            .split('\n')
            .map(|x| x.to_string())
            .collect();

        let grid = Grid::new(&input);

        assert_eq!(count_occupied_seats(&grid, true), 37);
        assert_eq!(count_occupied_seats(&grid, false), 26);
    }
}
