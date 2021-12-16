use crate::puzzle_input;
use std::collections::HashSet;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d08-input1.txt");

    let (result, _) = detect_loop(&input);
    println!("** Part 1 Final: {:?}", result);
    println!("** Part 2 Final: {:?}", try_repair(&input));
}

pub fn try_repair(program: &[String]) -> i32 {
    let mut input = program.to_vec();
    for (i, line) in program.iter().enumerate() {
        if line.starts_with("nop") || line.starts_with("jmp") {
            input[i] = if line.starts_with("nop") {
                line.replace("nop", "jmp")
            } else {
                line.replace("jmp", "nop")
            };
            let (result, infinite) = detect_loop(&input);
            if !infinite {
                return result;
            }
            input[i] = line.to_string();
        }
    }
    0
}

pub fn detect_loop(program: &[String]) -> (i32, bool) {
    let mut console = GameConsole::new(program);
    let mut set = HashSet::new();
    let mut inf = false;

    while let Ok(position) = console.step() {
        if !set.insert(position) {
            println!("Infinite loop, repeating {:?}", console.position);
            inf = true;
            break;
        }
        if position >= console.instructions.len() {
            println!("The End!");
            inf = false;
            break;
        }
    }
    (console.accumulator, inf)
}

struct GameConsole<'a> {
    instructions: &'a [String],
    accumulator: i32,
    position: usize,
}
impl<'a> GameConsole<'a> {
    pub fn new(values: &'a [String]) -> GameConsole {
        GameConsole {
            instructions: values,
            accumulator: 0,
            position: 0,
        }
    }

    fn get_parameters(&self) -> Result<(&str, i32), String> {
        if self.position >= self.instructions.len() {
            return Err(format!("{:?} exceeends program length", self.position));
        }
        let mut s = self.instructions[self.position].split(' ');
        if let Some(op) = s.next() {
            if let Some(a) = s.next() {
                if let Ok(arg) = a.parse::<i32>() {
                    return Ok((op, arg));
                }
            }
        }
        Err(format!(
            "Unable to parse instructions from {:?} at line {:?}",
            s, self.position
        ))
    }

    pub fn step(&mut self) -> Result<usize, String> {
        match self.get_parameters() {
            Ok((op, arg)) => {
                return match op {
                    "acc" => {
                        self.accumulator += arg;
                        self.position += 1;
                        Ok(self.position)
                    }
                    "jmp" => {
                        let x = self.position as i32 + arg;
                        self.position = x as usize;
                        Ok(self.position)
                    }
                    "nop" => {
                        self.position += 1;
                        Ok(self.position)
                    }
                    _ => {
                        let msg = format!("Unknown operation {:?}", op);
                        println!("{}", msg);
                        Err(msg)
                    }
                }
            }
            Err(msg) => Err(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_console() {
        let input: Vec<String> = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        let (result, infinite) = detect_loop(&input);
        assert_eq!(result, 5);
        assert!(infinite);

        assert_eq!(try_repair(&input), 8);
    }
}
