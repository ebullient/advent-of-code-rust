use crate::puzzle_input;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d05-input.txt");
    let (mut stacks, instructions) = get_stacks(&input);
    let mut stacks2 = stacks.clone();

    let result = move_one_by_one(&mut stacks, &instructions);
    println!("** Part 1 Final: {:?}", result.iter().join(""));

    let result2 = move_many_crates(&mut stacks2, &instructions);
    println!("** Part 2 Final: {:?}", result2.iter().join(""));
}

fn get_stacks(input: &[String]) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\[(.)\]|   ) ?").unwrap();
        static ref MOVE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();

    stacks.push(Vec::with_capacity(1)); // 0
    for line in input {
        if line.is_empty() {
            continue;
        }

        if line.contains("[") {
            let mut i = 1;
            for caps in RE.captures_iter(line) {
                if let Some(j) = caps.get(2) {
                    let ch = j.as_str().chars().nth(0).unwrap();
                    if let Some(stack) = stacks.get_mut(i) {
                        stack.insert(0, ch);
                    } else {
                        let mut stack = Vec::new();
                        stack.push(ch);
                        stacks.push(stack);
                    }
                } else if stacks.len() <= i {
                    stacks.push(Vec::new());
                }
                i += 1;
            }
        } else if line.starts_with("move") {
            let caps = MOVE.captures(line).unwrap();
            instructions.push((
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
                caps[3].parse::<usize>().unwrap(),
            ));
        }
    }

    println!("{:?}", stacks);
    println!("{:?}", instructions);

    (stacks, instructions)
}

fn move_one_by_one(
    stacks: &mut Vec<Vec<char>>,
    instructions: &Vec<(usize, usize, usize)>,
) -> Vec<char> {
    for (n, from, to) in instructions {
        println!("move {:?} from {:?} to {:?}", n, from, to);
        for i in 0..*n {
            let x = &stacks[*from].pop().unwrap();
            let _ = &stacks[*to].push(*x);
            println!("{:?}: {:?}", i, stacks);
        }
    }
    get_top_crates(stacks)
}

fn move_many_crates(
    stacks: &mut Vec<Vec<char>>,
    instructions: &Vec<(usize, usize, usize)>,
) -> Vec<char> {
    println!("{:?}", stacks);
    for (n, from, to) in instructions {
        println!("move {:?} from {:?} to {:?}", n, from, to);
        let idx = &stacks[*from].len() - n;
        let mut s: Vec<char> = stacks[*from].splice(idx.., []).collect();
        let _ = &stacks[*to].append(&mut s);
        println!("{:?}", stacks);
    }
    get_top_crates(stacks)
}

fn get_top_crates(stacks: &Vec<Vec<char>>) -> Vec<char> {
    let mut result = Vec::with_capacity(stacks.len());
    for (i, stack) in stacks.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if let Some(top) = stack.iter().last() {
            result.push(*top);
        } else {
            result.push(' ');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
            "#
        .split('\n')
        .map(|x| x.to_string())
        .collect();

        let (mut stacks, instructions) = get_stacks(&input);
        let mut stacks2 = stacks.clone();

        let result = move_one_by_one(&mut stacks, &instructions);
        assert_eq!(result.iter().join(""), "CMZ");

        println!("-----");

        let result2 = move_many_crates(&mut stacks2, &instructions);
        assert_eq!(result2.iter().join(""), "MCD");
    }
}
