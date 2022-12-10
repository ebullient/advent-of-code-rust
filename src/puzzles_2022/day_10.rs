use crate::puzzle_input;
use regex::Regex;
use std::collections::VecDeque;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d10-input.txt");

    let mut part1 = ClockCircuit::new();
    part1.program(&input);
    let mut part2 = part1.clone();

    let result = part1.run(20, 40);
    println!("** Part 1 Final: {:?}", result.iter().sum::<i64>());

    part2.scan_crt();
}

#[derive(Clone, Debug)]
struct CPU {
    register: i64,
    queue: VecDeque<i64>,
}
impl CPU {
    fn new() -> CPU {
        CPU {
            register: 1,
            queue: VecDeque::new(),
        }
    }

    fn addx(&mut self, value: i64) {
        self.queue.push_back(0);
        self.queue.push_back(value);
    }

    fn noop(&mut self) {
        self.queue.push_back(0);
    }

    fn tick(&mut self) -> (i64, bool) {
        let result = self.register;
        let mut has_next = false;
        if let Some(value) = self.queue.pop_front() {
            self.register += value;
            has_next = true;
        }
        (result, has_next)
    }
}

#[derive(Clone, Debug)]
struct ClockCircuit {
    cpu: CPU,
}
impl ClockCircuit {
    fn new() -> ClockCircuit {
        ClockCircuit { cpu: CPU::new() }
    }

    fn program(&mut self, input: &[String]) {
        lazy_static! {
            static ref INSTR: Regex = Regex::new(r"(\S+) ([\d-]+)").unwrap();
        }
        for line in input {
            if line.eq("noop") {
                self.cpu.noop();
            } else if INSTR.is_match(line) {
                let caps = INSTR.captures(line).unwrap();
                let value = (&caps[2]).parse::<i64>().unwrap();
                match &caps[1] {
                    "addx" => self.cpu.addx(value),
                    _ => panic!("What is this? {:?} {:?}", line, value),
                }
            } else {
                panic!("What is this? {:?}", line);
            }
        }
    }

    fn run(&mut self, initial: i64, interval: i64) -> Vec<i64> {
        let mut i = 1;
        let mut result = Vec::new();
        let mut tick = self.cpu.tick();
        while tick.1 {
            if i == initial || (i > initial && (i - initial) % interval == 0) {
                result.push(tick.0 * i);
            }
            tick = self.cpu.tick();
            i += 1;
        }
        result
    }

    fn scan_crt(&mut self) {
        let mut cycle = 1;
        let mut sprite = self.cpu.tick();
        while sprite.1 {
            if (cycle - 2..=cycle).contains(&sprite.0) {
                print!("#");
            } else {
                print!(".");
            }
            if cycle % 40 == 0 {
                println!();
                cycle = 0;
            }
            sprite = self.cpu.tick();
            cycle += 1;
        }
    }

    #[allow(dead_code)]
    fn step(&mut self) -> (i64, bool) {
        self.cpu.tick()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short() {
        let input: Vec<String> = puzzle_input::split_string(
            "noop
        addx 3
        addx -5",
        );

        let mut circuit = ClockCircuit::new();
        circuit.program(&input);

        assert_eq!(circuit.step(), (1, true));
        assert_eq!(circuit.step(), (1, true));
        assert_eq!(circuit.step(), (1, true));
        assert_eq!(circuit.step(), (4, true));
        assert_eq!(circuit.step(), (4, true));
        assert_eq!(circuit.step(), (-1, false));
    }

    #[test]
    fn test_long() {
        let input: Vec<String> = puzzle_input::split_string(
            "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop",
        );

        let mut circuit = ClockCircuit::new();
        circuit.program(&input);
        let mut crt = circuit.clone();

        let result = circuit.run(20, 40);
        assert_eq!(result[0], 420);
        assert_eq!(result[1], 1140);
        assert_eq!(result[2], 1800);
        assert_eq!(result[3], 2940);
        assert_eq!(result[4], 2880);
        assert_eq!(result[5], 3960);
        assert_eq!(result.iter().sum::<i64>(), 13140);

        crt.scan_crt();
    }
}
