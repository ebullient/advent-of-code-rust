use crate::puzzle_input;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2021-d01-input.txt");
    let report: Vec<i32> = input
        .iter()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let increased = sonar_scan(&report);
    let increased_sum = sonar_window_scan(&report);

    println!("** Part 1 Final: {:?}", increased);
    println!("** Part 2 Final: {:?}", increased_sum);
}

fn sonar_scan(report: &[i32]) -> i32 {
    let mut previous = 0;
    let mut increase = 0;

    for reading in report {
        if previous != 0 && *reading > previous {
            increase += 1;
        }
        previous = *reading;
    }

    increase
}

fn sonar_window_scan(report: &[i32]) -> i32 {
    let mut previous = 0;
    let mut increase = 0;

    for i in 2..(report.len()) {
        let sum = report[i] + report[i - 1] + report[i - 2];
        if previous != 0 && sum > previous {
            increase += 1;
        }
        previous = sum;
    }

    increase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<i32> = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263"
        .split('\n')
        .map(|x| x.to_string())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

        let increases = sonar_scan(&input);
        assert_eq!(increases, 7);

        let increases = sonar_window_scan(&input);
        assert_eq!(increases, 5)
    }
}
