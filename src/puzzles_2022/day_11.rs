use itertools::Itertools;

use crate::puzzle_input;
use core::slice::Iter;
use std::collections::VecDeque;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d11-input.txt");
    let mut monkeys = monkey_see(&input);
    let mut part2 = monkeys.clone();
    let n = monkeys.len();

    for _ in 0..20 {
        monkey_business(&mut monkeys, n);
    }

    let lcd: i64 = part2.iter().map(|x| x.test).product();
    for _ in 0..10000 {
        monkey_business_scaled(&mut part2, n, lcd);
    }

    println!("** Part 1 Final: {:?}", level(&monkeys));
    println!("** Part 2 Final: {:?}", level(&part2));
}

const ITEM: &str = "Starting items: ";
const OP: &str = "Operation: new = old ";
const TEST: &str = "Test: divisible by ";
const TEST_TRUE: &str = "If true: throw to monkey ";
const TEST_FALSE: &str = "If false: throw to monkey ";

#[derive(Clone, Debug)]
enum OP {
    Add,
    Multiply,
    Square,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<i64>,
    op: OP,
    op_value: i64,
    test: i64,
    truthy: usize,
    falsy: usize,
    inspected: i64,
}

fn get_monkey(iter: &mut Iter<String>) -> Monkey {
    let items = iter
        .next()
        .unwrap()
        .replace(ITEM, "")
        .split(",")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let ops = iter.next().unwrap().replace(OP, "");
    let mut opsplit = ops.split_ascii_whitespace();
    let (op, op_value) = match opsplit.next() {
        Some("+") => (OP::Add, opsplit.next().unwrap().parse::<i64>().unwrap()),
        Some("*") => match opsplit.next() {
            Some("old") => (OP::Square, 0),
            Some(x) => (OP::Multiply, x.parse::<i64>().unwrap()),
            None => panic!("What is this? {:?}", ops),
        },
        Some(x) => panic!("What is this? {:?}", x),
        None => panic!("What is this? {:?}", ops),
    };

    let test = iter
        .next()
        .unwrap()
        .replace(TEST, "")
        .trim()
        .parse::<i64>()
        .unwrap();
    let truthy = iter
        .next()
        .unwrap()
        .replace(TEST_TRUE, "")
        .trim()
        .parse::<usize>()
        .unwrap();
    let falsy = iter
        .next()
        .unwrap()
        .replace(TEST_FALSE, "")
        .trim()
        .parse::<usize>()
        .unwrap();

    Monkey {
        items,
        op,
        op_value,
        test,
        truthy,
        falsy,
        inspected: 0,
    }
}

fn monkey_see(input: &[String]) -> Vec<Monkey> {
    let mut result = Vec::new();
    let mut iter = input.iter();

    while let Some(line) = iter.next() {
        if line.starts_with("Monkey") {
            result.push(get_monkey(&mut iter));
        }
    }
    result
}

fn monkey_do(monkey: &Monkey, worry_level: i64, lcd: i64) -> (i64, usize) {
    // inspect
    let mut next = match monkey.op {
        OP::Add => worry_level + monkey.op_value,
        OP::Multiply => worry_level * monkey.op_value,
        OP::Square => worry_level * worry_level,
    };
    // get bored
    next = if lcd == 0 { next / 3 } else { next % lcd };
    // test and throw
    (
        next,
        if next % monkey.test == 0 {
            monkey.truthy
        } else {
            monkey.falsy
        },
    )
}

fn monkey_business(monkeys: &mut [Monkey], n: usize) {
    for i in 0..n {
        let mut m1 = monkeys[i].to_owned();
        while let Some(w1) = m1.items.pop_front() {
            m1.inspected += 1;
            let (w2, j) = monkey_do(&m1, w1, 0);
            let mut m2 = monkeys[j].to_owned();
            m2.items.push_back(w2);
            monkeys[j] = m2;
        }
        monkeys[i] = m1;
    }
}

fn monkey_business_scaled(monkeys: &mut [Monkey], n: usize, lcd: i64) {
    for i in 0..n {
        let mut m1 = monkeys[i].to_owned();
        while let Some(w1) = m1.items.pop_front() {
            m1.inspected += 1;
            let (w2, j) = monkey_do(&m1, w1, lcd);
            let mut m2 = monkeys[j].to_owned();
            m2.items.push_back(w2);
            monkeys[j] = m2;
        }
        monkeys[i] = m1;
    }
}

fn level(monkeys: &[Monkey]) -> i64 {
    monkeys
        .iter()
        .map(|x| x.inspected)
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
        "#,
        );

        let mut monkeys = monkey_see(&input);
        let mut part2 = monkeys.clone();
        let n = monkeys.len();

        assert_eq!(monkey_do(&monkeys[0], 79, 0), (500, 3));
        assert_eq!(monkey_do(&monkeys[0], 98, 0), (620, 3));

        assert_eq!(monkey_do(&monkeys[1], 54, 0), (20, 0));
        assert_eq!(monkey_do(&monkeys[1], 65, 0), (23, 0));
        assert_eq!(monkey_do(&monkeys[1], 75, 0), (27, 0));
        assert_eq!(monkey_do(&monkeys[1], 74, 0), (26, 0));

        assert_eq!(monkey_do(&monkeys[2], 79, 0), (2080, 1));
        assert_eq!(monkey_do(&monkeys[2], 60, 0), (1200, 3));
        assert_eq!(monkey_do(&monkeys[2], 97, 0), (3136, 3));

        assert_eq!(monkey_do(&monkeys[3], 74, 0), (25, 1));
        assert_eq!(monkey_do(&monkeys[3], 500, 0), (167, 1));
        assert_eq!(monkey_do(&monkeys[3], 620, 0), (207, 1));
        assert_eq!(monkey_do(&monkeys[3], 1200, 0), (401, 1));
        assert_eq!(monkey_do(&monkeys[3], 3136, 0), (1046, 1));

        for _ in 0..20 {
            monkey_business(&mut monkeys, n);
        }
        assert_eq!(monkeys[0].inspected, 101);
        assert_eq!(monkeys[1].inspected, 95);
        assert_eq!(monkeys[2].inspected, 7);
        assert_eq!(monkeys[3].inspected, 105);

        assert_eq!(level(&monkeys), 10605);

        let lcd: i64 = monkeys.iter().map(|x| x.test).product();

        monkey_business_scaled(&mut part2, n, lcd);
        assert_eq!(part2[0].inspected, 2);
        assert_eq!(part2[1].inspected, 4);
        assert_eq!(part2[2].inspected, 3);
        assert_eq!(part2[3].inspected, 6);

        for _ in 1..20 {
            monkey_business_scaled(&mut part2, n, lcd);
        }
        assert_eq!(part2[0].inspected, 99);
        assert_eq!(part2[1].inspected, 97);
        assert_eq!(part2[2].inspected, 8);
        assert_eq!(part2[3].inspected, 103);

        for _ in 20..1000 {
            monkey_business_scaled(&mut part2, n, lcd);
        }
        assert_eq!(part2[0].inspected, 5204);
        assert_eq!(part2[1].inspected, 4792);
        assert_eq!(part2[2].inspected, 199);
        assert_eq!(part2[3].inspected, 5192);

        for _ in 1000..10000 {
            monkey_business_scaled(&mut part2, n, lcd);
        }
        assert_eq!(part2[0].inspected, 52166);
        assert_eq!(part2[1].inspected, 47830);
        assert_eq!(part2[2].inspected, 1938);
        assert_eq!(part2[3].inspected, 52013);
    }
}
