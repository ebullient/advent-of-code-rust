extern crate lazy_static;
extern crate regex;

use crate::puzzle_input;
use regex::Regex;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d05-input1.txt");
    let mut seats: Vec<isize> = vec![];
    for line in input {
        let (_, _, seat) = find_seat(line.as_str());
        seats.push(seat);
    }
    seats.sort();
    let mut seat = 0;
    for n in 1..seats.len() - 2 {
        if seats[n + 2] - seats[n] != 2 {
            seat = seats[n] + 1;
        }
        println!("{} {} {}", seats[n], seats[n + 1], seats[n + 2]);
    }
    println!("** Part 1 Final: {:?}", seats.pop().unwrap());
    println!("** Part 2 Final: {:?}", seat);
}

pub fn find_seat(s: &str) -> (isize, isize, isize) {
    lazy_static! {
        static ref TICKET: Regex = Regex::new(r"^([FB]+)([LR]+)$").unwrap();
    }
    let row: isize;
    let column: isize;

    if let Some(caps) = TICKET.captures(&s) {
        row = to_integer(&caps[1]);
        column = to_integer(&caps[2]);
    } else {
        row = 0;
        column = 0;
    }

    (row, column, row * 8 + column)
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
        let (row, column, seat) = find_seat("BFFFBBFRRR");
        assert_eq!(row, 70);
        assert_eq!(column, 7);
        assert_eq!(seat, 567);
        let (row, column, seat) = find_seat("FFFBBBFRRR");
        assert_eq!(row, 14);
        assert_eq!(column, 7);
        assert_eq!(seat, 119);
        let (row, column, seat) = find_seat("BBFFBBFRLL");
        assert_eq!(row, 102);
        assert_eq!(column, 4);
        assert_eq!(seat, 820);
    }
}
