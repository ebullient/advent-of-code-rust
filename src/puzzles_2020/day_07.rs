use crate::puzzle_input;

use petgraph::algo::has_path_connecting;
use petgraph::graphmap::DiGraphMap;
use std::str::Split;

pub fn run() {
    let input = puzzle_input::read_string("./input/2020-d07-input1.txt");
    let graph = parse_input(&input);
    println!("** Part 1 Final: {:?}", count_paths(&graph, "shiny gold"));
    println!("** Part 2 Final: {:?}", count_bags(&graph, "shiny gold"));
}

fn parse_input(input_ref: &str) -> DiGraphMap<&str, i32> {
    let mut graph = DiGraphMap::new();

    for entry in input_ref.split('\n') {
        let mut s: Split<&str> = entry.trim().split(" bags contain ");
        let node_x = graph.add_node(s.next().unwrap());
        graph.add_node(node_x);

        // Regex and ownership are stupid. Munge the crap out of this string
        if let Some(predicate) = s.next() {
            for chunk in predicate.split(", ") {
                if let Some(i) = chunk.find(' ') {
                    let (n, remainder) = chunk.split_at(i);
                    if let Ok(weight) = n.parse::<i32>() {
                        if let Some(j) = remainder.find(" bag") {
                            let (y, _) = remainder.split_at(j);
                            let node_y = graph.add_node(y.trim());
                            graph.add_node(node_y);
                            graph.add_edge(node_x, node_y, weight);
                        }
                    }
                }
            }
        } else {
            println!("No predicate: {:?}", entry);
        }
    }
    graph
}

fn count_paths(graph: &DiGraphMap<&str, i32>, target: &str) -> i32 {
    graph
        .nodes()
        .filter(|x| x != &target)
        .filter(|x| has_path_connecting(&graph, x, target, None))
        .count() as i32
}

fn count_bags(graph: &DiGraphMap<&str, i32>, start: &str) -> i32 {
    graph
        .edges(start)
        .map(|x| *x.2 + *x.2 * count_bags(graph, x.1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_bags() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";
        let graph = parse_input(input);

        assert_eq!(count_paths(&graph, "shiny gold"), 4);
        assert_eq!(count_bags(&graph, "shiny gold"), 32);
    }
}
