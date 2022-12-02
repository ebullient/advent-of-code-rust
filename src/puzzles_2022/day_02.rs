use crate::puzzle_input;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d02-input.txt");

    println!("** Part 1 Final: {:?}", score_contest(&input));
    println!("** Part 2 Final: {:?}", score_contest_2(&input));
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn convert(label: &str) -> Shape {
    match label {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!(),
    }
}

fn find_shape(a: &Shape, win_loss: &str) -> Shape {
    match win_loss {
        // lose
        "X" => match a {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        // draw
        "Y" => a.clone(),
        // win
        "Z" => match a {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        _ => panic!(),
    }
}

fn check_win(a: Shape, b: Shape) -> bool {
    if (a == Shape::Rock && b == Shape::Scissors)
        || (a == Shape::Paper && b == Shape::Rock)
        || (a == Shape::Scissors && b == Shape::Paper)
    {
        return true;
    }

    false
}

fn score_round(first: Shape, second: Shape) -> i32 {
    let mut score = match second {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    if first == second {
        score += 3;
    } else if check_win(second, first) {
        score += 6;
    }

    score
}

fn score_contest(input: &[String]) -> i32 {
    let mut total = 0;

    for line in input {
        if line.is_empty() {
            continue;
        }
        let mut split = line.split_whitespace();
        let first = convert(split.next().unwrap());
        let second = convert(split.next().unwrap());
        total += score_round(first, second);
    }

    total
}

fn score_contest_2(input: &[String]) -> i32 {
    let mut total = 0;

    for line in input {
        if line.is_empty() {
            continue;
        }
        let mut split = line.split_whitespace();
        let first = convert(split.next().unwrap());
        let second = find_shape(&first, split.next().unwrap());
        total += score_round(first, second);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = "A Y
        B X
        C Z
        "
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect();

        assert_eq!(score_round(Shape::Rock, Shape::Paper), 8);
        assert_eq!(score_round(Shape::Paper, Shape::Rock), 1);
        assert_eq!(score_round(Shape::Scissors, Shape::Scissors), 6);

        assert_eq!(score_contest(&input), 15);
    }

    #[test]
    fn test_2() {
        let mut shape = convert("A");
        assert_eq!(score_round(shape, find_shape(&shape, "Y")), 4);
        shape = convert("B");
        assert_eq!(score_round(shape, find_shape(&shape, "X")), 1);
        shape = convert("C");
        assert_eq!(score_round(shape, find_shape(&shape, "Z")), 7);

        let input: Vec<String> = "A Y
        B X
        C Z
        "
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect();

        assert_eq!(score_contest_2(&input), 12);
    }
}
