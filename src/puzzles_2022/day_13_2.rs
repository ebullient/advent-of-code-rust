use crate::puzzle_input;
use std::cmp::Ordering;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d13-input.txt");
    let (_, decoder_key) = sort(&input);

    println!("** Part 1 Final: {:?}", iterate_sum(&input));
    println!("** Part 2 Final: {:?}", decoder_key);
}

#[derive(Debug, PartialEq)]
enum Token {
    ListStart,
    ListEnd,
    ListDelimit,
    Digit,
    EOL,
}

fn read(s: &mut String) -> (Token, i32) {
    if s.is_empty() {
        return (Token::EOL, -1);
    }
    let next = s.remove(0);
    match next {
        '[' => return (Token::ListStart, -1),
        ']' => return (Token::ListEnd, -1),
        ',' => return (Token::ListDelimit, -1),
        _ => {} // continue
    };

    let mut str = String::from(next);
    while let Some(x) = s.chars().next() {
        if x.is_digit(10) {
            str.push(s.remove(0));
        } else {
            return (Token::Digit, str.parse::<i32>().unwrap());
        }
    }
    (Token::EOL, -1)
}

fn compare(left: &mut String, right: &mut String) -> Ordering {
    loop {
        if left.is_empty() && right.is_empty() {
            return Ordering::Equal;
        }
        let l = read(left);
        let r = read(right);

        if l.0 == r.0 {
            // Tokens of the same type
            if l.0 == Token::Digit && l.1 != r.1 {
                return if l.1 < r.1 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }; // unequal digits
            }
            // lots of carrying on as usual here: matching [ and ] and ,
        } else if l.0 == Token::ListStart && r.0 == Token::Digit {
            left.insert(0, '['); // back up
            right.insert_str(0, &format!("[{}]", r.1));
        } else if l.0 == Token::Digit && r.0 == Token::ListStart {
            right.insert(0, '['); // back up
            left.insert_str(0, &format!("[{}]", l.1));
        } else if l.0 == Token::ListEnd {
            return Ordering::Less;
        } else if r.0 == Token::ListEnd {
            return Ordering::Greater;
        } else {
            panic!("What lands here?")
        }
    }
}

fn iterate_sum(input: &[String]) -> i32 {
    let mut result = 0;
    let mut i = 1;
    let mut iter = input.iter();

    while let Some(line) = iter.next() {
        if line.is_empty() {
            continue;
        }
        let order = compare(&mut line.to_string(), &mut iter.next().unwrap().to_string());
        if order == Ordering::Less {
            result += i;
        }
        i += 1;
    }
    result
}

fn sort(input: &[String]) -> (Vec<String>, i32) {
    let mut all = input.to_vec();
    all.retain(|x| !x.is_empty());
    all.push(String::from("[[2]]"));
    all.push(String::from("[[6]]"));

    all.sort_by(|a, b| compare(&mut a.to_string(), &mut b.to_string()));

    let decoder_key = all
        .iter()
        .enumerate()
        .filter(|(_, v)| *v == "[[2]]" || *v == "[[6]]")
        .map(|(i, _)| i as i32 + 1)
        .product::<i32>();

    (all, decoder_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    fn test_in_order(left: &str, right: &str) -> bool {
        let mut l_value = left.to_string();
        let mut r_value = right.to_string();
        let order = compare(&mut l_value, &mut r_value);
        order == Ordering::Less
    }

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "[1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]",
        );

        assert_eq!(test_in_order(&"[1,1,3,10,1]", &"[1,1,5,10,1]"), true);
        assert_eq!(test_in_order(&"[[1],[2,3,4]]", &"[[1],10]"), true);
        assert_eq!(test_in_order(&"[9]", &"[[8,7,6]]"), false);
        assert_eq!(test_in_order(&"[[4,4],4,4]", &"[[4,4],4,4,4]"), true);
        assert_eq!(test_in_order(&"[]", &"[3]"), true);
        assert_eq!(test_in_order(&"[[[]]]", &"[[]]"), false);

        assert_eq!(iterate_sum(&input), 13);

        let (_, decoder_key) = sort(&input);
        assert_eq!(decoder_key, 140);
    }
}
