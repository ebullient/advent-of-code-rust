use crate::puzzle_input;
use std::collections::HashMap;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d08-input.txt");

    let grid = Grid::new(&input);

    println!("** Part 1 Final: {:?}", count_visible(&grid));
    println!("** Part 2 Final: {:?}", viewing_distance(&grid));
}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    data: HashMap<(usize, usize), i32>,
    x_max: usize,
    y_max: usize,
}
impl Grid {
    fn new(input: &[String]) -> Grid {
        let mut data: HashMap<(usize, usize), i32> = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, col) in row.trim().chars().enumerate() {
                data.insert((y, x), col.to_digit(10).unwrap() as i32);
            }
        }
        Grid {
            data,
            y_max: input.len(),
            x_max: input[0].len(),
        }
    }

    fn traverse<T>(
        &self,
        pt: (usize, usize),
        offset: (i32, i32),
        f: &dyn Fn(i32, Vec<i32>) -> T,
    ) -> T {
        let current_height = self.data[&pt];
        let mut heights = Vec::new();

        let mut new_y = pt.0 as i32 + offset.0;
        let mut new_x = pt.1 as i32 + offset.1;
        while new_y >= 0 && new_y < self.y_max as i32 && new_x >= 0 && new_x < self.x_max as i32 {
            heights.push(self.data[&(new_y as usize, new_x as usize)]);
            new_y += offset.0;
            new_x += offset.1;
        }

        f(current_height, heights)
    }

    fn left<T>(&self, pt: (usize, usize), f: &dyn Fn(i32, Vec<i32>) -> T) -> T {
        self.traverse(pt, (0, -1), f)
    }

    fn right<T>(&self, pt: (usize, usize), f: &dyn Fn(i32, Vec<i32>) -> T) -> T {
        self.traverse(pt, (0, 1), f)
    }

    fn down<T>(&self, pt: (usize, usize), f: &dyn Fn(i32, Vec<i32>) -> T) -> T {
        self.traverse(pt, (1, 0), f)
    }

    fn up<T>(&self, pt: (usize, usize), f: &dyn Fn(i32, Vec<i32>) -> T) -> T {
        self.traverse(pt, (-1, 0), f)
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..self.y_max {
            for x in 0..self.x_max {
                print!("{}", self.data.get(&(y, x)).unwrap());
            }
            println!();
        }
        println!();
    }
}

fn is_visible(current_height: i32, heights: Vec<i32>) -> bool {
    heights.iter().all(|h| *h < current_height)
}

fn count_visible(grid: &Grid) -> i32 {
    let mut result = 0;

    for y in 0..grid.y_max {
        for x in 0..grid.x_max {
            let pt = (y, x);
            if y == 0
                || x == 0
                || y == grid.y_max - 1
                || x == grid.x_max - 1
                || grid.left(pt, &is_visible)
                || grid.right(pt, &is_visible)
                || grid.up(pt, &is_visible)
                || grid.down(pt, &is_visible)
            {
                result += 1;
            }
        }
    }
    result
}

fn scenic_score(current_height: i32, heights: Vec<i32>) -> i32 {
    let mut i = 0;
    for height in heights.iter() {
        i += 1;
        if *height >= current_height {
            break;
        }
    }
    i
}

fn score_point(grid: &Grid, pt: (usize, usize)) -> i32 {
    grid.left(pt, &scenic_score)
        * grid.right(pt, &scenic_score)
        * grid.up(pt, &scenic_score)
        * grid.down(pt, &scenic_score)
}

fn viewing_distance(grid: &Grid) -> i32 {
    let mut max_scenic_score = 0;

    for y in 0..grid.y_max {
        for x in 0..grid.x_max {
            let pt = (y, x);
            if y == 0 || y == grid.y_max - 1 || x == 0 || x == grid.x_max - 1 {
                continue;
            }
            let score = score_point(grid, pt);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = r#"
        30373
        25512
        65332
        33549
        35390
        "#
        .split('\n')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();

        let grid = Grid::new(&input);
        grid.dump();
        assert_eq!(count_visible(&grid), 21);

        let mut pt = (1, 2);
        assert_eq!(grid.up(pt, &scenic_score), 1);
        assert_eq!(grid.left(pt, &scenic_score), 1);
        assert_eq!(grid.right(pt, &scenic_score), 2);
        assert_eq!(grid.down(pt, &scenic_score), 2);
        assert_eq!(score_point(&grid, pt), 4);

        pt = (3, 2);
        assert_eq!(grid.up(pt, &scenic_score), 2);
        assert_eq!(grid.left(pt, &scenic_score), 2);
        assert_eq!(grid.down(pt, &scenic_score), 1);
        assert_eq!(grid.right(pt, &scenic_score), 2);
        assert_eq!(score_point(&grid, pt), 8);

        assert_eq!(viewing_distance(&grid), 8);
    }
}
