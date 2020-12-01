use super::compute;
use crate::puzzle_input;

pub fn run() {
    if let Ok(input) = puzzle_input::read_string("./input/2019-d05-input1.txt") {
        let mut codes: Vec<i32> = input.trim().split(',')
                                    .map(|x| x.trim().parse::<i32>().unwrap())
                                    .collect();

        let mut max_signal = 0;
        let mut phases = vec![0, 1, 2, 3, 4];
        let mut io = compute::ProgramIO::new(vec![1]);

        println!("** Part 1 Final: {:?}", phases);
    }
}
