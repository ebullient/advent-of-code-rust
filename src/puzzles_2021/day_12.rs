use crate::puzzle_input;
use regex::Regex;
use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d12-input.txt");
    let g = parse(&input);
    println!("** Part 1 Final: {:?}", count_paths(&g));
    println!("** Part 2 Final: {:?}", count_paths_more_caves(&g));
}

fn parse(input: &Vec<String>) -> UnGraphMap<&str, i32> {
    let mut g = UnGraphMap::new();
    for line in input {
        let mut i = line.split('-');
        let a = i.next().unwrap();
        let b = i.next().unwrap();
        g.add_edge(a, b, 1);
    }
    g
}

fn count_paths_more_caves(graph: &UnGraphMap<&str, i32>) -> usize {
    let mut current: Vec<String> = vec![];
    let mut all: Vec<Vec<String>> = vec![];
    let mut visitor = PermissiveVisitor::new();

    current.push(String::from("start"));
    dfs(&graph, &mut visitor, &mut current, &mut all, "start", "end");
    all.len()
}

fn count_paths(graph: &UnGraphMap<&str, i32>) -> usize {
    let mut current: Vec<String> = vec![];
    let mut all: Vec<Vec<String>> = vec![];
    let mut visitor = DefaultVisitor::new();

    current.push(String::from("start"));
    dfs(&graph, &mut visitor, &mut current, &mut all, "start", "end");
    all.len()
}

fn dfs(graph: &UnGraphMap<&str, i32>, visitor: &mut dyn Visitor,
    current: &mut Vec<String>, all: &mut Vec<Vec<String>>, b: &str, end: &str) {

    visitor.set_visiting(b, true);
    if b == end {
        all.push(current.to_vec());
    } else {
        for n in graph.neighbors(b) {
            if !visitor.is_visiting(n) {
                current.push(n.to_string());
                dfs(graph, visitor, current, all, n, end);
                current.pop();
            }
        }
    }
    visitor.set_visiting(b, false);
}

pub trait Visitor {
    fn set_visiting(&mut self, n: &str, visit: bool);
    fn is_visiting(&mut self, n: &str) -> bool;
}

lazy_static! {
    static ref LOWER: Regex = Regex::new(r"[a-z]+").unwrap();
}

#[derive(Clone, Debug)]
pub struct DefaultVisitor {
    visiting: HashMap<String, bool>
}
impl DefaultVisitor {
    pub fn new() -> DefaultVisitor {
        DefaultVisitor {
            visiting: HashMap::new()
        }
    }
}
impl Visitor for DefaultVisitor {
    fn set_visiting(&mut self, n: &str, visit: bool) {
        if LOWER.is_match(n) {
            self.visiting.insert(n.to_string(), visit);
        }
    }

    fn is_visiting(&mut self, n: &str) -> bool {
        if let Some(x) = self.visiting.get(n) {
            return *x
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct PermissiveVisitor {
    visiting: HashMap<String, bool>,
    twice: Option<String>
}
impl PermissiveVisitor {
    pub fn new() -> PermissiveVisitor {
        PermissiveVisitor {
            visiting: HashMap::new(),
            twice: None
        }
    }
}
impl Visitor for PermissiveVisitor {
    fn set_visiting(&mut self, n: &str, visit: bool) {
        if LOWER.is_match(n) {
            if visit {
                if let Some(prev) = self.visiting.insert(n.to_string(), visit) {
                    if prev {
                        self.twice = Some(n.to_string());
                    }
                }
            } else if Some(n.to_string()) == self.twice {
                self.twice = None;
            } else {
                self.visiting.insert(n.to_string(), visit);
            }
        }
    }

    fn is_visiting(&mut self, n: &str) -> bool {
        if "start" == n {
            return true
        }
        if let Some(x) = self.visiting.get(n) {
            return *x && self.twice != None
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string("start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end");
        let g = parse(&input);
        assert_eq!(10, count_paths(&g));
        assert_eq!(36, count_paths_more_caves(&g));

        let input: Vec<String> = puzzle_input::split_string("dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc");
        let g = parse(&input);
        assert_eq!(19, count_paths(&g));
        assert_eq!(103, count_paths_more_caves(&g));

        let input: Vec<String> = puzzle_input::split_string("fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW");
        let g = parse(&input);
        assert_eq!(226, count_paths(&g));
        assert_eq!(3509, count_paths_more_caves(&g));

    }
}
