use super::compute;

pub fn run() {
    if let Ok(input) = super::read_string("./input/2019-d05-input1.txt") {
        let mut codes: Vec<i32> = input.trim().split(',')
                                    .map(|x| x.parse::<i32>().unwrap())
                                    .collect();

                                    // What value is left at position 0 after the program halts?
        compute::run(&mut codes);
    }
}

