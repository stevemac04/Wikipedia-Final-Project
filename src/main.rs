mod graph_ops;
use graph_ops::{Graph, page_rank};
use std::fs::File;
use std::io::prelude::*;
use rand::{Rng, thread_rng};
use std::collections::HashMap;
fn main() {
    let (vertex_count, edge_vec) = read_file("links.tsv");
    let data_graph = Graph::create_directed(vertex_count, &edge_vec);

}

fn read_file(path: &str) -> (usize, Vec<(usize, usize)>) {
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();

    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut vertex_map: HashMap<String, usize> = HashMap::new();
    let mut vertex_count = 0;

    for line in buf_reader {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.trim().split('\t').collect();
        if parts.len() != 2 {
            panic!("Each line must contain exactly two vertices separated by a tab");
        }
        let source = parts[0].to_string();
        let target = parts[1].to_string();

        // Map source vertex to a unique index if not already mapped
        let source_index = *vertex_map.entry(source).or_insert_with(|| {
            let index = vertex_count;
            vertex_count += 1;
            index
        });

        // Map target vertex to a unique index if not already mapped
        let target_index = *vertex_map.entry(target).or_insert_with(|| {
            let index = vertex_count;
            vertex_count += 1;
            index
        });

        edges.push((source_index, target_index));
    }

    (vertex_count, edges)
}

#[cfg(test)]
mod tests  {
    use super::*;
    #[test]
    fn test_read_graph() {
        let (test_vertices, test_edges) = read_file("test.tsv");
        let test_graph = Graph::create_directed(test_vertices, &test_edges);
    
        // Mapping team names to indices as they would be read and stored in the vertex_map
        let team_indices = [
            ("Penguins", 0),
            ("Flyers", 1),
            ("Rangers", 2),
            ("Blackhawks", 3),
            ("Red_Wings", 4),
            ("Bruins", 5),
            ("Canadiens", 6),
            ("Maple_Leafs", 7),
            ("Senators", 8),
            ("Oilers", 9),
            ("Flames", 10),
        ].iter().cloned().collect::<HashMap<&str, usize>>();
    
        // Setting up the expected outedges
        let mut expected_outedges = vec![vec![]; 11]; // 11 teams total
        expected_outedges[team_indices["Penguins"]] = vec![team_indices["Flyers"]];
        expected_outedges[team_indices["Flyers"]] = vec![team_indices["Rangers"], team_indices["Blackhawks"]];
        expected_outedges[team_indices["Rangers"]] = vec![team_indices["Penguins"]];
        expected_outedges[team_indices["Blackhawks"]] = vec![team_indices["Red_Wings"]];
        expected_outedges[team_indices["Red_Wings"]] = vec![];
        expected_outedges[team_indices["Bruins"]] = vec![team_indices["Flyers"]];
        expected_outedges[team_indices["Canadiens"]] = vec![team_indices["Bruins"]];
        expected_outedges[team_indices["Maple_Leafs"]] = vec![team_indices["Canadiens"]];
        expected_outedges[team_indices["Senators"]] = vec![team_indices["Maple_Leafs"], team_indices["Oilers"]];
        expected_outedges[team_indices["Oilers"]] = vec![team_indices["Flames"]];
        expected_outedges[team_indices["Flames"]] = vec![team_indices["Senators"]];
    
        let expected_graph = Graph {
            n: 11, // Total number of teams
            outedges: expected_outedges,
        };
    
        // Compare the actual graph with the expected graph using assert_eq!()
        assert_eq!(test_graph.n, expected_graph.n);
        assert_eq!(test_graph.outedges, expected_graph.outedges);
    }
}