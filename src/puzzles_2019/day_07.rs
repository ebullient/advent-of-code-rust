use super::compute;
use super::compute::{DefaultProgramIO, ProgramIO};
use crate::puzzle_input;

extern crate scoped_threadpool;
use scoped_threadpool::Pool;
use std::sync::mpsc::{channel, Receiver, Sender};
use itertools::Itertools;
use std::time::Duration;


pub fn run() {
    let input = puzzle_input::read_string("./input/2019-d07-input1.txt");
    let codes: Vec<i64> = input.trim().split(',')
                                .map(|x| x.trim().parse::<i64>().unwrap())
                                .collect();

    println!("** Part 1 Final: {:?}", find_max_thrust(&codes));

    let input = puzzle_input::read_string("./input/2019-d07-input1.txt");
    let codes: Vec<i64> = input.trim().split(',')
                                .map(|x| x.trim().parse::<i64>().unwrap())
                                .collect();

    println!("** Part 2 Final: {:?}", find_max_thrust_feedback(&codes));
}

fn find_max_thrust(codes: &Vec<i64>) -> i64 {
    let phases = vec![0, 1, 2, 3, 4];
    let mut max = 0;
    let mut max_sequence: Vec<i32> = phases.to_vec();

    for perm in phases.iter().permutations(phases.len()).unique() {
        let mut io = DefaultProgramIO::new(vec![]);
        let current: Vec<i32> = perm.iter().copied().map(|&x| x).collect();
        let mut last = 0;

        for x in &current {
            // Two inputs: phase, and input value (which starts at 0, and is then output of previous stage)
            ProgramIO::add_input(&mut io, (*x).into());
            ProgramIO::add_input(&mut io, last);
            compute::run(&mut codes.to_vec(), &mut io);
            last = ProgramIO::read_output(&io);
            // println!("  - phase {:?}: out {:?}", x, last);
        }

        if last > max {
            max = last;
            max_sequence = current;
        }
        println!("{:?}: out {:?}, max {:?} -> {:?}", perm, last, &max_sequence, max);
    }
    max
}

#[derive(Debug)]
struct Amplifier {
    pub name: char,
    codes: Vec<i64>,
    output: i64,
    next: Option<Sender<i64>>,
    tx: Sender<i64>,
    rx: Receiver<i64>,
}

impl Amplifier {
    pub fn new(ch: char, program: &Vec<i64>) -> Amplifier {
        let (tx, rx) = channel::<i64>();
        Amplifier {
            name: ch,
            codes: program.to_vec(),
            output: 0,
            next: None,
            tx: tx,
            rx: rx
        }
    }

    pub fn connect_next(&mut self, other: &Amplifier) {
        self.next = Some(other.tx.clone());
    }
}

impl compute::ProgramIO for Amplifier {
    fn add_input(&mut self, value: i64) {
        // println!("PUSH {:?}: -> {:?}", self.name, value);
        self.tx.send(value).unwrap();
    }

    fn take_input(&mut self) -> i64 {
        let value = match self.rx.recv_timeout(Duration::from_millis(400)) {
            Ok(value) => value,
            Err(error) => panic!("Unable to read value for {} {:?}", self.name, error),
        };

        // println!("RECV {:?}: <- {:?}", self.name, value);
        value
    }

    fn write_output(&mut self, value: i64) {
        // println!("SEND {:?}: -> {:?} .. {:?}", self.name, value, self.next);
        self.output = value;
        if let Some(sender) = &self.next {
            sender.send(value).unwrap();
        }
    }

    fn read_output(&self) -> i64 {
        // println!("OUT {:?}: {:?}", self.name, self.output);
        self.output
    }
}

fn get_amp_next_mut<T>(slice: &mut [T], index1: usize) -> (&mut T, &mut T) {
    let max = slice.len() - 1;
    let mut iter = slice.iter_mut();
    let first: &mut T;
    let last: &mut T;
    if index1 < max {
        first = iter.nth(index1).unwrap();
        last = iter.nth(0).unwrap();
    } else {
        last = iter.nth(0).unwrap();
        first = iter.nth(index1 - 1).unwrap();
    }
    (first, last)
}

fn find_max_thrust_feedback(codes: &Vec<i64>) -> i64 {
    let mut pool = Pool::new(5);
    let phases = vec![5, 6, 7, 8, 9];

    let mut max = 0;
    let mut max_sequence: Vec<i64> = phases.to_vec();

    for perm in phases.iter().permutations(phases.len()).unique() {
        let last: i64;

        let mut amplifiers: Vec<Amplifier> = vec![
            Amplifier::new('A', &codes),
            Amplifier::new('B', &codes),
            Amplifier::new('C', &codes),
            Amplifier::new('D', &codes),
            Amplifier::new('E', &codes)];

        let current: Vec<i64> = perm.iter().copied().map(|&x| x).collect();
        for (i, x) in current.iter().enumerate() {
            let (amp, next) = get_amp_next_mut(&mut amplifiers, i);

            // Set initial phase input for each amplifier
            ProgramIO::add_input(amp, *x);

            // Connect amplifier output to the next amp's input
            amp.connect_next(next);
        }

        // Provide required input for pass of Amplifier A
        ProgramIO::add_input(&mut amplifiers[0], 0);

        pool.scoped(|scope| {
            for amp in &mut amplifiers {
                scope.execute(move|| {
                    compute::run(&mut amp.codes.to_vec(), amp);
                });
            }
        });

        last = amplifiers[4].read_output();
        if last > max {
            max = last;
            max_sequence = current;
        }

        println!("{:?}: out {:?}, max {:?} -> {:?}", perm, last, &max_sequence, max);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thruster_signal_3() {
        let instr = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

        let codes: Vec<i64> = instr.split(',')
                                    .map(|x| x.trim().parse::<i64>().unwrap())
                                    .collect();

        assert_eq!(find_max_thrust(&codes), 65210);
    }

    #[test]
    fn test_thruster_feedback_loop() {
        let instr = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
        27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

        let codes: Vec<i64> = instr.split(',')
                                    .map(|x| x.trim().parse::<i64>().unwrap())
                                    .collect();

        assert_eq!(find_max_thrust_feedback(&codes), 139629729);
    }

    #[test]
    fn test_thruster_feedback_loop_2() {
        let instr = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
        -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

        let codes: Vec<i64> = instr.split(',')
                                    .map(|x| x.trim().parse::<i64>().unwrap())
                                    .collect();

        assert_eq!(find_max_thrust_feedback(&codes), 18216);
    }
}
