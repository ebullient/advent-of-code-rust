use itertools::Itertools;

use crate::puzzle_input;
pub fn run() {
    let input: String = puzzle_input::read_string("./input/2022-d06-input.txt");

    println!("** Part 1 Final: {:?}", find(&input.as_str(), 4));
    println!("** Part 2 Final: {:?}", find(&input.as_str(), 14));
}

fn find(input: &str, how_many: usize) -> usize {
    let mut i = how_many;
    for w in input.as_bytes().windows(how_many) {
        if w.iter().unique().count() == how_many {
            println!("{:?}: Found marker {:?}", i, w.iter().join(""));
            return i;
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
        assert_eq!(find("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(find("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

        assert_eq!(find("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
