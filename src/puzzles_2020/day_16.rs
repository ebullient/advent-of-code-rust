extern crate lazy_static;
extern crate regex;

use crate::puzzle_input;
use std::ops::RangeInclusive;
use regex::Regex;
use regex::Match;
use std::collections::HashMap;

pub fn run() {
    let input = puzzle_input::read_string("./input/2020-d16-input1.txt");
    let (rules, my_ticket, mut tickets) = parse_input(&input);

    println!("** Part 1 Final: {:?}", validate(&rules, &tickets));
    tickets.retain(|x| x.validate(&rules) == 0);


    println!("** Part 2 Final: {:?}", 0);
}

#[derive(Debug)]
struct Rules {
    bounds: HashMap<String, Vec<RangeInclusive<u32>>>
}
impl Rules {
    fn new(input: &str) -> Rules {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }
        let mut result = Rules {
            bounds: HashMap::new()
        };
        for line in input.lines() {
            if let Some(x) = RE.captures(line.trim()) {
                let name = x.get(1).unwrap().as_str();
                let bound1 = std::ops::RangeInclusive::new(
                        to_u32(x.get(2)), to_u32(x.get(3)));
                let bound2 = std::ops::RangeInclusive::new(
                        to_u32(x.get(4)), to_u32(x.get(5)));
                result.bounds.insert(name.to_string(), vec![bound1, bound2]);
            }
        }
        result
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>
}
impl Ticket {
    fn new(input: &str) -> Ticket {
        Ticket {
            fields: input.trim().split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        }
    }

    fn validate(&self, rules: &Rules) -> u32 {
        let mut remaining = self.fields.clone();
        for rule in rules.bounds.values().flat_map(|x| x.iter()) {
            remaining = remaining.iter()
                .filter(|x| !rule.contains(x))
                .map(|x| *x)
                .collect();
        }
        remaining.iter().sum::<u32>()
    }
}

fn to_u32(s: Option<Match<>>) -> u32 {
    s.unwrap().as_str().parse::<u32>().unwrap()
}

fn parse_input(input: &str) -> (Rules, Ticket, Vec<Ticket>) {
    let mut chunks = input.split("\n\n");
    let rules = Rules::new(chunks.next().unwrap());

    let line = chunks.next().unwrap().lines().last().unwrap();
    let my_ticket = Ticket::new(line);

    let tickets: Vec<Ticket> = chunks.next().unwrap().lines()
            .map(|x| x.trim())
            .filter(|x| !x.contains("tickets"))
            .map(|x| Ticket::new(x))
            .collect();

    (rules, my_ticket, tickets)
}

fn validate(rules: &Rules, tickets: &Vec<Ticket>) -> u32 {
    tickets.iter()
        .map(|x| x.validate(&rules))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ticket() {
        let input=
            "class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50

            your ticket:
            7,1,14

            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12";

        let (rules, my_ticket, mut tickets) = parse_input(input);
        println!("{:?}", tickets);

        tickets.retain(|x| x.validate(&rules) == 0);
        println!("{:?}", tickets);

    }
}
