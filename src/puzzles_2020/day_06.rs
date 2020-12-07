use crate::puzzle_input;
use std::collections::HashSet;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d06-input1.txt");
    println!("** Part 1 Final: {:?}", count_answers(&input));
    println!("** Part 2 Final: {:?}", count_intersection_answers(&input));
}

fn count_answers(batch: &Vec<String>) -> i32 {
    let mut set: HashSet<char> = HashSet::new();
    batch
        .split(|x| x.is_empty())
        .map(|group| {
            set.clear();
            group.iter().for_each(|x| set.extend(x.chars()));
            set.len()
        })
        .sum::<usize>() as i32
}

fn count_intersection_answers(batch: &Vec<String>) -> i32 {
    batch
        .split(|x| x.is_empty())
        .map(|group| {
            let mut set: HashSet<char> = group[0].chars().collect();
            group.iter().for_each(|x| {
                let v: HashSet<char> = x.chars().collect();
                set = set.intersection(&v).map(|x| *x).collect();
            });
            set.len()
        })
        .sum::<usize>() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_answers() {
        let input = puzzle_input::read_all_lines("./input/2020-d06-test.txt");
        assert_eq!(count_answers(&input), 11);
        assert_eq!(count_intersection_answers(&input), 6);
    }
}
