use super::compute;
use crate::puzzle_input;

pub fn run() {
    if let Ok(input) = puzzle_input::read_string("./input/2019-d05-input1.txt") {
        let mut codes: Vec<i32> = input.trim().split(',')
                                    .map(|x| x.trim().parse::<i32>().unwrap())
                                    .collect();

        let mut io = compute::DefaultProgramIO::new(vec![1]);
        compute::run(&mut codes, &mut io);
        println!("** Part 1 Final: {:?}", io);
    }

    if let Ok(input) = puzzle_input::read_string("./input/2019-d05-input1.txt") {
        let mut codes: Vec<i32> = input.trim().split(',')
                                    .map(|x| x.trim().parse::<i32>().unwrap())
                                    .collect();

        let mut io = compute::DefaultProgramIO::new(vec![5]);
        compute::run(&mut codes, &mut io);
        println!("** Part 2 Final: {:?}", io);
    }
}

