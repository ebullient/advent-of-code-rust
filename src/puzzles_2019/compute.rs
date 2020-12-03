
pub trait ProgramIO {
    fn add_input(&mut self, value: i64);
    fn take_input(&mut self) -> i64;

    fn write_output(&mut self, value: i64);
    fn read_output(&self) -> i64;
}

#[derive(Clone, Debug)]
pub struct DefaultProgramIO {
    input: Vec<i64>,
    output: i64
}

impl DefaultProgramIO {
    pub fn new(values: Vec<i64>) -> DefaultProgramIO {
        DefaultProgramIO {
            input: values,
            output: 0
        }
    }
}

impl ProgramIO for DefaultProgramIO {
    fn add_input(&mut self, value: i64) {
        self.input.push(value);
    }

    fn take_input(&mut self) -> i64 {
        self.input.remove(0)
    }

    fn write_output(&mut self, value: i64) {
        self.output = value;
    }

    fn read_output(&self) -> i64 {
        self.output
    }
}

#[derive(Debug,PartialEq)]
enum Mode {
    Position,  // 0
    Immediate, // 1, not allowed for writes
    Relative   // 2
}

fn to_mode(x: i32) -> Mode {
    match x {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => panic!("Unknown mode {}", x)
    }
}

// A tuple struct
struct Modes(Mode, Mode, Mode);

fn get_parameters(input: i64) -> (Modes, i32) {
    let mut x = input as i32;
    let op = x % 100;
    x /= 100;
    let c = x % 10;
    x /= 10;
    let b = x % 10;
    x /= 10;
    let a = x;

    (Modes(to_mode(a), to_mode(b), to_mode(c)), op)
}

fn read_value(codes: &mut Vec<i64>, mode: Mode, i: usize) -> i64 {
    if mode == Mode::Immediate {
        return codes[i]
    }

    let ix = codes[i] as usize;
    if ix > codes.len() {
        println!("Growing from {} to {}", codes.len(), ix);
        codes.resize(ix, 0);
    }

    codes[ix]
}

fn find_target(codes: &mut Vec<i64>, mode: Mode, i: usize) -> usize {
    if mode == Mode::Immediate {
        panic!("Attempting to retrieve target {} for writing in immediate mode", i);
    }

    codes[i] as usize
}

fn opcode_1(modes: Modes, i: usize, codes: &mut Vec<i64>) -> usize {
    // Add value from ix + value from iy, place in iz
    let x = read_value(codes, modes.2, i+1);
    let y = read_value(codes, modes.1, i+2);
    let iz = find_target(codes, modes.0, i+3) as usize;

    codes[iz] = x + y;

    i+4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_2(modes: Modes, i: usize, codes: &mut Vec<i64>) -> usize {
    // Multiply value from ix * value from iy, place in iz
    // Add value from ix + value from iy, place in iz
    let x = read_value(codes, modes.2, i+1);
    let y = read_value(codes, modes.1, i+2);
    let iz = find_target(codes, modes.0, i+3) as usize;

    codes[iz] = x * y;

    i+4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_3(modes: Modes, i: usize, codes: &mut Vec<i64>, io: &mut dyn ProgramIO) -> usize {
    // Opcode 3 takes a single integer as input and saves it to the 
    // position given by its only parameter. 

    let ix = find_target(codes, modes.2, i+1) as usize;
    codes[ix] = io.take_input();
 
    i+2 // advance 2: 1 opcode + 1 parameter
}

fn opcode_4(modes: Modes, i: usize, codes: &mut Vec<i64>, io: &mut dyn ProgramIO) -> usize {
    // Opcode 4 outputs the value of its only parameter. 
    // For example, the instruction 4,50 would output the value at address 50.
    let x = read_value(codes, modes.2, i+1);
    io.write_output(x);

    i+2 // advance 2: 1 opcode + 1 parameter
}

fn opcode_5(modes: Modes, i: usize, codes: &mut Vec<i64>) -> usize {
    // Opcode 5 is jump-if-true: if the first parameter is non-zero, 
    // it sets the instruction pointer to the value from the second parameter. 
    // Otherwise, it does nothing.
    let x = read_value(codes, modes.2, i+1);
    let y = read_value(codes, modes.1, i+2);

    if x != 0 {
        y as usize
    } else {
        i+3 // advance 3: 1 opcode + 2 parameters
    }
}

fn opcode_6(modes: Modes, i: usize, codes: &mut Vec<i64>) -> usize {
    // Opcode 6 is jump-if-false: if the first parameter is zero, 
    // it sets the instruction pointer to the value from the second parameter. 
    // Otherwise, it does nothing.
    let x = read_value(codes, modes.2, i+1);
    let y = read_value(codes, modes.1, i+2);

    if x == 0 {
        y as usize
    } else {
        i+3 // advance 3: 1 opcode + 2 parameters
    }
}

fn opcode_7(modes: Modes, i: usize, codes: &mut Vec<i64>) -> usize {
    // Opcode 7 is less than: if the first parameter is less than the second parameter, 
    // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    let x = read_value(codes, modes.2, i+1);
    let y = read_value(codes, modes.1, i+2);
    let iz = find_target(codes, modes.0, i+3) as usize;

    codes[iz] = 
        if x < y {
            1
        } else {
            0
        };

    i+4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_8(modes: Modes, i: usize, codes: &mut Vec<i64>) -> usize {
    // Opcode 8 is equals: if the first parameter is equal to the second parameter, 
    // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    let x = read_value(codes, modes.2, i+1);
    let y = read_value(codes, modes.1, i+2);
    let iz = find_target(codes, modes.0, i+3) as usize;

    codes[iz] = 
        if x == y {
            1
        } else {
            0
        };

    i+4 // advance 4: 1 opcode + 3 parameters
}

pub fn run(codes: &mut Vec<i64>, io: &mut dyn ProgramIO) {
    let mut i: usize = 0;
    //let mut 
    loop {
        let (modes, op) = get_parameters(codes[i]);
        //println!("{},{},{} {:?}", modes.0, modes.1, modes.2, op);
        match op {
            1 => { i = opcode_1(modes, i, codes) },
            2 => { i = opcode_2(modes, i, codes) },
            3 => { i = opcode_3(modes, i, codes, io) },
            4 => { i = opcode_4(modes, i, codes, io) },
            5 => { i = opcode_5(modes, i, codes) },
            6 => { i = opcode_6(modes, i, codes) },
            7 => { i = opcode_7(modes, i, codes) },
            8 => { i = opcode_8(modes, i, codes) },
            99 => { 
                break; 
            },
            _ => {
                println!("ERROR: {0} Unknown at index {1}", codes[i], i);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn intcode_program(input_ref: &str, io: &mut dyn ProgramIO) -> String {
        let mut codes: Vec<i64> = input_ref.split(',')
                                           .map(|x| x.trim().parse::<i64>().unwrap())
                                           .collect();
        run(&mut codes, io);
        codes.iter().join(",")
    }

    #[test]
    fn test_intcode_program() {
        let mut io = DefaultProgramIO::new(vec![0]);

        assert_eq!(intcode_program("1,0,0,0,99", &mut io), "2,0,0,0,99");
        assert_eq!(intcode_program("2,3,0,3,99", &mut io), "2,3,0,6,99");
        assert_eq!(intcode_program("2,4,4,5,99,0", &mut io), "2,4,4,5,99,9801");
        assert_eq!(intcode_program("1,1,1,4,99,5,6,0,99", &mut io), "30,1,1,4,2,5,6,0,99");
        assert_eq!(intcode_program("1,9,10,3,2,3,11,0,99,30,40,50", &mut io), "3500,9,10,70,2,3,11,0,99,30,40,50");
    }

    #[test]
    fn test_parameter_mode() {
        let (modes, op) = get_parameters(01245);
        assert_eq!(modes.0, Mode::Position);
        assert_eq!(modes.1, Mode::Immediate);
        assert_eq!(modes.2, Mode::Relative);
        assert_eq!(op, 45);
    }

    #[test]
    fn test_parameter_mode_2() {
        let (modes, op) = get_parameters(1002);
        assert_eq!(modes.0, Mode::Position);
        assert_eq!(modes.1, Mode::Immediate);
        assert_eq!(modes.2, Mode::Position);
        assert_eq!(op, 02);
    }

    #[test]
    fn test_equal_to() {
        let mut io = DefaultProgramIO::new(vec![0,8]);

        intcode_program("3,9,8,9,10,9,4,9,99,-1,8", &mut io);
        assert_eq!(io.read_output(), 0);

        intcode_program("3,9,8,9,10,9,4,9,99,-1,8", &mut io);
        assert_eq!(io.read_output(), 1);
    }

    #[test]
    fn test_less_than() {
        let mut io = DefaultProgramIO::new(vec![0,8]);

        intcode_program("3,9,7,9,10,9,4,9,99,-1,8", &mut io);
        assert_eq!(io.read_output(), 1);

        intcode_program("3,9,7,9,10,9,4,9,99,-1,8", &mut io);
        assert_eq!(io.read_output(), 0);
    }

    #[test]
    fn test_equal_to_immediate() {
        let mut io = DefaultProgramIO::new(vec![0,8]);

        intcode_program("3,3,1108,-1,8,3,4,3,99", &mut io);
        assert_eq!(io.read_output(), 0);

        intcode_program("3,3,1108,-1,8,3,4,3,99", &mut io);
        assert_eq!(io.read_output(), 1);
    }

    #[test]
    fn test_less_than_immediate() {
        let mut io = DefaultProgramIO::new(vec![0,8]);

        intcode_program("3,3,1107,-1,8,3,4,3,99", &mut io);
        assert_eq!(io.read_output(), 1);

        intcode_program("3,3,1107,-1,8,3,4,3,99", &mut io);
        assert_eq!(io.read_output(), 0);
    }

    #[test]
    fn test_jump_to_position() {
        let mut io = DefaultProgramIO::new(vec![0,8]);

        intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &mut io);
        assert_eq!(io.read_output(), 0);

        intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &mut io);
        assert_eq!(io.read_output(), 1);
    }

    #[test]
    fn test_jump_to_immediate() {
        let mut io = DefaultProgramIO::new(vec![0,8]);

        intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &mut io);
        assert_eq!(io.read_output(), 0);

        intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &mut io);
        assert_eq!(io.read_output(), 1);
    }

    #[test]
    fn test_larger_example() {
        let instr = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        let mut io = DefaultProgramIO::new(vec![0,8,18]);

        intcode_program(instr, &mut io);
        assert_eq!(io.read_output(), 999);

        intcode_program(instr, &mut io);
        assert_eq!(io.read_output(), 1000);

        intcode_program(instr, &mut io);
        assert_eq!(io.read_output(), 1001);
    }

    #[test]
    fn test_thruster_signal_1() {
        let instr = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";

        let mut a_io = DefaultProgramIO::new(vec![4, 0]);
        intcode_program(instr, &mut a_io);

        let mut b_io = DefaultProgramIO::new(vec![3, a_io.read_output()]);
        intcode_program(instr, &mut b_io);

        let mut c_io = DefaultProgramIO::new(vec![2, b_io.read_output()]);
        intcode_program(instr, &mut c_io);

        let mut d_io = DefaultProgramIO::new(vec![1, c_io.read_output()]);
        intcode_program(instr, &mut d_io);

        let mut e_io = DefaultProgramIO::new(vec![0, d_io.read_output()]);
        intcode_program(instr, &mut e_io);
        assert_eq!(e_io.read_output(), 43210);
    }

    #[test]
    fn test_thruster_signal_2() {
        let instr = "3,23,3,24,1002,24,10,24,1002,23,-1,23,
        101,5,23,23,1,24,23,23,4,23,99,0,0";

        let mut a_io = DefaultProgramIO::new(vec![0, 0]);
        intcode_program(instr, &mut a_io);

        let mut b_io = DefaultProgramIO::new(vec![1, a_io.read_output()]);
        intcode_program(instr, &mut b_io);

        let mut c_io = DefaultProgramIO::new(vec![2, b_io.read_output()]);
        intcode_program(instr, &mut c_io);

        let mut d_io = DefaultProgramIO::new(vec![3, c_io.read_output()]);
        intcode_program(instr, &mut d_io);

        let mut e_io = DefaultProgramIO::new(vec![4, d_io.read_output()]);
        intcode_program(instr, &mut e_io);
        assert_eq!(e_io.read_output(), 54321);
    }

    #[test]
    fn test_thruster_signal_3() {
        let instr = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

        let mut a_io = DefaultProgramIO::new(vec![1, 0]);
        intcode_program(instr, &mut a_io);

        let mut b_io = DefaultProgramIO::new(vec![0, a_io.read_output()]);
        intcode_program(instr, &mut b_io);

        let mut c_io = DefaultProgramIO::new(vec![4, b_io.read_output()]);
        intcode_program(instr, &mut c_io);

        let mut d_io = DefaultProgramIO::new(vec![3, c_io.read_output()]);
        intcode_program(instr, &mut d_io);

        let mut e_io = DefaultProgramIO::new(vec![2, d_io.read_output()]);
        intcode_program(instr, &mut e_io);
        assert_eq!(e_io.read_output(), 65210);
    }
}