use crate::puzzle_input;
use std::cmp;
use std::collections::HashSet;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d13-input.txt");
    let mut paper = Paper::new(&input);

    let (axis, index) = paper.next().unwrap();
    paper.fold(axis, index);

    println!("** Part 1 Final: {:?}", paper.dots.len());
    while let Some((axis, index)) = paper.next() {
        println!("Folding {:?}={:?}", axis, index);
        paper.fold(axis, index);
    }
    paper.dump();
    println!("** Part 2 Final: {:?}", 0);
}

#[derive(Clone, Debug)]
struct Paper {
    dots: HashSet<(usize, usize)>,
    folds: Vec<(char, usize)>,
    state: usize,
    height: usize,
    width: usize,
}

impl Paper {
    fn new(input: &Vec<String>) -> Paper {
        let mut dots: HashSet<(usize, usize)> = HashSet::new();
        let mut folds: Vec<(char, usize)> = vec![];
        let mut height: usize = 0;
        let mut width: usize = 0;

        for line in input {
            if line.contains(",") {
                let mut split = line.split(",");
                let x = split.next().unwrap().parse::<usize>().unwrap();
                let y = split.next().unwrap().parse::<usize>().unwrap();
                width = cmp::max(x, width);
                height = cmp::max(y, height);
                dots.insert((y, x));
            } else if line.contains("=") {
                let fold = line.get(11..).unwrap();
                let mut split = fold.split("=");
                let axis = split.next().unwrap().chars().next().unwrap();
                let index = split.next().unwrap().parse::<usize>().unwrap();
                folds.push((axis, index));
            }
        }

        Paper {
            dots: dots,
            folds: folds,
            state: 0,
            height: height + 1,
            width: width + 1,
        }
    }

    fn next(&mut self) -> Option<(char, usize)> {
        if let Some((axis, index)) = self.folds.get(self.state) {
            self.state += 1;
            return Some((*axis, *index));
        }
        None
    }

    fn fold(&mut self, axis: char, index: usize) {
        let all: HashSet<_> = self.dots.iter().cloned().collect();
        self.dots.retain(|pt| {
            if axis == 'x' {
                pt.1 < index
            } else {
                pt.0 < index
            }
        });
        let diff: HashSet<_> = all.difference(&self.dots).cloned().collect();

        for pt in &diff {
            if axis == 'x' {
                self.dots.insert((pt.0, index - (pt.1 - index)));
                self.width = index;
            } else {
                self.dots.insert((index - (pt.0 - index), pt.1));
                self.height = index;
            }
        }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(_) = self.dots.get(&(y, x)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5",
        );

        let mut paper = Paper::new(&input);
        paper.dump();

        while let Some((axis, index)) = paper.next() {
            paper.fold(axis, index);
            paper.dump();
        }

        assert_eq!(16, paper.dots.len());
    }
}
