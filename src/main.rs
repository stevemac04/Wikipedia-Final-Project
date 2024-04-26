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