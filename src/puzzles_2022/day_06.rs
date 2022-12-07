use itertools::Itertools;

use crate::puzzle_input;
use std::collections::{HashMap, VecDeque};

pub fn run() {
    let input: String = puzzle_input::read_string("./input/2022-d06-input.txt");

    println!("** Part 1 Final: {:?}", find_marker(&input.as_str()));
    println!("** Part 2 Final: {:?}", find_message(&input.as_str()));
}

fn find_marker(input: &str) -> usize {
    let mut i: usize = 1;
    let mut marker: VecDeque<char> = VecDeque::with_capacity(5);
    let mut letters: HashMap<char, usize> = HashMap::with_capacity(26);

    for c in input.chars() {
        // println!("{:?}: {:?} ... {:?}", i, c, letters);
        marker.push_back(c);
        letters.entry(c).and_modify(|i| *i += 1).or_insert(1);
        if i >= 4 {
            if letters.values().all(|&x| x < 2) {
                println!("{:?}: Found marker {:?}", i, marker.iter().join(""));
                return i;
            }
            let front = marker.pop_front().unwrap();
            letters.entry(front).and_modify(|i| *i -= 1);
        }
        i += 1;
    }

    panic!("Marker not found");
}

fn find_message(input: &str) -> usize {
    let mut i: usize = 1;
    let mut marker: VecDeque<char> = VecDeque::with_capacity(15);
    let mut letters: HashMap<char, usize> = HashMap::with_capacity(26);

    for c in input.chars() {
        println!("{:?}: {:?} ... {:?}", i, c, letters);
        marker.push_back(c);
        letters.entry(c).and_modify(|i| *i += 1).or_insert(1);
        if i >= 14 {
            if letters.values().all(|&x| x < 2) {
                println!("{:?}: Found message {:?}", i, marker.iter().join(""));
                return i;
            }
            let front = marker.pop_front().unwrap();
            letters.entry(front).and_modify(|i| *i -= 1);
        }
        i += 1;
    }

    panic!("Marker not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(find_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
