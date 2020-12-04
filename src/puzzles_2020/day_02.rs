use crate::puzzle_input;

pub fn run() {
    let mut lines = puzzle_input::read_all_lines("./input/2020-d02-input1.txt");

    println!("** Part 1 Final: {:?}", lines.iter_mut()
        .filter(|x| is_valid(x))
        .count());


    println!("** Part 2 Final: {:?}", lines.iter_mut()
        .filter(|x| is_really_valid(x))
        .count());
}

fn is_valid(line: &str) -> bool {
    let mut iter = line.split_whitespace();
    let mut repeat_iter = iter.next().unwrap().split('-');
    let letter = iter.next().unwrap().chars().nth(0).unwrap();
    let password = iter.next().unwrap();

    let min = repeat_iter.next().unwrap().parse::<usize>().unwrap();
    let max = repeat_iter.next().unwrap().parse::<usize>().unwrap();
    let count = password.chars().filter(|x| *x == letter).count();

    count >= min && count <= max
}

fn char_at(s: &str, pos: usize) -> char {
    let n = (pos - 1) as usize;
    s.chars().nth(n).unwrap()
}

fn is_really_valid(line: &str) -> bool {
    let mut iter = line.split_whitespace();
    let mut position_iter = iter.next().unwrap().split('-');
    let letter = iter.next().unwrap().chars().nth(0).unwrap();
    let password = iter.next().unwrap();

    let first = position_iter.next().unwrap().parse::<usize>().unwrap();
    let last = position_iter.next().unwrap().parse::<usize>().unwrap();

    // character should not appear in both places
    (letter == char_at(password, first)) != (letter == char_at(password, last))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sled_password() {
        assert_eq!(is_valid("1-3 a: abcde"), true);
        assert_eq!(is_valid("1-3 b: cdefg"), false);
        assert_eq!(is_valid("2-9 c: ccccccccc"), true);
    }

    #[test]
    fn test_char_at() {
        assert_eq!(char_at("1-3 a: abcde", 3), '3');
    }

    #[test]
    fn test_valid_toboggan_password() {
        assert_eq!(is_really_valid("1-3 a: abcde"), true);
        assert_eq!(is_really_valid("1-3 b: cdefg"), false);
        assert_eq!(is_really_valid("2-9 c: ccccccccc"), false);
    }
}
