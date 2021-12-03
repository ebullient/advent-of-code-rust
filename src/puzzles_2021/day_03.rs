use crate::puzzle_input;
use std::convert::TryInto;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d03-input.txt");

    let (gamma, epsilon) = count_bits(&input);
    println!("** Part 1 Final: {:?} == 841526", product(&gamma, &epsilon));

    let o2 = filter_bits(&input, gamma.chars().nth(0).unwrap(), '1', '1', '0');
    let co2 = filter_bits(&input, epsilon.chars().nth(0).unwrap(), '0', '0', '1');
    println!("** Part 2 Final: {:?} == 4790390", product(&o2, &co2));
}

// Use the binary numbers to generate two new binary numbers:
//   the gamma rate and the epsilon rate
//  Each bit in the gamma rate:
//     the most common bit in the corresponding position of all numbers in the list.
//  Each bit in the epsilon rate:
//     the least common bit from each position is used.
fn count_bits(report: &Vec<String>) -> (String, String) {
    let width = report.get(0).unwrap().len();
    let max: u32 = (report.len() / 2).try_into().unwrap();
    let sum = vec![0; width];

    let result = report.iter().fold(sum, |mut acc, line| {
        for (i, _) in line.chars().enumerate().filter(|(_, x)| *x == '1') {
            acc[i] += 1;
        }
        acc
    });

    // Keep the string values because I'm insane. Yes. Bit math is a thing.
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..width {
        if result[i] > max {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    (gamma, epsilon)
}

// Keep only numbers selected by the bit criteria.
// Discard numbers which do not match the bit criteria.
// If you only have one number left, stop; this is the rating value for which you are searching.
// Otherwise, repeat the process, considering the next bit to the right.
fn filter_bits(report: &Vec<String>, start: char, gt: char, eq: char, lt: char) -> String {
    let mut filtered: Vec<String> = report.clone();
    let mut i = 0;
    let mut x = start;

    loop {
        filtered.retain(|y| y.chars().nth(i) == Some(x));
        // println!("{:?} {:?}, {:?}", i, x, filtered);
        if filtered.len() == 1 {
            break;
        }
        i += 1;
        let count: i32 = filtered.iter().map(|line| line.chars().nth(i).unwrap().to_digit(10).unwrap() as i32).sum();
        let length: i32 = filtered.len() as i32;
        x = if count > (length - count) {
                gt
            } else if count == (length - count) {
                eq
            } else  {
                lt
            };
    }

    filtered.get(0).unwrap().to_string()
}

// power consumption = gamma rate * epsilon rate.
// life support rating = oxygen generator rating * CO2 scrubber rating.
fn product(binary1: &String, binary2: &String) -> i32 {
    let x = i32::from_str_radix(binary1, 2).unwrap();
    let y = i32::from_str_radix(binary2, 2).unwrap();
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        let (gamma, epsilon) = count_bits(&input);
        assert_eq!("10110", gamma);
        assert_eq!("01001", epsilon);
        assert_eq!(198, product(&gamma, &epsilon));

        assert_eq!("10111", filter_bits(&input, gamma.chars().nth(0).unwrap(), '1', '1', '0'));
        assert_eq!("01010", filter_bits(&input, epsilon.chars().nth(0).unwrap(), '0', '0', '1'));
    }
}
