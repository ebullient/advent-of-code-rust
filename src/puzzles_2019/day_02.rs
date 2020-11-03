use super::compute;

pub fn run() {
    part_1();
    part_2();
}

fn run_program(input: &str, noun: i32, verb: i32) -> i32 {
    let trim = input.trim();
    // To do this, before running the program, replace position 1 with the value 12 
    // and replace position 2 with the value 2. 
    let mut codes: Vec<i32> = trim.split(',')
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect();
    codes[1] = noun;
    codes[2] = verb;
    // What value is left at position 0 after the program halts?
    compute::run(&mut codes);
    codes[0]
}

fn part_1() {
    // Get us back to where we were when things caught fire
    if let Ok(input) = super::read_string("./input/2019-d02-input1.txt") {
        let trim = input.trim();
        // To do this, before running the program, replace position 1 with the value 12 
        // and replace position 2 with the value 2. 
        // What value is left at position 0 after the program halts?
        let result: i32 = run_program(trim, 12, 2);
        println!("** Part 1 Final: {:?}", result);
    }
}

fn part_2() {
    if let Ok(input) = super::read_string("./input/2019-d02-input1.txt") {
        let trim = input.trim();
        // What pair of inputs produces output 19690720
        let target: i32 = 19690720;
        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                let result: i32 = run_program(trim, noun, verb);
                if result == target {
                    println!("** Part 2 Final: {0}", 100 * noun + verb);
                    break 'outer;
                }
            }
        }
    }
}
