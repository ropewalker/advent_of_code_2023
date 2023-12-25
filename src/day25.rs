use aoc_runner_derive::{aoc, aoc_generator};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

#[aoc_generator(day25)]
fn parse_input(wiring_diagram: &str) -> Vec<(String, Vec<String>)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(string(alpha+) ": " repeat_sep(string(alpha+), " ")));

    parser.parse(wiring_diagram).unwrap()
}

#[derive(Default, Clone)]
struct Graph {
    vertices: Vec<(String, usize)>,
    edges: Vec<(String, String, usize)>,
}

fn contract(graph: &Graph, number_of_vertices: usize, rng: &mut ThreadRng) -> Graph {
    let mut graph = graph.clone();

    while graph.vertices.len() > number_of_vertices {
        let mut new_graph: Graph = Default::default();

        let (u, v, _) = graph.edges.choose(rng).unwrap();

        let mut uv = u.clone();
        uv.push_str(v);

        let mut uv_vertex_cardinality = 0;

        for (w, w_vertex_cardinality) in graph.vertices.iter() {
            if w == u || w == v {
                uv_vertex_cardinality += w_vertex_cardinality;
            } else {
                new_graph
                    .vertices
                    .push((w.to_owned(), *w_vertex_cardinality));

                let mut wuv_edge_cardinality = 0;

                for (_, _, ab_edge_cardinality) in graph
                    .edges
                    .iter()
                    .filter(|(a, b, _)| (a == w && b == u) || (a == u && b == w))
                {
                    wuv_edge_cardinality += ab_edge_cardinality;
                }

                for (_, _, ab_edge_cardinality) in graph
                    .edges
                    .iter()
                    .filter(|(a, b, _)| (a == w && b == v) || (a == v && b == w))
                {
                    wuv_edge_cardinality += ab_edge_cardinality;
                }

                if wuv_edge_cardinality > 0 {
                    new_graph
                        .edges
                        .push((w.to_owned(), uv.to_owned(), wuv_edge_cardinality));
                }
            }
        }

        new_graph
            .vertices
            .push((uv.to_owned(), uv_vertex_cardinality));

        for edge in graph.edges.iter() {
            if &edge.0 != u && &edge.0 != v && &edge.1 != u && &edge.1 != v {
                new_graph.edges.push(edge.to_owned());
            }
        }

        graph = new_graph;
    }

    graph
}

fn fast_min_cut(graph: &Graph, rng: &mut ThreadRng) -> Graph {
    if graph.vertices.len() <= 6 {
        contract(graph, 2, rng)
    } else {
        let t = (1.0 + graph.vertices.len() as f64 / 2.0f64.sqrt()).ceil() as usize;

        let graph1 = contract(graph, t, rng);
        let graph2 = contract(graph, t, rng);

        [fast_min_cut(&graph1, rng), fast_min_cut(&graph2, rng)]
            .into_iter()
            .min_by(|g1, g2| {
                g1.edges
                    .first()
                    .unwrap()
                    .2
                    .cmp(&g2.edges.first().unwrap().2)
            })
            .unwrap()
    }
}

#[aoc(day25, part1)]
fn part1(wiring_diagram: &[(String, Vec<String>)]) -> usize {
    let mut rng = thread_rng();

    let mut graph: Graph = Default::default();

    let mut unique_vertices = HashSet::new();

    for (source, destinations) in wiring_diagram.iter() {
        unique_vertices.insert(source.to_owned());

        for destination in destinations {
            unique_vertices.insert(destination.to_owned());
            graph
                .edges
                .push((source.to_owned(), destination.to_owned(), 1));
        }
    }

    graph.vertices = unique_vertices
        .into_iter()
        .map(|vertex| (vertex, 1))
        .collect::<Vec<_>>();

    loop {
        let contracted_graph = fast_min_cut(&graph, &mut rng);

        if contracted_graph.edges.first().unwrap().2 == 3 {
            return contracted_graph
                .vertices
                .iter()
                .map(|(_, cardinality)| cardinality)
                .product::<usize>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 54);
    }
}
