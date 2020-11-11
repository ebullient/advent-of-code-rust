use super::compute;

pub fn run() {
    if let Ok(input) = super::read_string("./input/2019-d05-input1.txt") {
        let mut codes: Vec<i32> = input.trim().split(',')
                                    .map(|x| x.trim().parse::<i32>().unwrap())
                                    .collect();

        let mut io = compute::ProgramIO{input: 1, output: 0};
        compute::run(&mut codes, &mut io);
        println!("** Part 1 Final: {:?}", io);
    }

    if let Ok(input) = super::read_string("./input/2019-d05-input1.txt") {
        let mut codes: Vec<i32> = input.trim().split(',')
                                    .map(|x| x.trim().parse::<i32>().unwrap())
                                    .collect();

        let mut io = compute::ProgramIO{input: 5, output: 0};
        compute::run(&mut codes, &mut io);
        println!("** Part 2 Final: {:?}", io);
    }
}

