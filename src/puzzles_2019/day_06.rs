use crate::puzzle_input;
use petgraph::algo::astar;
use petgraph::algo::dijkstra;
use petgraph::graphmap::UnGraphMap;

pub fn run() {
    let input = puzzle_input::read_string("./input/2019-d06-input1.txt");
    let g = parse_input(&input);
    println!("** Part 1 Final: {:?}", chksum_orbits(&g));
    println!("** Part 2 Final: {:?}", count_transfers(&g));
}

fn parse_input(input_ref: &str) -> UnGraphMap<&str, i32> {
    let mut graph = UnGraphMap::new();
    graph.add_node("COM");

    for entry in input_ref.split_whitespace() {
        let v: Vec<&str> = entry.split(')').collect();
        graph.add_edge(v[0], v[1], 1);
    }
    graph
}

fn chksum_orbits(graph: &UnGraphMap<&str, i32>) -> i32 {
    let res = dijkstra(&graph, "COM", None, |_| 1);
    let mut i = 0;
    res.iter().for_each(|(_, v)| i += v);
    i
}

fn count_transfers(graph: &UnGraphMap<&str, i32>) -> i32 {
    let path = astar(&graph, "YOU", |finish| finish == "SAN", |_| 1, |_| 0);
    println!("{:?}", path);
    path.unwrap().0 - 2
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

    #[test]
    fn test_orbit_transfers() {
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
        K)L
        K)YOU
        I)SAN";

        let g = parse_input(input);
        println!("{:?}", g);

        assert_eq!(count_transfers(&g), 4);
    }
}
