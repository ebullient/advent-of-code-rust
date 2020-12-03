use crate::puzzle_input;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d03-input1.txt");
    let mut grid = Grid::new(7, input);
    let t1 = grid.traverse_badly(3, 1);
    println!("** Part 1 Final: {:?}", t1);

    let t2 = grid.traverse_badly(1, 1);
    let t3 = grid.traverse_badly(5, 1);
    let t4 = grid.traverse_badly(7, 1);
    let t5 = grid.traverse_badly(1, 2);
    println!("** Part 2 Final: {:?}", t1 * t2 * t3 * t4 * t5);
}

#[derive(Clone, Debug)]
struct Grid {
    data: Vec<Vec<char>>,
    debug: bool
}
impl Grid {
    pub fn new(x_factor: usize, values: Vec<String>) -> Grid {
        let mut data: Vec<Vec<char>> = Vec::with_capacity(values.len());
        let y = values.len();
        let x = y * x_factor;
        let x1 = values[0].chars().count();
        let n = if x % x1 != 0 {
            (x / x1) + 1
        } else {
            x / x1
        };
        println!("{:?} rows, {:?}. Data {:?} is {:?} long, must repeat {:?} times",
            y,
            x,
            values[0],
            values[0].len(),
            n
        );

        for row in values.iter() {
            let mut col: Vec<char> = Vec::with_capacity(x);
            for _ in 0..n {
                col.extend(row.chars());
            }
            data.push(col);
        }

        Grid {
            data: data,
            debug: false
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        for (y, row) in self.data.iter().enumerate() {
            print!("{}", format!("{:04}: ", y));
            row.iter().for_each(|x| print!("{}", x));
            println!();
        }
    }

    pub fn traverse_badly(&mut self, i: usize, j: usize) -> i64 {
        let mut x = 0;
        let mut y = 0;
        let max = self.data.len() - 1;
        let mut trees = 0;
        while y < max {
            x += i;
            y += j;
            if self.data[y][x] == '#' {
                trees += 1;
            }
        }

        println!("Found {} trees using right {}, down {}", trees, i, j);
        trees
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_map() {
        let input =
        "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        let mut grid = Grid::new(7, input.split_whitespace()
            .map(|x| x.trim().to_string())
            .collect());

        assert_eq!(grid.traverse_badly(1, 1), 2);
        assert_eq!(grid.traverse_badly(3, 1), 7);
        assert_eq!(grid.traverse_badly(5, 1), 3);
        assert_eq!(grid.traverse_badly(7, 1), 4);
        assert_eq!(grid.traverse_badly(1, 2), 2);
    }
}
