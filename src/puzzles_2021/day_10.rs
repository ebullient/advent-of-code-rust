use crate::puzzle_input;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d10-input.txt");

    let parsed: Vec<(Option<char>, Option<Vec<char>>)> = input.iter().map(|x| parse(x)).collect();

    println!(
        "** Part 1 Final: {:?}",
        parsed.iter().map(|x| corrupted_score(x.0)).sum::<i32>()
    );

    let mut scores: Vec<i64> = parsed
        .iter()
        .filter(|x| x.1 != None)
        .map(|x| completed_score(&x.1))
        .collect();
    scores.sort_unstable();
    println!("** Part 2 Final: {:?}", scores[scores.len() / 2]);
}

fn parse(input: &str) -> (Option<char>, Option<Vec<char>>) {
    let mut expected: Vec<char> = vec![];
    for c in input.chars() {
        match c {
            '[' => expected.push(']'),
            '(' => expected.push(')'),
            '<' => expected.push('>'),
            '{' => expected.push('}'),
            ']' | ')' | '>' | '}' => {
                if expected.pop() != Some(c) {
                    return (Some(c), None);
                }
            }
            _ => panic!(),
        };
    }
    expected.reverse();
    (
        None,
        if expected.is_empty() {
            None
        } else {
            Some(expected)
        },
    )
}

fn corrupted_score(bad: Option<char>) -> i32 {
    match bad {
        None => 0,
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        _ => panic!(),
    }
}

fn completed_score(remaining: &Option<Vec<char>>) -> i64 {
    if let Some(list) = remaining {
        list.iter()
            .map(|c| match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            })
            .inspect(|s| println!("score {:?}", s))
            .fold(0, |acc, s| acc * 5 + s)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string(input: Option<Vec<char>>) -> String {
        match input {
            None => String::from(""),
            Some(x) => x.iter().cloned().collect(),
        }
    }

    fn to_vec(input: &str) -> Option<Vec<char>> {
        Some(input.chars().collect())
    }

    #[test]
    fn test() {
        assert_eq!(Some(']'), parse("(]").0);
        assert_eq!(Some('>'), parse("{()()()>").0);
        assert_eq!(Some('}'), parse("(((()))}").0);
        assert_eq!(Some(')'), parse("<([]){()}[{}])").0);

        assert_eq!(None, parse("()").0);
        assert_eq!(None, parse("[]").0);
        assert_eq!(None, parse("([])").0);
        assert_eq!(None, parse("<([{}])>").0);

        assert_eq!("}}]])})]", to_string(parse("[({(<(())[]>[[{[]{<()<>>").1));
        assert_eq!(")}>]})", to_string(parse("[(()[<>])]({[<{<<[]>>(").1));
        assert_eq!("}}>}>))))", to_string(parse("(((({<>}<{<{<>}{[]{[]{}").1));
        assert_eq!("]]}}]}]}>", to_string(parse("{<[[]]>}<{[{[{[]{()[[[]").1));
        assert_eq!("])}>", to_string(parse("<{([{{}}[<[[[<>{}]]]>[]]").1));

        assert_eq!(288957, completed_score(&to_vec("}}]])})]")));
        assert_eq!(5566, completed_score(&to_vec(")}>]})")));
        assert_eq!(1480781, completed_score(&to_vec("}}>}>))))")));
        assert_eq!(995444, completed_score(&to_vec("]]}}]}]}>")));
        assert_eq!(294, completed_score(&to_vec("])}>")));

        let input: Vec<String> = puzzle_input::split_string(
            "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]",
        );

        let parsed: Vec<(Option<char>, Option<Vec<char>>)> =
            input.iter().map(|x| parse(x)).collect();

        assert_eq!(26397, parsed.iter().map(|x| corrupted_score(x.0)).sum());

        let mut scores: Vec<i64> = parsed
            .iter()
            .filter(|x| x.1 != None)
            .map(|x| completed_score(&x.1))
            .collect();
        scores.sort_unstable();
        println!("{:?}", scores);
        assert_eq!(288957, scores[scores.len() / 2]);
    }
}
