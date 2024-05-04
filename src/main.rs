mod graph_ops;
use graph_ops::{Graph, page_rank};

use std::fs::File;
use std::io::prelude::*;
use rand::{Rng, thread_rng};
use std::collections::HashMap;


fn main() {
    let mut rng = thread_rng(); // for seed
    let r_seed: u64 = rng.gen(); // for seed
    let (vertex_count, edge_vec, label_vec, index_vec) = read_file("links.tsv");
    let data_graph = Graph {
        n: vertex_count,
        outedges: edge_vec,
        vertex_labels: label_vec,
        vertex_indices: index_vec,
    };
    
    // PAGE RANK
    let page_rank_return = page_rank(&data_graph, r_seed);
    println!("Top 5 most common ending vertices:");
    for (index, count) in page_rank_return.iter().take(5) { // take the top 5 counts
        let count_fraction = *count as f64 / (100.0 * vertex_count as f64); // change to a decimal (percentage) of the total walks
        println!("vertex {}: approximate pagerank = {:.4}", index, count_fraction); //print
    }

    // MIN DISTANCE
    let mut input = String::new();
    println!("Enter the first article name:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let article1 = input.trim().to_string();
    input.clear();

    println!("Enter the second article name:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let article2 = input.trim().to_string();

    if data_graph.vertex_indices.contains_key(&article1) && data_graph.vertex_indices.contains_key(&article2) {
        let distance = data_graph.min_distance(&article1, &article2);
        match distance {
            Some(d) => println!("The distance between '{}' and '{}' is {}", article1, article2, d),
            None => println!("No path exists between '{}' and '{}'", article1, article2),
        }
    } else {
        if !data_graph.vertex_indices.contains_key(&article1) {
            println!("Article '{}' does not exist in the graph.", article1);
        }
        if !data_graph.vertex_indices.contains_key(&article2) {
            println!("Article '{}' does not exist in the graph.", article2);
        }
    }
    // CONNECTED COMPONENTS
    let components = data_graph.connected_components();
    // GRAPH SPLIT
    let main_component = &components[0];  // Get the first component (the main component of our graph)
    let subgraph = data_graph.create_subgraph(main_component); // Create a subgraph from the first component
    // SEPARATION
    let degree_separation = subgraph.max_degree_of_separation();

    match degree_separation {
        Some(value) => println!("The maximum degree of separation between any two articles is '{}'.", value),
        None => println!("The graph is not entirely connected."),
    }
}

fn read_file(path: &str) -> (usize, Vec<Vec<usize>>, Vec<String>, HashMap<String, usize>) {
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();

    let mut edges: Vec<(String, String)> = Vec::new();
    let mut vertex_labels: Vec<String> = Vec::new();
    let mut vertex_indices: HashMap<String, usize> = HashMap::new();

    for line in buf_reader {
        // if line.starts_with("#") or empty
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.trim().split('\t').collect();
        if parts.len() != 2 {
            panic!("Each line must contain exactly two vertices separated by a tab");
        }
        let source = parts[0].to_string();
        let target = parts[1].to_string();
        // insert source vertex if it's not already included
        if !vertex_indices.contains_key(&source) {
            vertex_indices.insert(source.clone(), vertex_labels.len());
            vertex_labels.push(source.clone());
        }

        // insert target vertex if it's not already included
        if !vertex_indices.contains_key(&target) {
            vertex_indices.insert(target.clone(), vertex_labels.len());
            vertex_labels.push(target.clone());
        }

        edges.push((source, target));
    }
    let vertex_count = vertex_labels.len();

    let mut outedges: Vec<Vec<usize>> = vec![Vec::new(); vertex_count];

    for (source, target) in edges {
        let src_index = *vertex_indices.get(&source).unwrap();
        let tgt_index = *vertex_indices.get(&target).unwrap();
        outedges[src_index].push(tgt_index);
    }
    
    (vertex_count, outedges, vertex_labels, vertex_indices)
}

#[cfg(test)]
mod tests  {
    use super::*;
    use float_cmp::approx_eq;
    #[test]
    fn test_read_graph() {
        let (vertex_count, outedges, vertex_labels, vertex_indices) = read_file("test.tsv");

        let test_graph = Graph {
            n: vertex_count,
            outedges: outedges,
            vertex_labels: vertex_labels,
            vertex_indices: vertex_indices,
        };

        // mapping team names to indices as they would be read and stored in the vertex_map
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
    
        // setting up the expected outedges
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

        let expected_vertex_labels = vec![
        "Penguins", "Flyers", "Rangers", "Blackhawks", "Red_Wings", 
        "Bruins", "Canadiens", "Maple_Leafs", "Senators", "Oilers", "Flames"
        ].iter().map(|&name| name.to_string()).collect::<Vec<String>>();

        let expected_vertex_indices: HashMap<String, usize> = [
        ("Penguins", 0), ("Flyers", 1), ("Rangers", 2), ("Blackhawks", 3), 
        ("Red_Wings", 4), ("Bruins", 5), ("Canadiens", 6), ("Maple_Leafs", 7), 
        ("Senators", 8), ("Oilers", 9), ("Flames", 10)
        ].iter().cloned().map(|(k, v)| (k.to_string(), v)).collect();
    
        let expected_graph = Graph {
            n: 11, // hard code 11 total teams expected
            outedges: expected_outedges,
            vertex_labels: expected_vertex_labels,
            vertex_indices: expected_vertex_indices,
        };
    
        // compare the actual graph values with the expected graph values using assert_eq!()
        assert_eq!(test_graph.n, expected_graph.n);
        assert_eq!(test_graph.outedges, expected_graph.outedges);
        assert_eq!(test_graph.vertex_labels, expected_graph.vertex_labels);
        assert_eq!(test_graph.vertex_indices, expected_graph.vertex_indices);
    }
    #[test]
    fn test_page_rank() {
        let (vertex_count, outedges, vertex_labels, vertex_indices) = read_file("test.tsv");

        let test_graph = Graph {
            n: vertex_count,
            outedges: outedges,
            vertex_labels: vertex_labels,
            vertex_indices: vertex_indices,
        };

        let actual_results = page_rank(&test_graph, 42);
        let expected_results = vec![
            ("Flyers", 0.2155),
            ("Blackhawks", 0.1136),
            ("Penguins", 0.1055),
            ("Rangers", 0.1027),
            ("Red_Wings", 0.0864),
            ("Senators", 0.0800),
            ("Canadiens", 0.0691),
            ("Flames", 0.0655),
            ("Bruins", 0.0618),
            ("Oilers", 0.0555),
            ("Maple_Leafs", 0.0445)
        ];

        // compare actual with expected
        for (i, (actual_label, actual_count)) in actual_results.iter().enumerate() {
            let (expected_label, expected_percent) = &expected_results[i];
            let count_percent = *actual_count as f64 / (100.0 * vertex_count as f64);

            let scale = 10000.0; // 10^4 for 4 decimal places (for rounding)
            let rounded_actual_percent = (count_percent * scale).round() / scale;

            assert_eq!(actual_label, expected_label);
            assert_eq!(rounded_actual_percent, *expected_percent);
        }
    }
    #[test]
    fn test_bfs() {
        let (vertex_count, outedges, vertex_labels, vertex_indices) = read_file("test.tsv");

        let test_graph = Graph {
            n: vertex_count,
            outedges: outedges,
            vertex_labels: vertex_labels,
            vertex_indices: vertex_indices,
        };
        let start_label = "Flyers";
        let start_index = *test_graph.vertex_indices.get(start_label).expect("Doesn't exist");

        let actual_result = test_graph.bfs(start_index);

        let expected_result = vec![
            Some(2), // Distance to vertex 0
            Some(0), // Distance to vertex 1 (itself)
            Some(1), // Distance to vertex 2
            Some(1), // Distance to vertex 3
            Some(2), // Distance to vertex 4
            None, // Unreachable
            None, // Unreachable
            None, // Unreachable
            None, // Unreachable
            None, // Unreachable
            None, // Unreachable
        ];
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn test_min_distance() {
        let (vertex_count, outedges, vertex_labels, vertex_indices) = read_file("test.tsv");

        let test_graph = Graph {
            n: vertex_count,
            outedges: outedges,
            vertex_labels: vertex_labels,
            vertex_indices: vertex_indices,
        };

        let start_label = "Oilers";
        let end_label = "Bruins";

        let distance_option = test_graph.min_distance(start_label, end_label);
        let actual_distance = distance_option.unwrap();

        let expected_distance: u32 = 5;
        assert_eq!(actual_distance, expected_distance);
    }
    #[test]
    fn test_max_separation() {
        let (vertex_count, outedges, vertex_labels, vertex_indices) = read_file("test.tsv");

        let test_graph = Graph {
            n: vertex_count,
            outedges: outedges,
            vertex_labels: vertex_labels,
            vertex_indices: vertex_indices,
        };

        let degree_separation = test_graph.max_degree_of_separation();
        assert_eq!(degree_separation, None);
    }
    #[test]
    fn test_separation_alternate() {
        let test_graph = Graph {
            n: 3,
            outedges: vec![vec![1], vec![2], vec![0]], // A simple triangle graph
            vertex_labels: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vertex_indices: HashMap::from([("A".to_string(), 0), ("B".to_string(), 1), ("C".to_string(), 2)]),
        };
    
        let max_sep = test_graph.max_degree_of_separation();
        println!("Max degree of separation: {:?}", max_sep);
    }
}