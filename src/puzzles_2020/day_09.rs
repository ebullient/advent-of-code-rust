use crate::puzzle_input;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d09-input1.txt")
        .iter()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    println!("** Part 1 Final: {:?}", find_weakness(&input, 25));
    println!("** Part 2 Final: {:?}", find_match(&input, 105950735));
}

fn find_weakness(seq: &Vec<i64>, n: usize) -> i64 {
    let mut min = 0;
    let mut max = n;
    for i in n..seq.len() {
        // this will try all combinations (no early exit)
        if 0 == seq[min..max].iter().combinations(2)
                .map(|x| x[0] + x[1])
                .filter(|x| *x == seq[i])
                .count() {
                    return seq[i];
                }

        min += 1;
        max += 1;
    }
    0
}

fn find_match(seq: &Vec<i64>, target: i64) -> i64 {
    for range in 2..seq.len() {
        for x in seq.windows(range) {
            if target == x.iter().sum::<i64>() {
                if let MinMax(x, y) = x.iter().minmax() {
                    return x + y
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576".split('\n')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        assert_eq!(find_weakness(&input, 5), 127);
        assert_eq!(find_match(&input, 127), 62);
    }
}
