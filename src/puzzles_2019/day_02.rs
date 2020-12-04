use super::compute;
use crate::puzzle_input;

pub fn run() {
    part_1();
    part_2();
}

fn run_program(input: &str, noun: i64, verb: i64) -> i64 {
    let mut io = compute::DefaultProgramIO::new(vec![0]);

    let trim = input.trim();
    // To do this, before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    let mut codes: Vec<i64> = trim.split(',')
                                .map(|x| x.parse::<i64>().unwrap())
                                .collect();
    codes[1] = noun;
    codes[2] = verb;
    // What value is left at position 0 after the program halts?
    compute::run(&mut codes, &mut io);
    codes[0]
}

fn part_1() {
    // Get us back to where we were when things caught fire
    let input = puzzle_input::read_string("./input/2019-d02-input1.txt");
    let trim = input.trim();
    // To do this, before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    // What value is left at position 0 after the program halts?
    let result = run_program(trim, 12, 2);
    println!("** Part 1 Final: {:?}", result);
}

fn part_2() {
    let input = puzzle_input::read_string("./input/2019-d02-input1.txt");
    let trim = input.trim();
    // What pair of inputs produces output 19690720
    let target = 19690720;
    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            let result = run_program(trim, noun, verb);
            if result == target {
                println!("** Part 2 Final: {0}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
