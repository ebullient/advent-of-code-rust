use crate::puzzle_input;
use std::cmp;

pub fn run() {
    let mut input: Vec<i32> = puzzle_input::read_string("./input/2021-d07-input.txt")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (single, exp) = calculate_min_fuel(&mut input);
    println!("** Part 1 Final: {:?}", single);
    println!("** Part 2 Final: {:?}", exp);
}

fn calculate_min_fuel(input: &mut Vec<i32>) -> (i32, i32) {
    input.sort_unstable();

    let median = input[input.len() / 2];

    // The answer to part 2 is going to be somewhere around the mean of the set.
    let mean: f32 = input.iter().sum::<i32>() as f32 / input.len() as f32;
    let m1: i32 = mean.floor() as i32;
    let m2: i32 = mean.ceil() as i32;

    println!("median is {:?}", median);
    println!("mean: {:?} {:?}", m1, m2);

    let single = input.iter().map(|x| (median - x).abs()).sum();

    let mut exp = i32::MAX;
    for i in 0..input.len() / 2 {
        let (x1, x2) = triangle(input, m1 - i as i32, m2 + i as i32);
        let next = cmp::min(x1, x2);
        if next >= exp {
            break;
        }
        exp = next;
    }
    (single, exp)
}

fn triangle(input: &mut Vec<i32>, m1: i32, m2: i32) -> (i32, i32) {
    input
        .iter()
        .map(|x| {
            let n1 = (m1 - x).abs();
            let n2 = (m2 - x).abs();
            ((n1 * (n1 + 1)) / 2 as i32, (n2 * (n2 + 1)) / 2 as i32)
        })
        .reduce(|accum, item| (accum.0 + item.0, accum.1 + item.1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input: Vec<i32> = "16,1,2,0,4,2,7,1,2,14"
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (single, exp) = calculate_min_fuel(&mut input);
        assert_eq!(37, single);
        assert_eq!(168, exp);
    }
}
