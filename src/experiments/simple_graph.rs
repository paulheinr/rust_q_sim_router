#[derive(Debug)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl Graph {
    fn is_valid(&self) -> bool {
        self.has_valid_vertices() && self.valid_edges()
    }

    fn has_valid_vertices(&self) -> bool {
        only_unique_values(&self.vertices)
    }

    fn valid_edges(&self) -> bool {
        let mut res = true;
        for edge in self.edges.iter() {
            res = res && self.valid_edge(edge);
        }
        res && only_unique_values(&self.edges)
    }

    fn valid_edge(&self, edge: &Edge) -> bool {
        self.vertices.iter().any(|v| v.id == edge.from)
    }

    fn traverse_path_sum_weights(&self, path: Vec<i32>) -> f32 {
        0.0
    }
}

trait Id {
    fn get_id(&self) -> i32;
}

fn only_unique_values<T: Id>(list: &Vec<T>) -> bool {
    let mut x: Vec<i32> = list.iter().map(|v| v.get_id()).collect();
    x.dedup();
    x.len() == list.len()
}

#[derive(Debug, PartialEq)]
pub struct Vertex {
    pub id: i32,
}

impl Id for Vertex {
    fn get_id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub weight: f32,
}

impl Id for Edge {
    fn get_id(&self) -> i32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use crate::experiments::simple_graph::{Edge, Graph, Vertex};

    #[test]
    fn print_graph() {
        let vertex_list = vec![Vertex { id: 1 }, Vertex { id: 2 }, Vertex { id: 3 }];
        let edge_list = vec![Edge { id: 1, from: 1, to: 2, weight: 0.5 },
                             Edge { id: 2, from: 1, to: 3, weight: 2.0 },
                             Edge { id: 3, from: 2, to: 2, weight: 1.0 },
                             Edge { id: 4, from: 2, to: 3, weight: 4.0 },
                             Edge { id: 5, from: 3, to: 1, weight: 2.0 }];
        let graph = Graph {
            vertices: vertex_list,
            edges: edge_list,
        };
        println!("{:?}", graph);
        println!("First edge is {:?}", graph.edges.get(0));
        println!("Graph is valid: {:?}", graph.is_valid());
        println!("Graph has valid vertices: {:?}", graph.has_valid_vertices());
    }

    #[test]
    fn test_invalid_graph() {
        let graph = Graph {
            vertices: vec![Vertex { id: 1 }, Vertex { id: 1 }],
            edges: vec![Edge { id: 1, from: 3, to: 1, weight: 2.0 }, Edge { id: 2, from: 1, to: 1, weight: 2.0 }],
        };
        assert_eq!(graph.has_valid_vertices(), false);
        assert_eq!(graph.has_valid_vertices(), false);
    }
}