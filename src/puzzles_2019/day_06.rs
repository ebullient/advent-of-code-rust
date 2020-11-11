use petgraph::Direction;
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::depth_first_search;
use petgraph::visit::DfsEvent;
use petgraph::algo::dijkstra;

pub fn run() {
    if let Ok(input) = super::read_string("./input/2019-d06-input1.txt") {
        let g = parse_input(&input);
        println!("** Part 1 Final: {:?}", chksum_orbits(&g));
    }
}

fn parse_input(input_ref: &str) -> DiGraphMap<&str, i32> {
    let mut graph = DiGraphMap::new();
    let origin = graph.add_node("COM");

    for entry in input_ref.split_whitespace() {
        let v: Vec<&str> = entry.split(')').collect();
        graph.add_edge(v[0], v[1], 1);
    }
    graph
}

fn chksum_orbits(graph: &DiGraphMap<&str, i32>) -> i32 {
    let res = dijkstra(&graph, "COM", None, |_| 1);
    let mut i = 0;
    res.iter().for_each(|(k, v)| i += v);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_orbits() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";

        let g = parse_input(input);
        println!("{:?}", g);

        assert_eq!(chksum_orbits(&g), 42);
    }
}