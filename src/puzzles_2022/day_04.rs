use crate::puzzle_input;
use std::ops::RangeInclusive;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d04-input.txt");
    let (contains, overlaps) = eval_assignments(&input);

    println!("** Part 1 Final: {:?}", contains);
    println!("** Part 2 Final: {:?}", overlaps);
}

fn get_ranges(s: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let v: Vec<i32> = s
        .split(&['-', ','][..])
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (
        RangeInclusive::new(v[0], v[1]),
        RangeInclusive::new(v[2], v[3]),
    )
}

fn range_contains(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    (r1.contains(&r2.start()) && r1.contains(&r2.end()))
        || (r2.contains(&r1.start()) && r2.contains(&r1.end()))
}

fn range_overlaps(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    r1.contains(&r2.start())
        || r1.contains(&r2.end())
        || r2.contains(&r1.start())
        || r2.contains(&r1.end())
}

fn eval_assignments(input: &[String]) -> (i32, i32) {
    let mut contains = 0;
    let mut overlaps = 0;

    for line in input {
        if line.is_empty() {
            continue;
        }
        let (r1, r2) = get_ranges(&line);
        if range_contains(&r1, &r2) {
            contains += 1;
        }
        if range_overlaps(&r1, &r2) {
            overlaps += 1;
        }
    }

    (contains, overlaps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8",
        );

        let (contains, overlaps) = eval_assignments(&input);
        assert_eq!(contains, 2);
        assert_eq!(overlaps, 4);
    }
}
