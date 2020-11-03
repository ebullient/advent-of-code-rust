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
                    println!("{0} and {1} make {2}", noun, verb, result);
                    println!("** Part 2 Final: {0}", 100 * noun + verb);
                    break 'outer;
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn intcode_program(input_ref: &str) -> String {
        let mut codes: Vec<i32> = input_ref.split(',')
                                           .map(|x| x.parse::<i32>().unwrap())
                                           .collect();
        compute::run(&mut codes);
        codes.iter().join(",")
    }

    #[test]
    fn test_intcode_program() {
        assert_eq!(intcode_program("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(intcode_program("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(intcode_program("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(intcode_program("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
        assert_eq!(intcode_program("1,9,10,3,2,3,11,0,99,30,40,50"), "3500,9,10,70,2,3,11,0,99,30,40,50");
    }
}