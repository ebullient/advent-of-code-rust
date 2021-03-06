use super::compute;
use crate::puzzle_input;

pub fn run() {
    let input = puzzle_input::read_string("./input/2019-d09-input1.txt");
    let mut codes: Vec<i64> = input
        .trim()
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let mut io = compute::DefaultProgramIO::new(vec![1]); // test mode
    compute::run(&mut codes, &mut io);
    println!("** Part 1 Final: {:?}", io);

    let input = puzzle_input::read_string("./input/2019-d09-input1.txt");
    let mut codes: Vec<i64> = input
        .trim()
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let mut io = compute::DefaultProgramIO::new(vec![2]); // sensor boost
    compute::run(&mut codes, &mut io);
    println!("** Part 2 Final: {:?}", io);
}
