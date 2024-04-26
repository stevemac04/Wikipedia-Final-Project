use rand::{rngs::StdRng, Rng, SeedableRng};

pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex,Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
// Graph and implied functions are taken from Lecture
pub struct Graph {
    pub n: usize, // vertex labels in {0,...,n-1}
    pub outedges: AdjacencyLists,
}
impl Graph { 
    pub fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    pub fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
}


pub fn page_rank(graph: Graph, num_vertices: usize, seed: u64) -> Vec<(usize, usize)>{
    let mut rng = StdRng::seed_from_u64(seed); // set seed
    let mut end_counts: Vec<(usize, usize)> = Vec::new(); //empty vector with end_counts and vertex labels
    for i in 0..num_vertices {
        end_counts.push((i, 0)); // put vertex labels into vector
    }
    for vertex in 0..num_vertices { // iterate through each vertex
        let mut current_vertex = vertex; // current_vertex to keep track through walks
        for _ in 0..100 { // each vertex must be iterated through 100 walks
            for _ in 0..100 { // 100 steps per walk
                let x: i32 = rng.gen_range(1..=10) as i32; // Use seeded RNG for 1-10
                let current_len: usize = graph.outedges[current_vertex].len() as usize; // the amount of outedges that the current vertex has
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

