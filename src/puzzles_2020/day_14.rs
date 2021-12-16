use crate::puzzle_input;
use regex::Regex;
use std::collections::HashMap;

const DEFAULT_MASK: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d14-input1.txt");
    println!("** Part 1 Final: {:?}", run_init_program(&input, 1));
    println!("** Part 2 Final: {:?}", run_init_program(&input, 2));
}

fn apply_mask_to_value(mask: &str, val: u64) -> u64 {
    let bval = format!("{:036b}", val);
    let mut result = String::with_capacity(bval.len());
    for (x, y) in mask.chars().zip(bval.chars()) {
        result.push(if x == 'X' { y } else { x });
    }
    u64::from_str_radix(result.as_str(), 2).unwrap()
}

fn apply_mask_to_address(mask: &str, val: u64) -> Vec<u64> {
    let bval = format!("{:036b}", val);
    let mut values: Vec<String> = vec!["".to_string()];
    for (x, y) in mask.chars().zip(bval.chars()) {
        let mut extra: Vec<String> = vec![];
        if x == 'X' {
            for value in values.iter_mut() {
                let mut z = value.clone();
                z.push('1');
                extra.push(z);

                value.push('0');
            }
        } else {
            for value in values.iter_mut() {
                value.push(if x == '1' { '1' } else { y });
            }
        }
        values.extend(extra);
    }

    values
        .iter()
        .map(|x| u64::from_str_radix(x.as_str(), 2).unwrap())
        .collect()
}

fn run_init_program(input: &[String], v: i32) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mem\[(\d+)\]").unwrap();
    }
    let mut mask = DEFAULT_MASK;
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in input {
        let mut split = line.split(" = ");
        let instruction = split.next().unwrap();
        let value = split.next().unwrap();
        if instruction == "mask" {
            mask = value;
        } else if let Some(cap) = RE.captures(instruction) {
            let addr = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let x = value.parse::<u64>().unwrap();
            if v == 2 {
                let addresses = apply_mask_to_address(mask, addr);
                for a in addresses {
                    memory.insert(a, x);
                }
            } else {
                memory.insert(addr, apply_mask_to_value(mask, x));
            }
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask_to_address() {
        let mask = "000000000000000000000000000000X1001X";
        let mut result = apply_mask_to_address(mask, 42);
        result.sort_unstable();
        assert_eq!(result, vec![26, 27, 58, 59]);

        let mask = "00000000000000000000000000000000X0XX";
        result = apply_mask_to_address(mask, 26);
        result.sort_unstable();
        assert_eq!(result, vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }

    #[test]
    fn test_apply_mask_to_value() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        assert_eq!(apply_mask_to_value(mask, 11), 73);
        assert_eq!(apply_mask_to_value(mask, 101), 101);
        assert_eq!(apply_mask_to_value(mask, 0), 64);
    }

    #[test]
    fn test_init_program_v1() {
        let input: Vec<String> = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        assert_eq!(run_init_program(&input, 1), 165);
    }

    #[test]
    fn test_init_program_v2() {
        let input: Vec<String> = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        assert_eq!(run_init_program(&input, 2), 208);
    }
}
