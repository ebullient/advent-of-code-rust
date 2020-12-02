use crate::puzzle_input;

pub fn run() {
    let mut n = 0;
    if let Ok(mut lines) = puzzle_input::read_lines("./input/2020-d02-input1.txt") {
        while let Some(item) = lines.next() {
            if let Ok(line) = item {
                if is_valid(line.as_str()) {
                    n += 1;
                }
            }
        }
    }
    println!("** Part 1 Final: {:?}", n);

    n = 0;
    if let Ok(mut lines) = puzzle_input::read_lines("./input/2020-d02-input1.txt") {
        while let Some(item) = lines.next() {
            if let Ok(line) = item {
                if is_really_valid(line.as_str()) {
                    n += 1;
                }
            }
        }
    }
    println!("** Part 2 Final: {:?}", n);
}

fn is_valid(line: &str) -> bool {
    let mut iter = line.split_whitespace();
    let mut repeat_iter = iter.next().unwrap().split('-');
    let letter = iter.next().unwrap().trim_matches(':').parse::<char>().unwrap();
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
    let letter = iter.next().unwrap().trim_matches(':').parse::<char>().unwrap();
    let password = iter.next().unwrap();

    let first = position_iter.next().unwrap().parse::<usize>().unwrap();
    let last = position_iter.next().unwrap().parse::<usize>().unwrap();

    (letter == char_at(password, first) && letter != char_at(password, last)) || 
    (letter != char_at(password, first) && letter == char_at(password, last))
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