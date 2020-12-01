use crate::puzzle_input;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d01-input1.txt");
    let expenses: Vec<i64> = input.iter()
                                .map(|x| x.trim().parse::<i64>().unwrap())
                                .collect();

    println!("** Part 1 Final: {:?}", check_two_expenses(&expenses));
    println!("** Part 2 Final: {:?}", check_three_expenses(&expenses));
}

fn check_two_expenses(expenses: &Vec<i64>) -> i64 {
    for (i, &x) in expenses.iter().enumerate() {
        let next = i + 1;
        let slice = &expenses[next..];
        for y in slice.iter() {
            if x + y == 2020 {
                return x * y
            }
        }
    }
    0
}

fn check_three_expenses(expenses: &Vec<i64>) -> i64 {
    for (i, &x) in expenses.iter().enumerate() {
        let m = i + 1;
        let s1 = &expenses[m..];
        for (j, &y) in s1.iter().enumerate() {
            let n = j + 1;
            let s2 = &expenses[n..];
            for z in s2.iter() {
                if x + y + z == 2020 {
                    return x * y * z
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
    fn test_check_expenses() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(check_two_expenses(&input), 514579);
        assert_eq!(check_three_expenses(&input), 241861950);
    }
}