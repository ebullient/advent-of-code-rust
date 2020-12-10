use crate::puzzle_input;
use std::collections::HashMap;

pub fn run() {
    let mut input: Vec<i32> = puzzle_input::read_all_lines("./input/2020-d10-input1.txt")
        .iter()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    println!("** Part 1 Final: {:?}", find_distribution(&mut input));
    println!("** Part 2 Final: {:?}", count_all_combinations(&input));
}

fn count_all_combinations(list: &Vec<i32>) -> i64 {
    let mut map: HashMap<usize, i64> = HashMap::new();
    count_paths(&list, &mut map, 0)
}

fn count_paths(list: &Vec<i32>, map: &mut HashMap<usize, i64>, i: usize) -> i64 {
    // Last element. There is only one path to it.
    // Backtrack begins here
    if i == list.len() - 1 {
        return 1;
    }
    // Avoid re-doing work
    if let Some(x) = map.get(&i) {
        return *x;
    }
    // DFS traversal along valid paths
    // list is in sorted order.. so as soon as the diffs are > 3,
    // we've run out of valid values
    let mut total: i64 = 0;
    for j in i + 1..list.len() {
        if list[j] - list[i] <= 3 {
            total += count_paths(list, map, j);
        } else {
            break;
        }
    }
    // remember for later
    map.insert(i, total);
    total
}

fn find_distribution(list: &mut Vec<i32>) -> Result<i32, String> {
    list.sort();
    list.insert(0, 0);
    list.push(list.last().unwrap() + 3);

    let mut result = (0, 0);
    for n in 1..list.len() {
        let x = list[n] - list[n - 1];
        if x == 1 {
            result.0 += 1;
        } else if x == 3 {
            result.1 += 1;
        } else {
            return Err(format!(
                "Unexpected difference {:?} between elements at {:?}; {:?}",
                x, n, list
            ));
        }
    }

    println!("{:?}", result);
    Ok(result.0 * result.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jolt_example_1() {
        let mut input: Vec<i32> = "16
            10
            15
            5
            1
            11
            7
            19
            6
            12
            4"
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

        assert_eq!(find_distribution(&mut input), Ok(35));
        assert_eq!(count_all_combinations(&input), 8);
    }

    #[test]
    fn test_jolt_example_2() {
        let mut input: Vec<i32> = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3"
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

        assert_eq!(find_distribution(&mut input), Ok(220));
        assert_eq!(count_all_combinations(&input), 19208);
    }
}
