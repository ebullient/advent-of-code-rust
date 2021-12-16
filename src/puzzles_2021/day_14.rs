use crate::puzzle_input;
use itertools::Itertools;
use std::collections::HashMap;
use std::slice::Iter;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d14-input.txt");
    let (mut totals, rules) = parse_rules(&input);

    for i in 0..40 {
        if i == 10 {
            println!("** Part 1 Final: {:?}", totals.score());
        }
        totals = step(&totals, &rules);
    }
    println!("** Part 2 Final: {:?}", totals.score());
}

#[derive(Clone, Debug, PartialEq)]
struct Rules {
    data: HashMap<String, char>,
}
impl Rules {
    fn new(iter: &mut Iter<String>) -> Rules {
        let mut data = HashMap::new();
        while let Some(line) = iter.next() {
            if line.contains(" -> ") {
                data.insert(line[..2].to_string(), line[6..].chars().next().unwrap());
            }
        }

        Rules { data: data }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Totals {
    data: HashMap<String, usize>,
    letters: HashMap<char, usize>,
}
impl Totals {
    fn new(start: &str) -> Totals {
        let data = start
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut acc, (a, b)| {
                let pair = format!("{}{}", a, b);
                let count = acc.entry(pair).or_insert(0);
                *count += 1;
                acc
            });

        let letters = start.chars().fold(HashMap::new(), |mut acc, c| {
            let i = acc.entry(c).or_insert(0);
            *i += 1;
            acc
        });

        Totals {
            data: data,
            letters: letters,
        }
    }

    fn inc_letter(&mut self, letter: char, n: usize) {
        let counter = self.letters.entry(letter).or_insert(0);
        *counter += n;
    }

    fn inc_pair(&mut self, pair: String, n: usize) {
        let counter = self.data.entry(pair).or_insert(0);
        *counter += n;
    }

    fn score(&self) -> usize {
        self.letters.values().max().unwrap() - self.letters.values().min().unwrap()
    }
}

fn parse_rules(input: &Vec<String>) -> (Totals, Rules) {
    let mut i = input.iter();
    let totals = Totals::new(i.next().unwrap()); // start with first line
    let rules = Rules::new(&mut i); // parse the rest

    (totals, rules)
}

fn step(totals: &Totals, rules: &Rules) -> Totals {
    let mut next = Totals::new("");
    next.letters = totals.letters.clone();

    for (pair, count) in &totals.data {
        if let Some(v) = rules.data.get(pair) {
            let mut i = pair.chars();
            next.inc_pair(format!("{}{}", i.next().unwrap(), v), *count);
            next.inc_pair(format!("{}{}", v, i.next().unwrap()), *count);
            next.inc_letter(*v, *count);
        } else {
            panic!("pair wasn't found in the rules");
        }
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C",
        );

        let (mut totals, rules) = parse_rules(&input);

        totals = step(&totals, &rules);
        let mut ex_totals = Totals::new("NCNBCHB");
        assert_eq!(ex_totals, totals);

        totals = step(&totals, &rules);
        ex_totals = Totals::new("NBCCNBBBCBHCB");
        assert_eq!(ex_totals, totals);

        totals = step(&totals, &rules);
        ex_totals = Totals::new("NBBBCNCCNBBNBNBBCHBHHBCHB");
        assert_eq!(ex_totals, totals);

        totals = step(&totals, &rules);
        ex_totals = Totals::new("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
        assert_eq!(ex_totals, totals);

        for _ in 4..10 {
            totals = step(&totals, &rules);
        }
        assert_eq!(1588, totals.score());

        for _ in 10..40 {
            totals = step(&totals, &rules);
        }
        assert_eq!(2188189693529, totals.score());
    }
}
