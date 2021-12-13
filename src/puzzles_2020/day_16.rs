use crate::puzzle_input;
use regex::Match;
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

pub fn run() {
    let input = puzzle_input::read_string("./input/2020-d16-input1.txt");
    let (mut rules, my_ticket, mut tickets) = parse_input(&input);

    println!("** Part 1 Final: {:?}", validate(&rules, &tickets));
    tickets.retain(|x| {
        let (count, _) = x.validate(&rules);
        count == 0
    });
    // Keep for later
    let my_values = my_ticket.fields.clone();

    // Push my ticket into the list
    tickets.push(my_ticket);
    rules.find_eligible_fields(&tickets);
    let field_order = rules.find_field_order();

    println!("** Part 2 Final: {:?}", field_order.iter().enumerate()
        .filter(|(_, e)| e.starts_with("departure"))
        .map(|(i, _)| my_values[i] as u64)
        .product::<u64>());
}

fn parse_input(input: &str) -> (Rules, Ticket, Vec<Ticket>) {
    let mut chunks = input.split("\n\n");
    let rules = Rules::new(chunks.next().unwrap());

    let line = chunks.next().unwrap().lines().last().unwrap();
    let my_ticket = Ticket::new(line);

    let tickets: Vec<Ticket> = chunks
        .next()
        .unwrap()
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.contains("tickets"))
        .map(|x| Ticket::new(x))
        .collect();

    (rules, my_ticket, tickets)
}

fn to_u32(s: Option<Match>) -> u32 {
    s.unwrap().as_str().parse::<u32>().unwrap()
}

fn validate(rules: &Rules, tickets: &Vec<Ticket>) -> u32 {
    tickets.iter().map(|x| {
        let (_, sum) = x.validate(&rules);
        sum
    }).sum()
}

#[derive(Debug)]
struct Rules {
    bounds: HashMap<String, Vec<RangeInclusive<u32>>>,
    placement: HashMap<String, Vec<usize>>,
    num_fields: usize
}
impl Rules {
    fn new(input: &str) -> Rules {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([^:]*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }
        let mut result = Rules {
            bounds: HashMap::new(),
            placement: HashMap::new(),
            num_fields: 0
        };
        for line in input.lines() {
            if let Some(x) = RE.captures(line.trim()) {
                let name = x.get(1).unwrap().as_str();
                let bound1 = std::ops::RangeInclusive::new(to_u32(x.get(2)), to_u32(x.get(3)));
                let bound2 = std::ops::RangeInclusive::new(to_u32(x.get(4)), to_u32(x.get(5)));
                result.bounds.insert(name.to_string(), vec![bound1, bound2]);
            }
        }
        result
    }

    fn find_eligible_fields(&mut self, tickets: &Vec<Ticket>) {
        self.num_fields = tickets[0].fields.len();
        let all_tickets = tickets.len();
        for (key, b) in self.bounds.iter() {
            let ok_index = self.placement.entry(key.to_string()).or_insert(vec![]);
            for i in 0 .. self.num_fields {
                let valid = tickets.iter()
                    .map(|x| x.fields[i])
                    .filter(|x| b.iter().any(|y| y.contains(x)))
                    .count();

                if valid == all_tickets {
                    ok_index.push(i);
                }
            }
        }
    }

    fn find_field_order(&mut self) -> Vec<String> {
        let mut field_names: Vec<String> = vec!["".to_string(); self.num_fields];
        for _ in 0 .. self.num_fields {
            let name: String;
            let field: usize;
            if let Some(entry) = self.placement.iter().find(|(_, y)| y.len() == 1) {
                name = entry.0.to_string();
                field = entry.1[0];
            } else {
                println!("Unable to find a field with only one remaining match");
                break;
            }

            // end borrow.. .. now mutate to remove
            self.placement.retain(|x, y| {
                y.retain(|c| *c != field);
                *x != name
            });
            field_names[field] = name;
        }
        field_names
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>,
}
impl Ticket {
    fn new(input: &str) -> Ticket {
        Ticket {
            fields: input
                .trim()
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
        }
    }

    fn validate(&self, rules: &Rules) -> (u32, u32) {
        let result: (u32, u32) = (0, 0);
        self.fields.iter()
            .filter(|x| ! rules.bounds.values().flat_map(|y| y.iter()).any(|y| y.contains(x)))
            .fold(result, |(mut count, mut sum), x| {
                count += 1;
                sum += x;
                (count, sum)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ticket() {
        let input = "class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50

            your ticket:
            7,1,14

            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12";

        let (rules, _, tickets) = parse_input(input);
        assert_eq!(validate(&rules, &tickets), 71);
    }

    #[test]
    fn test_validate_fields() {
        let input = "class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9";
        let (mut rules, my_ticket, mut tickets) = parse_input(input);

        tickets.push(my_ticket);
        rules.find_eligible_fields(&tickets);
        let field_order = rules.find_field_order();
        assert_eq!(field_order, vec!["row", "class", "seat"]);
    }
}
