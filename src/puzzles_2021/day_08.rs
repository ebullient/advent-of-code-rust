use crate::puzzle_input;
use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d08-input.txt");
    let parsed = parse(&input);

    println!("** Part 1 Final: {:?}", count_unique(&parsed));
    println!("** Part 2 Final: {:?}", compute(&parsed));
}

fn count_unique(input: &Vec<(Vec<String>, Vec<String>)>) -> i32 {
    let count = input
        .iter()
        .flat_map(|x| x.1.iter())
        .fold(0, |acc, x| match x.len() {
            2 | 3 | 4 | 7 => acc + 1,
            _ => acc,
        });
    count
}

//    ONE: [usize; 2]   = [      2,       5   ];
//    SEVEN: [usize; 3] = [0,    2,       5   ];
//    FOUR: [usize; 4]  = [   1, 2, 3,    5   ];
//    TWO: [usize; 5]   = [0,    2, 3, 4,    6];
//    THREE: [usize; 5] = [0,    2, 3,    5, 6];
//    FIVE: [usize; 5]  = [0, 1,    3,    5, 6];
//    SIX: [usize; 6]   = [0, 1,    3, 4, 5, 6];
//    ZERO: [usize; 6]  = [0, 1, 2,    4, 5, 6];
//    NINE: [usize; 6]  = [0, 1, 2, 3,    5, 6];

fn decipher(input: &Vec<String>) -> HashMap<char, char> {
    let mut possible = vec![String::from("abcdefg"); 7];
    let mut actual = [' '; 7];

    // input[0] has length 2 (wires 2 and 5)
    // input of length 6 ALL have wire 5 (but not wire 2): pin wire 5
    possible[5].retain(|c| {
        input[0].contains(c) && input.iter().filter(|x| x.len() == 6).all(|x| x.contains(c))
    });
    actual[5] = possible[5].chars().next().unwrap();

    // now that we know wire 5, we know wire 2
    possible[2].retain(|c| input[0].contains(c) && !actual.contains(&c));
    actual[2] = possible[2].chars().next().unwrap();

    // input[1] has lenth 3: pin down wire 0
    possible[0].retain(|c| input[1].contains(c) && !actual.contains(&c));
    actual[0] = possible[0].chars().next().unwrap();

    // input length > 4 all have both wires 0 and 6: pin down wire 6
    possible[6].retain(|c| {
        input.iter().filter(|x| x.len() > 4).all(|x| x.contains(c)) && !actual.contains(&c)
    });
    actual[6] = possible[6].chars().next().unwrap();

    // input[2] has length 4 (wires 1 and 3)
    // input length == 5: all will have wire 3 (not wire 1)
    possible[3].retain(|c| {
        input[2].contains(c)
            && input.iter().filter(|x| x.len() == 5).all(|x| x.contains(c))
            && !actual.contains(&c)
    });
    actual[3] = possible[3].chars().next().unwrap();

    // because we know wire 3, we now know wire 1
    possible[1].retain(|c| input[2].contains(c) && !actual.contains(&c));
    actual[1] = possible[1].chars().next().unwrap();

    // Only one left
    possible[4].retain(|c| !input[0].contains(c) && !actual.contains(&c));
    actual[4] = possible[4].chars().next().unwrap();

    let mut map = HashMap::new();
    for (i, x) in possible.iter().enumerate() {
        map.insert(
            x.chars().next().unwrap(),
            char::from_digit(i as u32, 10).unwrap(),
        );
    }
    map
}

fn decode(map: &HashMap<char, char>, input: &Vec<String>) -> i32 {
    let mut number = String::new();
    for code in input {
        let wires: String = code
            .chars()
            .map(|x| *map.get(&x).unwrap())
            .sorted()
            .collect();

        match wires.as_str() {
            "25" => number.push('1'),
            "025" => number.push('7'),
            "1235" => number.push('4'),
            "02346" => number.push('2'),
            "02356" => number.push('3'),
            "01356" => number.push('5'),
            "013456" => number.push('6'),
            "012456" => number.push('0'),
            "012356" => number.push('9'),
            "0123456" => number.push('8'),
            _ => panic!(),
        }
    }
    number.parse::<i32>().unwrap()
}

fn compute(input: &Vec<(Vec<String>, Vec<String>)>) -> i32 {
    let mut total = 0;
    for row in input {
        let single_map = decipher(&row.0);
        total += decode(&single_map, &row.1);
    }
    total
}

// Just parse each line into sorted bits
fn parse(input: &Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    let mut result = vec![];
    for line in input {
        let mut patterns = vec![];
        let mut values = vec![];
        let mut pipe = false;
        line.split_whitespace().for_each(|x| {
            if x == "|" {
                pipe = true;
            } else if pipe {
                values.push(x.chars().sorted().collect::<String>())
            } else {
                patterns.push(x.chars().sorted().collect::<String>());
            }
        });
        patterns.sort_by(|a, b| a.len().cmp(&b.len()));
        result.push((patterns, values));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let single_test: Vec<String> = vec![String::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )];
        let single_parsed = parse(&single_test);
        let single_map = decipher(&single_parsed[0].0);
        let single_result = decode(&single_map, &single_parsed[0].1);
        assert_eq!(5353, single_result);

        let input: Vec<String> = puzzle_input::split_string(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        );

        let parsed = parse(&input);
        let result = count_unique(&parsed);
        assert_eq!(26, result);

        let total = compute(&parsed);
        assert_eq!(61229, total);
    }
}
