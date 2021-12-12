use crate::puzzle_input;
use std::slice::Iter;
use std::collections::HashMap;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d04-input.txt");

    let (draw, mut boards) = parse_input(&input);
    let n = boards.len();
    let (last, winner) = play(&draw, &mut boards, n);
    let (_, total) = finish(last, &winner);
    println!("** Part 1 Final: {:?}", total);

    boards.iter_mut().for_each(|b| b.clear());

    let (last2, winner2) = play_through(&draw, &mut boards, n);
    let (_, total2) = finish(last2, &winner2);
    println!("** Part 2 Final: {:?}", total2);
}

#[derive(Clone, Debug)]
struct Board {
    data: HashMap<i32, (usize, usize)>,
    marked: Vec<i32>,
    rows: [usize; 5],
    cols: [usize; 5],
    bingo: bool
}
impl Board {
    pub fn new(iter: &mut Iter<String>) -> Board {
        let mut data: HashMap<i32, (usize, usize)> = HashMap::new();
        for i in 0..5 {
            let row: Vec<i32> = iter.next().unwrap()
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
            for (j, value) in row.iter().enumerate() {
                data.insert(*value, (i, j));
            }
        }

        Board {
            data: data,
            marked: Vec::new(),
            rows: [0; 5],
            cols: [0; 5],
            bingo: false
        }
    }

    pub fn mark(&mut self, i: i32) -> bool {
        if self.bingo {
            return false;
        }
        if let Some(place) = self.data.get(&i) {
            self.marked.push(i);
            self.cols[place.0] += 1;
            self.rows[place.1] += 1;
            if self.cols[place.0] == 5 || self.rows[place.1] == 5 {
                self.bingo = true;
                return true;
            }
        }
        false
    }

    pub fn clear(&mut self) {
        self.marked.clear();
        self.cols.iter_mut().for_each(|m| *m = 0);
        self.rows.iter_mut().for_each(|m| *m = 0);
        self.bingo = false;
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<i32>, Vec<Board>) {
    let mut boards: Vec<Board> = Vec::new();
    let mut iter = input.iter();
    let draw: Vec<i32> = iter.next().unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    while let Some(_) = iter.next() { // blank line (break if None)
        boards.push(Board::new(&mut iter));
    }
    (draw, boards)
}

fn play<'a>(draw: &'a Vec<i32>, boards: &'a mut Vec<Board>, n: usize) -> (i32, &'a Board) {
    for d in draw {
        for i in 0..n {
            let bingo = boards[i].mark(*d);
            if bingo {
                return (*d, &boards[i]);
            }
        }
    }
    panic!("No winner?! We did it wrong.");
}

fn play_through<'a>(draw: &'a Vec<i32>, boards: &'a mut Vec<Board>, n: usize) -> (i32, &'a Board) {
    let mut results: Vec<(i32, usize)> = Vec::new();

    for d in draw {
        for i in 0..n {
            let bingo = boards[i].mark(*d);
            if bingo {
                results.push((*d, i));
                //println!("bingo with draw: {:?}, i: {:?} --> r: {:?}", d, i, results);
            }
        }
    }

    let last = results.pop().unwrap();
    (last.0, &boards[last.1])
}

// Start by finding the sum of all unmarked numbers on that board;
// Then, multiply that sum by the number that was last called.
fn finish<'a>(draw: i32, board: &'a Board) -> (i32, i32) {
    let unmarked: i32 = board.data.keys()
        .map(|x| *x)
        .filter(|x| !board.marked.contains(x))
        .sum();
    (unmarked, unmarked * draw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11 0
         8  2 23  4 24
        21 9 14 16 7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24 4
        14 21 16 12 6

        14 21 17 24 4
        10 16 15  9 19
        18 8 23 26 20
        22 11 13  6  5
         2  0 12  3  7");

        let (draw, mut boards) = parse_input(&input);
        assert_eq!(3, boards.len());
        assert_eq!(27, draw.len());

        let n = boards.len();

        let (last, winner) = play(&draw, &mut boards, n);
        let (sum, total) = finish(last, &winner);
        assert_eq!(sum, 188);
        assert_eq!(total, 4512);

        boards.iter_mut().for_each(|b| b.clear());

        let (last2, winner2) = play_through(&draw, &mut boards, n);
        let (sum2, total2) = finish(last2, &winner2);
        assert_eq!(sum2, 148);
        assert_eq!(total2, 1924);
    }
}
