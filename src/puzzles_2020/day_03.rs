use crate::puzzle_input;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d03-input1.txt");
    let mut grid = Grid::new(input);

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
    cols: usize,
    y_max: usize,
}
impl Grid {
    pub fn new(values: Vec<String>) -> Grid {
        let mut data: Vec<Vec<char>> = Vec::with_capacity(values.len());
        for row in values.iter() {
            data.push(row.chars().collect());
        }

        Grid {
            data: data,
            cols: values[0].len(),
            y_max: values.len() - 1,
        }
    }

    pub fn traverse_badly(&mut self, i: usize, j: usize) -> i64 {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;
        while y < self.y_max {
            x += i;
            y += j;
            if self.data[y][x % self.cols] == '#' {
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
        let input = "..##.......
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

        let mut grid = Grid::new(
            input
                .split_whitespace()
                .map(|x| x.trim().to_string())
                .collect(),
        );

        assert_eq!(grid.traverse_badly(1, 1), 2);
        assert_eq!(grid.traverse_badly(3, 1), 7);
        assert_eq!(grid.traverse_badly(5, 1), 3);
        assert_eq!(grid.traverse_badly(7, 1), 4);
        assert_eq!(grid.traverse_badly(1, 2), 2);
    }
}
