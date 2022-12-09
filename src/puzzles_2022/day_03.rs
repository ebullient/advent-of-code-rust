use crate::puzzle_input;
use std::collections::HashSet;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d03-input.txt");

    println!("** Part 1 Final: {:?}", examine_backpacks(&input));
    println!("** Part 2 Final: {:?}", examine_team_backpacks(&input));
}

const LOWER: u32 = '`' as u32;
const UPPER: u32 = '@' as u32;

fn get_priority(c: char) -> u32 {
    if c as u32 > LOWER as u32 {
        return c as u32 - LOWER;
    }
    return c as u32 - UPPER + 26;
}

fn find_duplicates(pack: &str) -> HashSet<char> {
    let p = pack.len() / 2;
    let half = &pack[p..];
    let mut dups = HashSet::new();

    for (x, ch) in pack.chars().enumerate() {
        if x >= p {
            break;
        }
        if half.contains(ch) {
            dups.insert(ch);
        }
    }

    dups
}

fn examine_backpacks(input: &[String]) -> u32 {
    let mut total = 0;

    for line in input {
        if line.is_empty() {
            continue;
        }

        let dups = find_duplicates(line);
        for d in dups {
            total += get_priority(d);
        }
    }

    total
}

fn find_badges(p1: &str, p2: &str, p3: &str) -> char {
    let p1_chars: HashSet<char> = p1.chars().collect();
    let p2_chars: HashSet<char> = p2.chars().collect();
    let p1_p2: HashSet<char> = p1_chars.intersection(&p2_chars).map(|x| *x).collect();

    let p3_chars: HashSet<char> = p3.chars().collect();
    let p1_p2_p3: HashSet<_> = p3_chars.intersection(&p1_p2).map(|x| *x).collect();

    if p1_p2_p3.len() == 0 {
        panic!("Could not find badge: {:?} {:?} {:?}", p1, p2, p3);
    } else if p1_p2_p3.len() > 1 {
        panic!("Too many common items: {:?} {:?} {:?}", p1, p2, p3);
    }

    *p1_p2_p3.iter().next().unwrap()
}

fn examine_team_backpacks(input: &[String]) -> u32 {
    let mut total = 0;
    let mut iter = input.iter().peekable();

    loop {
        let p1 = iter.next();
        if p1.is_none() {
            break;
        }
        let p2 = iter.next();
        let p3 = iter.next();
        let badge = find_badges(p1.unwrap(), p2.unwrap(), p3.unwrap());
        total += get_priority(badge);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(examine_backpacks(&input), 157);
        assert_eq!(examine_team_backpacks(&input), 70);
    }
}
