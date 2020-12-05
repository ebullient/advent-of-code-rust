extern crate lazy_static;
extern crate regex;

use crate::puzzle_input;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d05-input1.txt");
    let mut seats: Vec<isize> = vec![];
    for line in input {
        let seat = to_integer(line.as_str());
        seats.push(seat);
    }
    seats.sort();
    let mut seat = 0;
    for n in 1..seats.len() - 2 {
        if seats[n + 2] - seats[n] != 2 {
            seat = seats[n] + 1;
        }
    }
    println!("** Part 1 Final: {:?}", seats.pop().unwrap());
    println!("** Part 2 Final: {:?}", seat);
}

fn to_integer(s: &str) -> isize {
    let bs: String = s
        .chars()
        .map(|x| match x {
            'B' => '1',
            'F' => '0',
            'L' => '0',
            'R' => '1',
            _ => x,
        })
        .collect();
    isize::from_str_radix(bs.as_str(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsp() {
        assert_eq!(to_integer("BFFFBBFRRR"), 567);
        assert_eq!(to_integer("FFFBBBFRRR"), 119);
        assert_eq!(to_integer("BBFFBBFRLL"), 820);
    }
}
