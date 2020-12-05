use std::collections::HashMap;

pub trait ProgramIO {
    fn add_input(&mut self, value: i64);
    fn take_input(&mut self) -> i64;

    fn write_output(&mut self, value: i64);
    fn read_output(&self) -> i64;
}

#[derive(Clone, Debug)]
pub struct DefaultProgramIO {
    input: Vec<i64>,
    output: i64,
    all: Vec<i64>,
}

impl DefaultProgramIO {
    pub fn new(values: Vec<i64>) -> DefaultProgramIO {
        DefaultProgramIO {
            input: values,
            output: 0,
            all: Vec::new(),
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
        self.all.push(value);
    }

    fn read_output(&self) -> i64 {
        self.output
    }
}

#[derive(Debug, PartialEq)]
enum Mode {
    Position,  // 0
    Immediate, // 1, not allowed for writes
    Relative,  // 2
}

fn to_mode(x: i32) -> Mode {
    match x {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => panic!("Unknown mode {}", x),
    }
}

// A tuple struct
struct Modes(Mode, Mode, Mode);

struct Computer<'a> {
    codes: &'a mut Vec<i64>,
    io: &'a mut dyn ProgramIO,
    extents: HashMap<usize, i64>,
    relative_base: usize,
}

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

// In Position mode (0), the parameter to be interpreted as a position -
//    if the parameter is 50, its value is the value stored at address 50 in memory
// In Immediate mode (1), a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.
// In Relative mode (2), the parameter is interpreted as a relative position:
//    The address a relative mode parameter refers to is itself plus the current relative base.
fn get_index(computer: &mut Computer, mode: Mode, i: usize) -> usize {
    if mode == Mode::Position {
        //println!("- {:?} mode from {}: {:?}", mode, i, computer.codes[i] as usize);
        computer.codes[i] as usize
    } else {
        // keep as i64 to allow negative numbers for addition
        let x: i64 = computer.codes[i] + computer.relative_base as i64;
        // println!("- {:?} mode from {} with relative base {:?}: {:?} --> {:?}", mode, i,
        //     computer.relative_base, computer.codes[i], x);
        x as usize
    }
}

fn read(computer: &mut Computer, mode: Mode, i: usize) -> i64 {
    if mode == Mode::Immediate {
        //println!("- {:?} mode from {}: {}", mode, i, computer.codes[i]);
        return computer.codes[i];
    }

    let ix = get_index(computer, mode, i);

    if ix >= computer.codes.len() {
        //println!("<-- {:?}:{:?}", ix, computer.extents.get(&ix));
        match computer.extents.get(&ix) {
            Some(value) => *value,
            None => 0,
        }
    } else {
        computer.codes[ix]
    }
}

fn store(computer: &mut Computer, mode: Mode, i: usize, value: i64) {
    if mode == Mode::Immediate {
        panic!(
            "Attempting to retrieve target {} for writing in immediate mode",
            i
        );
    }
    let ix = get_index(computer, mode, i);
    if ix >= computer.codes.len() {
        //println!("--> {:?}:{:?} .. {}", ix, computer.extents.get(&ix), value);
        computer.extents.insert(ix, value);
    } else {
        computer.codes[ix] = value;
    }
}

fn opcode_1(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Add value from ix + value from iy, place in iz
    let x = read(computer, modes.2, i + 1);
    let y = read(computer, modes.1, i + 2);

    store(computer, modes.0, i + 3, x + y);

    i + 4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_2(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Multiply value from ix * value from iy, place in iz
    // Add value from ix + value from iy, place in iz
    let x = read(computer, modes.2, i + 1);
    let y = read(computer, modes.1, i + 2);

    store(computer, modes.0, i + 3, x * y);

    i + 4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_3(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Opcode 3 takes a single integer as input and saves it to the
    // position given by its only parameter.

    let input = computer.io.take_input();
    store(computer, modes.2, i + 1, input);

    i + 2 // advance 2: 1 opcode + 1 parameter
}

fn opcode_4(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Opcode 4 outputs the value of its only parameter.
    // For example, the instruction 4,50 would output the value at address 50.
    let x = read(computer, modes.2, i + 1);
    computer.io.write_output(x);

    i + 2 // advance 2: 1 opcode + 1 parameter
}

fn opcode_5(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Opcode 5 is jump-if-true: if the first parameter is non-zero,
    // it sets the instruction pointer to the value from the second parameter.
    // Otherwise, it does nothing.
    let x = read(computer, modes.2, i + 1);
    let y = read(computer, modes.1, i + 2);

    if x != 0 {
        y as usize
    } else {
        i + 3 // advance 3: 1 opcode + 2 parameters
    }
}

fn opcode_6(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Opcode 6 is jump-if-false: if the first parameter is zero,
    // it sets the instruction pointer to the value from the second parameter.
    // Otherwise, it does nothing.
    let x = read(computer, modes.2, i + 1);
    let y = read(computer, modes.1, i + 2);

    if x == 0 {
        y as usize
    } else {
        i + 3 // advance 3: 1 opcode + 2 parameters
    }
}

fn opcode_7(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Opcode 7 is less than: if the first parameter is less than the second parameter,
    // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    let x = read(computer, modes.2, i + 1);
    let y = read(computer, modes.1, i + 2);

    store(computer, modes.0, i + 3, if x < y { 1 } else { 0 });

    i + 4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_8(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Opcode 8 is equals: if the first parameter is equal to the second parameter,
    // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    let x = read(computer, modes.2, i + 1);
    let y = read(computer, modes.1, i + 2);
    store(computer, modes.0, i + 3, if x == y { 1 } else { 0 });

    i + 4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_9(modes: Modes, i: usize, computer: &mut Computer) -> usize {
    // Opcode 9 adjusts the relative base by the value of its only parameter. The relative base increases
    // (or decreases, if the value is negative) by the value of the parameter.

    let x = read(computer, modes.2, i + 1) + computer.relative_base as i64;
    computer.relative_base = x as usize;

    i + 2 // advance 2: 1 opcode + 1 parameter
}

pub fn run(codes: &mut Vec<i64>, io: &mut dyn ProgramIO) {
    let mut i: usize = 0;
    let mut computer = Computer {
        codes: codes,
        io: io,
        extents: HashMap::new(),
        relative_base: 0,
    };

    loop {
        let (modes, op) = get_parameters(computer.codes[i]);
        //println!("{:?} === {:?},{:?},{:?} {:?}", computer.codes[i], modes.0, modes.1, modes.2, op);
        match op {
            1 => i = opcode_1(modes, i, &mut computer),
            2 => i = opcode_2(modes, i, &mut computer),
            3 => i = opcode_3(modes, i, &mut computer),
            4 => i = opcode_4(modes, i, &mut computer),
            5 => i = opcode_5(modes, i, &mut computer),
            6 => i = opcode_6(modes, i, &mut computer),
            7 => i = opcode_7(modes, i, &mut computer),
            8 => i = opcode_8(modes, i, &mut computer),
            9 => i = opcode_9(modes, i, &mut computer),
            99 => {
                break;
            }
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
        let mut codes: Vec<i64> = input_ref
            .split(',')
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
        assert_eq!(
            intcode_program("1,1,1,4,99,5,6,0,99", &mut io),
            "30,1,1,4,2,5,6,0,99"
        );
        assert_eq!(
            intcode_program("1,9,10,3,2,3,11,0,99,30,40,50", &mut io),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
    }

    #[test]
    fn test_intcode_parameter_mode() {
        let (modes, op) = get_parameters(01245);
        assert_eq!(modes.0, Mode::Position);
        assert_eq!(modes.1, Mode::Immediate);
        assert_eq!(modes.2, Mode::Relative);
        assert_eq!(op, 45);
    }

    #[test]
    fn test_intcode_parameter_mode_2() {
        let (modes, op) = get_parameters(1002);
        assert_eq!(modes.0, Mode::Position);
        assert_eq!(modes.1, Mode::Immediate);
        assert_eq!(modes.2, Mode::Position);
        assert_eq!(op, 02);
    }

    #[test]
    fn test_intcode_equal_to() {
        let mut io = DefaultProgramIO::new(vec![0, 8]);

        intcode_program("3,9,8,9,10,9,4,9,99,-1,8", &mut io);
        assert_eq!(io.read_output(), 0);

        intcode_program("3,9,8,9,10,9,4,9,99,-1,8", &mut io);
        assert_eq!(io.read_output(), 1);
    }

    #[test]
    fn test_intcode_less_than() {
        let mut io = DefaultProgramIO::new(vec![0, 8]);

        intcode_program("3,9,7,9,10,9,4,9,99,-1,8", &mut io);
        assert_eq!(io.read_output(), 1);

        intcode_program("3,9,7,9,10,9,4,9,99,-1,8", &mut io);
        assert_eq!(io.read_output(), 0);
    }

    #[test]
    fn test_intcode_equal_to_immediate() {
        let mut io = DefaultProgramIO::new(vec![0, 8]);

        intcode_program("3,3,1108,-1,8,3,4,3,99", &mut io);
        assert_eq!(io.read_output(), 0);

        intcode_program("3,3,1108,-1,8,3,4,3,99", &mut io);
        assert_eq!(io.read_output(), 1);
    }

    #[test]
    fn test_intcode_less_than_immediate() {
        let mut io = DefaultProgramIO::new(vec![0, 8]);

        intcode_program("3,3,1107,-1,8,3,4,3,99", &mut io);
        assert_eq!(io.read_output(), 1);

        intcode_program("3,3,1107,-1,8,3,4,3,99", &mut io);
        assert_eq!(io.read_output(), 0);
    }

    #[test]
    fn test_intcode_jump_to_position() {
        let mut io = DefaultProgramIO::new(vec![0, 8]);

        intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &mut io);
        assert_eq!(io.read_output(), 0);

        intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &mut io);
        assert_eq!(io.read_output(), 1);
    }

    #[test]
    fn test_intcode_jump_to_immediate() {
        let mut io = DefaultProgramIO::new(vec![0, 8]);

        intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &mut io);
        assert_eq!(io.read_output(), 0);

        intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &mut io);
        assert_eq!(io.read_output(), 1);
    }

    #[test]
    fn test_intcode_larger_example() {
        let instr = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        let mut io = DefaultProgramIO::new(vec![0, 8, 18]);

        intcode_program(instr, &mut io);
        assert_eq!(io.read_output(), 999);

        intcode_program(instr, &mut io);
        assert_eq!(io.read_output(), 1000);

        intcode_program(instr, &mut io);
        assert_eq!(io.read_output(), 1001);
    }

    #[test]
    fn test_intcode_thruster_signal_1() {
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
    fn test_intcode_thruster_signal_2() {
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
    fn test_intcode_thruster_signal_3() {
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

    #[test]
    fn test_intcode_quine() {
        let instr = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut io = DefaultProgramIO::new(vec![]);
        intcode_program(instr, &mut io);
        assert_eq!(
            io.all.iter().join(","),
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
        );
    }

    #[test]
    fn test_intcode_large_number() {
        let instr = "1102,34915192,34915192,7,4,7,99,0";
        let mut io = DefaultProgramIO::new(vec![]);
        intcode_program(instr, &mut io);
        println!("{:?}", io.all);
        assert_eq!(io.output.to_string().len(), 16);
    }

    #[test]
    fn test_intcode_echo_number() {
        let instr = "104,1125899906842624,99";
        let mut io = DefaultProgramIO::new(vec![]);
        intcode_program(instr, &mut io);
        assert_eq!(io.output, 1125899906842624);
    }
}
