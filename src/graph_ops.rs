use rand::{rngs::StdRng, Rng, SeedableRng};
pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex,Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Graph {
    pub n: usize, // number of vertices
    pub outedges: Vec<Vec<usize>>, // adjacency list representation
    pub vertex_labels: Vec<String>, // maps index to vertex label
    pub vertex_indices: HashMap<String, usize>, // maps vertex label to index
}

#[allow(dead_code)]
impl Graph {
    pub fn new(vertices: Vec<String>, edges: Vec<(String, String)>) -> Self {
        let n = vertices.len();
        let mut vertex_indices = HashMap::new();
        let mut outedges = vec![vec![]; n];
        let mut vertex_labels = vec![String::new(); n];

        for (i, vertex) in vertices.into_iter().enumerate() {
            vertex_indices.insert(vertex.clone(), i);
            vertex_labels[i] = vertex;
        }

        for (src, tgt) in edges {
            let src_idx = *vertex_indices.get(&src).unwrap();
            let tgt_idx = *vertex_indices.get(&tgt).unwrap();
            outedges[src_idx].push(tgt_idx);
        }

        Graph {
            n,
            outedges,
            vertex_labels,
            vertex_indices,
        }
    }

    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            if *u < self.n && *v < self.n {
                self.outedges[*u].push(*v);
            }
        }
    }

    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort_unstable();
        }
    }

    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
            vertex_labels: vec![String::new(); n],
            vertex_indices: HashMap::new(),
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
    pub fn create_subgraph(&self, component: &[String]) -> Graph { // This method is to create a subgraph of the main component of the original graph
        let mut subgraph_indices = HashMap::new();
        let mut subgraph_outedges = Vec::new();
        let mut subgraph_labels = Vec::new();

        // Assign new indices to the vertices in the component
        for (new_index, label) in component.iter().enumerate() {
            subgraph_indices.insert(label.clone(), new_index);
            subgraph_labels.push(label.clone());
        }

        // Build the adjacency list for the subgraph
        for label in component {
            let original_index = self.vertex_indices[label];
            let _new_index = subgraph_indices[label];
            let mut edges = Vec::new();

            for &neighbor_index in &self.outedges[original_index] {
                let neighbor_label = &self.vertex_labels[neighbor_index];
                if subgraph_indices.contains_key(neighbor_label) {
                    edges.push(subgraph_indices[neighbor_label]);
                }
            }
            subgraph_outedges.push(edges);
        }

        Graph {
            n: component.len(),
            outedges: subgraph_outedges,
            vertex_labels: subgraph_labels,
            vertex_indices: subgraph_indices,
        }
    }

    pub fn bfs(&self, start: Vertex) -> Vec<Option<u32>> { // help from lecture 28
        let mut distance: Vec<Option<u32>> = vec![None; self.n];
        distance[start] = Some(0);
        let mut queue: VecDeque<Vertex> = VecDeque::new();
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            for &u in &self.outedges[v] {
                if distance[u].is_none() {
                    distance[u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(u);
                }
            }
        }

        distance
    }

    pub fn min_distance(&self, start_label: &str, end_label: &str) -> Option<u32> {
        let start_index = self.vertex_indices.get(start_label)?;
        let end_index = self.vertex_indices.get(end_label)?;

        let distances = self.bfs(*start_index);
        distances[*end_index]
    }

    pub fn max_degree_of_separation(&self) -> Option<u32> {
        let mut max_separation = 0;
    
        // Perform BFS from each vertex to find the maximum distance to any other vertex
        for start in 0..self.n {
            let distances = self.bfs(start);
            if distances.iter().any(|d| d.is_none()) {
                println!("Vertex {} cannot reach all other vertices.", start);
                return None; // If any vertex is unreachable, return None
            }
            for distance in distances.iter().flatten() {
                if *distance > max_separation {
                    max_separation = *distance;
                }
            }
        }
    
        Some(max_separation)
    }
    
    pub fn connected_components(&self) -> Vec<Vec<String>> {
        let mut visited = vec![false; self.n];
        let mut components = Vec::new();

        for start in 0..self.n {
            if !visited[start] {
                let mut component = Vec::new();
                let mut stack = vec![start];

                while let Some(vertex) = stack.pop() {
                    if !visited[vertex] {
                        visited[vertex] = true;
                        component.push(self.vertex_labels[vertex].clone());  // Store the vertex label instead of the index
                        for &neighbor in &self.outedges[vertex] {
                            if !visited[neighbor] {
                                stack.push(neighbor);
                            }
                        }
                    }
                }

                components.push(component);
            }
        }

        components
    }
}
    
pub fn page_rank(graph: &Graph, seed: u64) -> Vec<(String, usize)> {
    let mut rng = StdRng::seed_from_u64(seed); // set seed
    let num_vertices = graph.n;
    let mut end_counts: Vec<(String, usize)> = graph.vertex_labels.iter().map(|label| (label.clone(), 0)).collect(); // initialize with labels and zero counts

    for i in 0..num_vertices { // iterate through each vertex by index
        let mut current_vertex = i; // current_vertex to keep track through walks
        for _ in 0..100 { // each vertex must be iterated through 100 walks
            for _ in 0..100 { // 100 steps per walk
                let x: i32 = rng.gen_range(1..=10) as i32; // Use seeded RNG for 1-10
                let current_len: usize = graph.outedges[current_vertex].len(); // the amount of outedges that the current vertex has
                if current_len == 0 || x == 1 { // options where we must jump to a new vertex
                    current_vertex = rng.gen_range(0..num_vertices); // jump to random vertex
                } else {
                    current_vertex = graph.outedges[current_vertex][rng.gen_range(0..current_len)]; // step using edge to connected vertex
                }
            }
            end_counts[current_vertex].1 += 1; // wherever we end, add one to the count for that vertex
        }
    }
    end_counts.sort_by_key(|&(_, count)| std::cmp::Reverse(count)); // sort the count vector by which vertices have the highest counts
    end_counts
}