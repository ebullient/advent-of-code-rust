use crate::puzzle_input;
use std::collections::HashSet;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d06-input1.txt");
    println!("** Part 1 Final: {:?}", count_answers(&input));
    println!("** Part 2 Final: {:?}", count_intersection_answers(&input));
}

fn count_answers(batch: &Vec<String>) -> i32 {
    let empty: Vec<String> = vec!["".to_string()];
    let state: HashSet<char> = HashSet::new();

    // chain an empty line at the end
    batch
        .iter()
        .chain(empty.iter())
        .scan(state, |state, x| {
            if x.is_empty() {
                let result = state.len();
                state.clear();
                Some(result as i32)
            } else {
                state.extend(x.chars());
                Some(0)
            }
        })
        .fold(0, |acc, x| acc + x)
}

#[derive(Debug)]
struct IntersectState {
    set: HashSet<char>,
    entry: bool,
}
impl IntersectState {
    pub fn clear(&mut self) {
        self.set.clear();
        self.entry = false;
    }

    pub fn intersect(&mut self, str: &String) {
        if self.entry == false {
            self.set.extend(str.chars());
            self.entry = true;
        } else {
            let u: HashSet<char> = self.set.clone();
            let v: HashSet<char> = str.chars().collect();
            let i: HashSet<char> = u.intersection(&v).map(|x| *x).collect();
            self.set.clear();
            self.set.extend(i);
        }
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }
}

fn count_intersection_answers(batch: &Vec<String>) -> i32 {
    let empty: Vec<String> = vec!["".to_string()];
    let state: IntersectState = IntersectState {
        set: HashSet::new(),
        entry: false,
    };

    // chain an empty line at the end
    batch
        .iter()
        .chain(empty.iter())
        .scan(state, |state, x| {
            if x.is_empty() {
                let result = state.len();
                state.clear();
                Some(result as i32)
            } else {
                state.intersect(x);
                Some(0)
            }
        })
        .fold(0, |acc, x| acc + x)
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
