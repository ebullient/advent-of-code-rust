use crate::puzzle_input;
use serde_json::Value;
use std::cmp::Ordering;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d13-input.txt");
    let (_, decoder_key) = sort(&input);

    println!("** Part 1 Final: {:?}", iterate_sum(&input));
    println!("** Part 2 Final: {:?}", decoder_key);
}

fn json_array(v: Value) -> Value {
    let x = v.as_i64().unwrap();
    serde_json::from_str(&format!("[{}]", x)).unwrap()
}

fn compare(left: &mut Vec<Value>, right: &mut Vec<Value>) -> Ordering {
    let mut result = Ordering::Equal;
    while result == Ordering::Equal {
        if left.is_empty() && right.is_empty() {
            break;
        } else if left.is_empty() {
            return Ordering::Less;
        } else if right.is_empty() {
            return Ordering::Greater;
        }

        let mut l = left.remove(0);
        let mut r = right.remove(0);

        if l.is_number() && r.is_number() {
            let x = l.as_i64().unwrap();
            let y = r.as_i64().unwrap();
            if x != y {
                result = if x < y {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
        } else if l.is_array() && r.is_array() {
            let nested_l = l.as_array_mut().unwrap();
            let nested_r = r.as_array_mut().unwrap();
            result = compare(nested_l, nested_r);
        } else if l.is_number() && r.is_array() {
            left.insert(0, json_array(l));
            right.insert(0, r); // put back
        } else if l.is_array() && r.is_number() {
            left.insert(0, l); // put back
            right.insert(0, json_array(r));
        }
    }
    result
}

fn iterate_sum(input: &[String]) -> i32 {
    let mut result = 0;
    let mut i = 1;
    let mut iter = input.iter();

    while let Some(line) = iter.next() {
        if line.is_empty() {
            continue;
        }
        let mut l_value: Value = serde_json::from_str(&line).unwrap();
        let mut r_value: Value = serde_json::from_str(&iter.next().unwrap()).unwrap();
        let order = compare(
            &mut l_value.as_array_mut().unwrap(),
            &mut r_value.as_array_mut().unwrap(),
        );
        println!("{:?}", line);
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

    all.sort_by(|a, b| {
        let mut l_value: Value = serde_json::from_str(&a).unwrap();
        let mut r_value: Value = serde_json::from_str(&b).unwrap();
        compare(
            &mut l_value.as_array_mut().unwrap(),
            &mut r_value.as_array_mut().unwrap(),
        )
    });

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
        let mut l_value: Value = serde_json::from_str(&left).unwrap();
        let mut r_value: Value = serde_json::from_str(&right).unwrap();
        let order = compare(
            &mut l_value.as_array_mut().unwrap(),
            &mut r_value.as_array_mut().unwrap(),
        );
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
