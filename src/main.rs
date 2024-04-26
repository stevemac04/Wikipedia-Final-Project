mod graph_ops;
use graph_ops::{Graph, page_rank};
use std::fs::File;
use std::io::prelude::*;
use rand::{Rng, thread_rng};
fn main() {
    println!("Hello, world!");
}

fn read_file(path: &str) -> io::Result<(usize, Vec<(usize, usize)>)> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file).lines();

    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut vertices = std::collections::HashSet::new();

    for line in buf_reader {
        let line = line?;
        let parts: Vec<&str> = line.trim().split('\t').collect();
        if parts.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Each line must contain exactly two vertices separated by a tab"));
        }
        let x = parts[0].parse::<usize>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid vertex format"))?;
        let y = parts[1].parse::<usize>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid vertex format"))?;
        edges.push((x, y));
        vertices.insert(x);
        vertices.insert(y);
    }

    Ok((vertices.len(), edges))
}