use crate::puzzle_input;
use petgraph::algo::astar;
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::EdgeRef;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2021-d15-input.txt");
    let (g, end, big_end) = parse(&input);

    let path = astar(&g, (0, 0), |finish| finish == end, |e| *e.weight(), |_| 0).unwrap();
    println!("** Part 1 Final: {:?}", path.0);

    let big_path = astar(
        &g,
        (0, 0),
        |finish| finish == big_end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();
    println!("** Part 2 Final: {:?}", big_path.0);
}

fn parse(
    input: &Vec<String>,
) -> (
    DiGraphMap<(usize, usize), i32>,
    (usize, usize),
    (usize, usize),
) {
    let mut graph = DiGraphMap::new();
    let height = input.len();
    let width = input[0].len();
    let max_height = height * 5;
    let max_width = width * 5;

    for (y, row) in input.iter().enumerate() {
        for (x, col) in row.trim().chars().enumerate() {
            let mut r = vec![0; 10];
            r[0] = col.to_digit(10).unwrap() as i32;
            for n in 1..10 {
                r[n] = r[n - 1] + 1;
                if r[n] > 9 {
                    r[n] = 1;
                }
            }
            add_edges(&mut graph, max_height, max_width, y, x, r[0]);

            for i in 0..5 as usize {
                for j in 0..5 as usize {
                    let r = r[i + j] as i32;
                    let y1 = y + i * height;
                    let x1 = x + j * width;
                    add_edges(&mut graph, max_height, max_width, y1, x1, r);
                }
            }
        }
    }
    (
        graph,
        (height - 1, width - 1),
        (max_height - 1, max_width - 1),
    )
}

fn add_edges(
    graph: &mut DiGraphMap<(usize, usize), i32>,
    max_height: usize,
    max_width: usize,
    y: usize,
    x: usize,
    risk: i32,
) {
    let here = (y, x);
    if y > 0 {
        graph.add_edge((y - 1, x), here, risk);
    }
    if x > 0 {
        graph.add_edge((y, x - 1), here, risk);
    }
    if y < max_height - 1 {
        graph.add_edge((y + 1, x), here, risk);
    }
    if x < max_width - 1 {
        graph.add_edge((y, x + 1), here, risk);
    }
}

#[allow(dead_code)]
fn dump(graph: &DiGraphMap<(usize, usize), i32>, max_height: usize, max_width: usize) {
    for y in 0..max_height {
        for x in 0..max_width {
            let other = if y == 0 { (y + 1, x) } else { (y - 1, x) };
            if let Some(w) = graph.edge_weight(other, (y, x)) {
                print!("{:?}", *w);
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581",
        );

        let (g, end, big_end) = parse(&input);

        let path = astar(&g, (0, 0), |finish| finish == end, |e| *e.weight(), |_| 0).unwrap();
        println!("{:?}", path);
        assert_eq!(40, path.0);

        let big_path = astar(
            &g,
            (0, 0),
            |finish| finish == big_end,
            |e| *e.weight(),
            |_| 0,
        )
        .unwrap();
        assert_eq!(315, big_path.0);
    }
}
